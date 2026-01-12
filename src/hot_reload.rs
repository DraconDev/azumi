use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::OnceLock;
use tokio::sync::broadcast;

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
pub fn router() -> Router {
    Router::new().route("/_azumi/live_reload", get(ws_handler))
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
