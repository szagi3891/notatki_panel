use poem_openapi::ApiResponse;
use poem_openapi::types::{ToJSON, ParseFromJSON};
use poem_openapi::{payload::Json};

use super::ErrorProcess;

#[derive(ApiResponse)]
pub enum ApiResponseHttp<T: Send + ToJSON + ParseFromJSON> {
    #[oai(status = 200)]
    Ok(Json<T>),
    #[oai(status = 400)]
    User(Json<String>),
    #[oai(status = 401)]
    Unauthorized(Json<String>),
    #[oai(status = 404)]
    NotFound(Json<String>),
    #[oai(status = 500)]
    Internal(Json<String>),
}

impl<T: Send + ToJSON + ParseFromJSON> ApiResponseHttp<T> {
    pub fn ok(value: T) -> ApiResponseHttp<T> {
        ApiResponseHttp::Ok(Json(value))
    }

    pub fn not_found(value: impl Into<String>) -> ApiResponseHttp<T> {
        let value = value.into();
        ApiResponseHttp::NotFound(Json(value))
    }

    pub fn internal(value: impl Into<String>) -> ApiResponseHttp<T> {
        let value = value.into();
        ApiResponseHttp::Internal(Json(value))
    }

    pub fn from(value: Result<T, ErrorProcess>) -> ApiResponseHttp<T> {
        match value {
            Ok(value) => ApiResponseHttp::Ok(Json(value)),
            Err(ErrorProcess::Server { message, context }) => {
                ApiResponseHttp::Internal(Json(format_message(context, message)))
            },
            Err(ErrorProcess::User { message, context }) => {
                ApiResponseHttp::User(Json(format_message(context, message)))
            }
        }
    }
}

fn format_message(context: Vec<String>, message: String) -> String {
    let context = context.as_slice().join(",");
    format!("{} context=({})", message, context)
}
