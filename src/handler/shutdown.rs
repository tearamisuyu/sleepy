use std::net::SocketAddr;
use axum::extract::ConnectInfo;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ShutdownRequest {
    code: u8,
    time: String
}

pub async fn shutdown(ConnectInfo(addr): ConnectInfo<SocketAddr>, req: axum::Json<ShutdownRequest>) -> impl IntoResponse {
    let time: &str = &req.time;

    match req.code {
        0 => {
            info!("Received shutdown request from {}", addr);
            if !time.is_empty() {
                std::process::Command::new("shutdown")
                    .args(&["/s", "/t", time, "/f"])
                    .spawn()
                    .expect("Failed to shutdown");
                (StatusCode::OK, String::from("Request received"))
            } else {
                std::process::Command::new("shutdown")
                    .args(&["/s", "/t", "0", "/f"])
                    .spawn()
                    .expect("Failed to shutdown");
                (StatusCode::OK, String::from("Request received"))
            }

        },
        1 => {
            info!("Received shutdown cancel request from {}", addr);
            info!("Cancelling shutdown");
            std::process::Command::new("shutdown")
                .args(&["/a"])
                .spawn()
                .expect("Failed to cancel shutdown");
            (StatusCode::OK, String::from("Shutdown cancelled"))
        },
        _ => {
            (StatusCode::BAD_REQUEST, String::from("Invalid action"))
        }
    }
}
