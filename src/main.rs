#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::SocketAddr;
use axum::{routing::post, Router};
use tower_http::services::ServeDir;

mod config;
mod handler;

#[tokio::main]
async fn main() {
    let config = config::Config::new().expect("Failed to load config");

    let app = Router::new()
        .route("/api/shutdown", post(handler::shutdown::shutdown))
        .route("/api/screenshot", post(handler::screenshot::screenshot))
        .nest_service("/", ServeDir::new(config.server.static_dir.clone()));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");
}
