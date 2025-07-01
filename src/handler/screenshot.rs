use std::net::SocketAddr;
use axum::{response::IntoResponse, Json};
use axum::extract::ConnectInfo;
use base64::{engine::general_purpose, Engine as _};
use log::info;
use rayon::prelude::*;
use screenshots::Screen;

pub async fn screenshot(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    info!("Screenshot request from {}", addr);
    
    let screens = Screen::all().unwrap();
    let images: Vec<String> = screens
        .into_par_iter()
        .filter_map(|screen| {
            screen.capture().ok().map(|image| {
                let buffer = image.buffer();
                general_purpose::STANDARD.encode(buffer)
            })
        })
        .collect();

    Json(images).into_response()
}
