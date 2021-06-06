use warp::http::Response;
use git2::Error;
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
    pub fn user<K, T: Into<String>>(message: T) -> Result<K, ErrorProcess> {
        Err(ErrorProcess::User {
            message: message.into(),
        })
    }

    pub fn server<K, T: Into<String>>(message: T) -> Result<K, ErrorProcess> {
        Err(ErrorProcess::Server {
            message: message.into(),
        })
    }

    pub fn to_response(self) -> Response<String> {
        match self {
            ErrorProcess::Server { message } => create_response_message(500, message),
            ErrorProcess::User { message } => create_response_message(400, message),
        }
    }
}

impl From<Error> for ErrorProcess {
    fn from(err: Error) -> ErrorProcess {
        ErrorProcess::Server {
            message: format!("{}", err),
        }
    }
}