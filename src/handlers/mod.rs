use warp::ws::{Message, WebSocket};
use tokio::sync::broadcast;
use futures::{StreamExt, SinkExt};

pub async fn handle_connection(ws: WebSocket, tx: broadcast::Sender<String>) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut rx = tx.subscribe();

    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if ws_tx.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });

    let receive_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_rx.next().await {
            if let Ok(text) = msg.to_str() {
                tx.send(text.to_string()).ok();
            }
        }
    });

    tokio::select! {
        _ = send_task => {}
        _ = receive_task => {}
    }
}
