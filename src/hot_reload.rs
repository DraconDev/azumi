use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};

/// Mounts the hot reload route at `/_azumi/live_reload`
pub fn router() -> Router {
    Router::new().route("/_azumi/live_reload", get(ws_handler))
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // Just keep the connection open.
    // When the server restarts, this connection will drop.
    // The client detects the drop and reloads when it can reconnect.
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if let Message::Close(_) = msg {
                return;
            }
        } else {
            return;
        }
    }
}
