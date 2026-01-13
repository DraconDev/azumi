use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::OnceLock;
use tokio::sync::broadcast;
use crate::FallbackRender;

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
            // Handle incoming websocket messages (to detect closure)
            res = socket.recv() => {
                match res {
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
            write!(f, "{}", part)?;
            if i < dynamics.len() {
                dynamics[i].render_azumi(f)?;
            }
        }
        Ok(())
    }
}

static TEMPLATE_REGISTRY: OnceLock<std::sync::RwLock<std::collections::HashMap<String, RuntimeTemplate>>> = OnceLock::new();

pub fn get_template(id: &str) -> Option<RuntimeTemplate> {
    TEMPLATE_REGISTRY.get_or_init(Default::default).read().unwrap().get(id).cloned()
}

#[derive(serde::Deserialize)]
struct TemplateUpdatePayload {
    id: String,
    parts: Vec<String>,
}

async fn update_template_handler(Json(payload): Json<TemplateUpdatePayload>) {
    let mut registry = TEMPLATE_REGISTRY.get_or_init(Default::default).write().unwrap();
    registry.insert(payload.id.clone(), RuntimeTemplate { static_parts: payload.parts });
    println!("🔥 Hot Reload: Updated template {}", payload.id);
    // Trigger browser reload
    let _ = get_broadcast_channel().send(serde_json::json!({"type": "reload"}).to_string());
}
