#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::net::SocketAddr;
use axum::{routing::post, Router};
use log::{error, info};
use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use tower_http::services::{ServeDir, ServeFile};

mod config;
mod handler;

fn init_logger() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::max(), simplelog::Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::max(), simplelog::Config::default(), File::create("log.txt").unwrap()),
        ]
    ).unwrap();
}

#[tokio::main]
async fn main() {
    init_logger();

    info!("Loading config...");
    let config = match config::Config::new() {
        Ok(config) => {
            info!("Config successfully loaded.");
            config
        },
        Err(err) => {
            error!("Failed to load config: {}", err);
            return;
        }
    };

    let app = Router::new()
        .route("/api/shutdown", post(handler::shutdown::shutdown))
        .route("/api/screenshot", post(handler::screenshot::screenshot))
        .nest_service("/assets", ServeDir::new("static"))
        .fallback_service(ServeFile::new(format!("{}/index.html", &config.server.static_dir)));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("Server running on {}", addr); 
    
    match axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await {
        Ok(_) => return,
        Err(err) => error!("Server error: {}", err),
    }
}
