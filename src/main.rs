#![windows_subsystem = "windows"]

use std::net::SocketAddr;
use axum::{routing::post, Router};
use tower_http::services::ServeDir;

mod config;
mod handler;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let app = Router::new()
        .route("/api/shutdown", post(handler::shutdown::shutdown))
        .route("/api/screenshot", post(handler::screenshot::screenshot))
        .nest_service("/", ServeDir::new(config.appdata_path));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");
}
