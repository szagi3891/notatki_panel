use warp::http::Response;

use super::create_response_message;

#[derive(Debug)]
pub enum ErrorProcess {
    User {
        message: String,
    },
    Server {
        message: String,
    }
}

impl ErrorProcess {
    pub fn user<T: Into<String>>(message: T) -> ErrorProcess {
        ErrorProcess::User {
            message: message.into(),
        }
    }

    pub fn server<T: Into<String>>(message: T) -> ErrorProcess {
        ErrorProcess::Server {
            message: message.into(),
        }
    }

    pub fn to_response(self) -> Response<String> {
        match self {
            ErrorProcess::Server { message } => create_response_message(500, message),
            ErrorProcess::User { message } => create_response_message(400, message),
        }
    }
}
