use vertigo::{RequestResponse};
use common::RootResponse;

pub fn check_request_response(response: RequestResponse) -> Result<(), String> {
    if response.status() == Some(200) {
        match response.into_data::<RootResponse>() {
            Ok(_) => Ok(()),
            Err(message) => Err(format!("status 200, error decode = {message}")),
        }
    } else {
        let status = response.status();
        Err(format!("http response = {status:?}"))
    }
}

