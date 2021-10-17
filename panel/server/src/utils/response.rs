use serde::Serialize;
use axum::{http::StatusCode};

pub fn create_response<R: Serialize>(code: StatusCode, response: R) -> (StatusCode, String) {
    let out = serde_json::to_string(&response);

    match out {
        Ok(out) => {
            (code, out)
        },
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Serde error: {}", err))
        }
    }
}

pub fn create_response_message<M: Into<String>>(code: StatusCode, response: M) -> (StatusCode, String) {
    (code, response.into())
}

