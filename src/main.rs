mod config;
mod handlers;

use warp::Filter;
use tokio::sync::broadcast;
use dotenv::dotenv;
use crate::config::{ConfigSingleton, ServerConfig};
use std::net::SocketAddr;
use crate::handlers::handle_connection;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = ConfigSingleton::get_instance();
    let config = config.read().await;

    let address = config
        .get::<String>(ServerConfig::Address.as_str())
        .unwrap_or("127.0.0.1".to_string());

    let port: u16 = config
        .get::<u16>(ServerConfig::Port.as_str())
        .unwrap_or(3030);

    let (tx, _rx) = broadcast::channel::<String>(10);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || tx.clone()))
        .map(|ws: warp::ws::Ws, tx| {
            ws.on_upgrade(move |socket| handle_connection(socket, tx))
        });

    let address = format!("{}:{}", address, port);
    warp::serve(ws_route).run(address.parse::<SocketAddr>().unwrap()).await;
}
