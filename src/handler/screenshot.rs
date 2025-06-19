use axum::{response::IntoResponse, Json};
use base64::{engine::general_purpose, Engine as _};
use rayon::prelude::*;
use screenshots::Screen;

pub async fn screenshot() -> impl IntoResponse {
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
