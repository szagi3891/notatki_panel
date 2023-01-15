use poem_openapi::ApiResponse;
use poem_openapi::types::{ToJSON, ParseFromJSON};
use poem_openapi::payload::{
    // Binary,
    Json
};
use std::convert::Infallible;
use std::ops::FromResidual;
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

    pub fn from_error_process(error: ErrorProcess) -> ApiResponseHttp<T> {
        let (internal, message) = error.to_string();
        match internal {
            true => ApiResponseHttp::Internal(Json(message)),
            false => ApiResponseHttp::User(Json(message)),
        }
    }

    pub fn from(value: Result<T, ErrorProcess>) -> ApiResponseHttp<T> {
        match value {
            Ok(value) => ApiResponseHttp::Ok(Json(value)),
            Err(error) => ApiResponseHttp::from_error_process(error)
        }
    }
}

// std::ops::FromResidual<std::result::Result<std::convert::Infallible, utils::error::ErrorProcess>>` is not implemented for `utils::response::ApiResponseHttp<T>`

impl<T:ToJSON + ParseFromJSON> FromResidual<Result<Infallible, ErrorProcess>> for ApiResponseHttp<T> {
    fn from_residual(residual: Result<Infallible, ErrorProcess>) -> Self {
        match residual {
            Ok(_) => {
                unreachable!();
            }
            Err(error) => {
                ApiResponseHttp::from(Err(error))
            }
        }
    }
}


// #[derive(ApiResponse)]
// enum GetResponse {
//     #[oai(status = 200)]
//     #[oai(content_type="")]
//     ImageFile(
//         Binary<Vec<u8>>
//     ),

//     #[oai(status = 404)]
//     NotFound(PlainText<String>),
// }