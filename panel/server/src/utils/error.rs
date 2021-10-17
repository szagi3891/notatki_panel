use std::fmt::Debug;

use axum::{http::StatusCode};
use git2::Error;
use super::create_response_message;

fn format_message(context: Vec<String>, message: String) -> String {
    let context = context.as_slice().join(",");
    format!("{} context=({})", message, context)
}

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

    pub fn to_response(self) -> (StatusCode, String) {
        match self {
            ErrorProcess::Server { message, context } => {
                create_response_message(StatusCode::INTERNAL_SERVER_ERROR, format_message(context, message))
            },
            ErrorProcess::User { message, context } => {
                create_response_message(StatusCode::NOT_FOUND, format_message(context, message))
            }
        }
    }

    pub fn context<T: Debug>(self, label: &str, label_message: T) -> Self {
        match self {
            ErrorProcess::User { mut context, message } => {
                context.push(format!("{} = {:?}", label, label_message));
                ErrorProcess::User { context, message }
            },
            ErrorProcess::Server { mut context, message } => {
                context.push(format!("{} = {:?}", label, label_message));
                ErrorProcess::User { context, message }
            },
        }
    }
}

impl From<Error> for ErrorProcess {
    fn from(err: Error) -> ErrorProcess {
        ErrorProcess::Server {
            context: Vec::new(),
            message: format!("{}", err),
        }
    }
}