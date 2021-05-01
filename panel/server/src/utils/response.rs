use warp::http::Response;
use serde::Serialize;

pub fn create_response<R: Serialize>(code: u16, response: R) -> Response<String> {
    let out = serde_json::to_string(&response);

    match out {
        Ok(out) => {
            Response::builder()
                .status(code)
                .body(out)
                .unwrap()
        },
        Err(err) => {
            Response::builder()
                .status(500)
                .body(format!("Serde error: {}", err))
                .unwrap()
        }
    }
}

pub fn create_response_message<M: Into<String>>(code: u16, response: M) -> Response<String> {
    Response::builder()
        .status(code)
        .body(response.into())
        .unwrap()
}

