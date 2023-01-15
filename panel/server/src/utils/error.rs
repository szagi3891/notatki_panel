use std::fmt::Debug;

use git2::Error;
use super::{response::ApiResponseHttp};
use poem_openapi::types::{ToJSON, ParseFromJSON};

#[derive(Debug)]
pub enum ErrorProcess {
    User {
        context: Vec<String>,
        message: String,
    },
    Server {
        context: Vec<String>,
        message: String,
    }
}

impl ErrorProcess {
    pub fn user<T: Into<String>>(message: T) -> ErrorProcess {
        ErrorProcess::User {
            context: Vec::new(),
            message: message.into(),
        }
    }

    pub fn user_result<K, T: Into<String>>(message: T) -> Result<K, ErrorProcess> {
        Err(ErrorProcess::User {
            context: Vec::new(),
            message: message.into(),
        })
    }

    pub fn server_result<K, T: Into<String>>(message: T) -> Result<K, ErrorProcess> {
        Err(ErrorProcess::Server {
            context: Vec::new(),
            message: message.into(),
        })
    }

    pub fn to_response<T: Send + ToJSON + ParseFromJSON>(self) -> ApiResponseHttp<T> {
        ApiResponseHttp::from(Err(self))
    }

    pub fn context<T: Debug>(self, label: &str, label_message: T) -> Self {
        match self {
            ErrorProcess::User { mut context, message } => {
                context.push(format!("{label} = {label_message:?}"));
                ErrorProcess::User { context, message }
            },
            ErrorProcess::Server { mut context, message } => {
                context.push(format!("{label} = {label_message:?}"));
                ErrorProcess::User { context, message }
            },
        }
    }

    pub fn to_string(self) -> (bool, String) {
        match self {
            ErrorProcess::Server { message, context } => {
                (true, format_message(context, message))
            },
            ErrorProcess::User { message, context } => {
                (false, format_message(context, message))
            }
        }
    }
}

impl From<Error> for ErrorProcess {
    fn from(err: Error) -> ErrorProcess {
        ErrorProcess::Server {
            context: Vec::new(),
            message: format!("{err}"),
        }
    }
}


fn format_message(context: Vec<String>, message: String) -> String {
    let context = context.as_slice().join(",");
    format!("{message} context=({context})")
}

