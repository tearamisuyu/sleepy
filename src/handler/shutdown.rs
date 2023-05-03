use axum::response::IntoResponse;
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ShutdownRequest {
    operation: String,
    time: String
}

pub async fn shutdown(req: axum::Json<ShutdownRequest>) -> impl IntoResponse {
    let operation = &*req.operation;
    let time = &req.time;

    println!("Received shutdown request: {} at {}", operation, time);

    match operation {
        "shutdown" => {
            println!("Shutting down");

            if time != "" {
                std::process::Command::new("shutdown")
                    .args(&["/s", "/t", time, "/f"])
                    .spawn()
                    .expect("Failed to shutdown");
                return (StatusCode::OK, String::from("Request received"))
            } else {
                std::process::Command::new("shutdown")
                    .args(&["/s", "/t", "0", "/f"])
                    .spawn()
                    .expect("Failed to shutdown");
                return (StatusCode::OK, String::from("Request received"))
            }

        },
        "cancel" => {
            println!("Cancelling shutdown");
            std::process::Command::new("shutdown")
                .args(&["/a"])
                .spawn()
                .expect("Failed to cancel shutdown");
        },
        _ => {
            return (StatusCode::BAD_REQUEST, String::from("Invalid operation"))
        }
    }


    (StatusCode::OK, String::from("Request received"))
}
