use axum::{
    Json,
    response::IntoResponse
};
use base64::{Engine as _, engine::general_purpose};
use screenshots::Screen;
use serde::Serialize;

#[derive(Serialize)]
pub struct ScreenshotResponse {
    pub images: Vec<String>
}

pub async fn screenshot() -> impl IntoResponse {
    let screens = Screen::all().unwrap();
    let mut images: Vec<String> = vec![];
    
    for screen in screens {
        let image = screen.capture().unwrap();
        let buffer = image.buffer();
        let encoded = general_purpose::STANDARD.encode(buffer);
        images.push(encoded);
    }

    Json(ScreenshotResponse {
        images
    }).into_response()
}
