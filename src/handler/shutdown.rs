use axum::response::IntoResponse;
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ShutdownRequest {
    action: String,
    time: String
}

pub async fn shutdown(req: axum::Json<ShutdownRequest>) -> impl IntoResponse {
    let time: &str = &req.time;

    println!("Shutdown in {} seconds", time);

    match req.action.as_str() {
        "shutdown" => {
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
        "cancel" => {
            println!("Cancelling shutdown");
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
