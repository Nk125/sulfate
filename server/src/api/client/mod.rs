mod state;

use axum::routing::any;
use axum::{extract::State, extract::ws::WebSocketUpgrade};
use std::sync::Arc;

pub fn routes<S>() -> axum::Router<S> {
    let arc = Arc::new(state::ClientPool {});

    axum::Router::new().route("/ws", any(websocket)).with_state(arc)
}

async fn websocket(State(_pool): State<Arc<state::ClientPool>>, ws: WebSocketUpgrade) -> axum::response::Response {
    tracing::debug!("New websocket connection");

    ws.on_upgrade(|mut _ws| async move {})
}
