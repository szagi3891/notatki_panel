use vertigo::{RequestResponse, Resource};
use common::RootResponse;

pub fn check_request_response(response: RequestResponse) -> Result<(), String> {
    if response.status() == Some(200) {
        match response.into_data::<RootResponse>() {
            Resource::Ready(_) => Ok(()),
            Resource::Loading => unreachable!(),
            Resource::Error(message) => Err(format!("status 200, error decode = {message}")),
        }
    } else {
        let status = response.status();
        Err(format!("http response = {status:?}"))
    }
}

