use poem_openapi::{ApiResponse, payload::{PlainText, Binary}};

#[derive(ApiResponse)]
pub enum StaticResponse {
    #[oai(status = 500)]
    InternalServerError(
        PlainText<String>,
        #[oai(header = "ContentType")] String
    ),

    #[oai(status = 404)]
    NotFound(
        PlainText<String>,
        #[oai(header = "ContentType")] String
    ),

    #[oai(status = 200)]
    Binary(
        Binary<Vec<u8>>,
        #[oai(header = "ContentType")] String
    ),
}

impl StaticResponse {
    pub fn internal_server(message: impl Into<String>) -> StaticResponse {
        let message = message.into();
        StaticResponse::InternalServerError(
            PlainText(message),
            "text/html".into(),
        )
    }

    pub fn not_found() -> StaticResponse {
        StaticResponse::NotFound(
            PlainText("Not found".into()),
            "text/html".into(),
        )
    }

    pub fn binary(header: impl Into<String>, body: Vec<u8>) -> StaticResponse {
        StaticResponse::Binary(
            Binary(body),
            header.into(),
        )
    }
}
