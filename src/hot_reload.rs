use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::OnceLock;
use tokio::sync::broadcast;
use crate::{Escaped, FallbackRender};

static BROADCAST_CHANNEL: OnceLock<broadcast::Sender<String>> = OnceLock::new();

fn get_broadcast_channel() -> &'static broadcast::Sender<String> {
    BROADCAST_CHANNEL.get_or_init(|| {
        let (tx, _) = broadcast::channel(100);
        tx
    })
}

/// Pushes a style update to all connected clients
pub fn push_style_update(scope_id: &str, css: &str) {
    let msg = serde_json::json!({
        "type": "style-update",
        "scopeId": scope_id,
        "css": css
    });
    let _ = get_broadcast_channel().send(msg.to_string());
}

/// Mounts the hot reload route at `/_azumi/live_reload`
///
/// # Security Warning
///
/// These endpoints are **development-only** and should NOT be exposed in production:
///
/// - `/_azumi/live_reload` - WebSocket endpoint for hot reload (no authentication)
/// - `/_azumi/update_template` - POST endpoint to update templates (no authentication)
///
/// In production, either:
/// 1. Remove this router entirely (hot reload is for development only)
/// 2. Restrict access at the network level (e.g., firewall rules to block external access)
/// 3. Add your own authentication middleware
///
/// If deploying to production with this enabled, ensure only localhost can access these routes.
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/_azumi/live_reload", get(ws_handler))
        .route("/_azumi/update_template", post(update_template_handler))
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut rx = get_broadcast_channel().subscribe();

    loop {
        tokio::select! {
            // Forward broadcasted messages to the websocket
            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    if socket.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            }
            // Handle incoming websocket messages (to detect closure and keep alive)
            res = socket.recv() => {
                match res {
                    Some(Ok(Message::Ping(data))) => {
                        // Respond to ping to keep connection alive through proxies
                        if socket.send(Message::Pong(data)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) => break,
                    Some(Err(_)) => break,
                    None => break,
                    _ => {}
                }
            }
        }
    }
}

// Runtime Template Support
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct RuntimeTemplate {
    pub static_parts: Vec<String>,
}

impl RuntimeTemplate {
    pub fn render(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        dynamics: &[&dyn FallbackRender],
    ) -> std::fmt::Result {
        for (i, part) in self.static_parts.iter().enumerate() {
            write!(f, "{}", Escaped(part))?;
            if i < dynamics.len() {
                dynamics[i].render_azumi(f)?;
            }
        }
        Ok(())
    }
}

static TEMPLATE_REGISTRY: OnceLock<std::sync::RwLock<std::collections::BTreeMap<String, RuntimeTemplate>>> = OnceLock::new();

const MAX_REGISTRY_SIZE: usize = 1000;

pub fn get_template(id: &str) -> Option<RuntimeTemplate> {
    let Ok(registry) = TEMPLATE_REGISTRY.get_or_init(Default::default).read() else {
        // RwLock poisoning indicates a prior panic in a writer - this is a serious issue
        // Return None rather than panicking again, but this may indicate underlying problems
        eprintln!("Hot Reload: Registry lock poisoned - template lookup failed");
        return None;
    };
    registry.get(id).cloned()
}

const MAX_TEMPLATE_PARTS: usize = 100;
const MAX_PART_SIZE: usize = 100_000; // 100KB per part
const MAX_TEMPLATE_ID_LEN: usize = 256;

#[derive(serde::Deserialize)]
struct TemplateUpdatePayload {
    id: String,
    parts: Vec<String>,
}

async fn update_template_handler(Json(payload): Json<TemplateUpdatePayload>) {
    // Validate input
    if payload.id.len() > MAX_TEMPLATE_ID_LEN {
        eprintln!("Hot Reload: Template ID too long");
        return;
    }
    if payload.parts.len() > MAX_TEMPLATE_PARTS {
        eprintln!("Hot Reload: Too many parts (max {})", MAX_TEMPLATE_PARTS);
        return;
    }
    for part in &payload.parts {
        if part.len() > MAX_PART_SIZE {
            eprintln!("Hot Reload: Part too large (max {} bytes)", MAX_PART_SIZE);
            return;
        }
    }

    let Ok(mut registry) = TEMPLATE_REGISTRY.get_or_init(Default::default).write() else {
        eprintln!("Hot Reload: Registry lock poisoned - template update failed");
        return;
    };

    // Evict entries when registry is full
    // Note: This evicts alphabetically (BTreeMap key order), NOT by insertion time.
    // This is NOT true LRU - it's just preventing unbounded growth.
    if registry.len() >= MAX_REGISTRY_SIZE {
        // Remove oldest 10% of entries
        let evict_count = (MAX_REGISTRY_SIZE / 10).max(1);
        let keys_to_remove: Vec<_> = registry.keys().take(evict_count).cloned().collect();
        for key in keys_to_remove {
            registry.remove(&key);
        }
    }

    registry.insert(payload.id.clone(), RuntimeTemplate { static_parts: payload.parts });
    #[cfg(debug_assertions)]
    println!("🔥 Hot Reload: Updated template {}", payload.id);
    let _ = get_broadcast_channel().send(serde_json::json!({"type": "reload"}).to_string());
}
