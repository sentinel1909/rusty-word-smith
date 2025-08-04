// app/src/errors.rs

// dependencies
use pavex::http::{HeaderValue, StatusCode};
use pavex::{
    Response, error_handler,
    response::body::{
        TypedBody,
        raw::{Bytes, Full},
    },
};
use pavex_static_files::ServeError;
use pavex_tera_template::TemplateError;
use serde::Serialize;
use serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Template error: {0}")]
    TemplateError(#[from] TemplateError),

    #[error("Static file error: {0}")]
    StaticFileError(#[from] ServeError),
}

#[derive(Serialize)]
struct ApiErrorResponse {
    msg: String,
    status: u16,
    details: String,
}

// Custom JSON response type that sets the correct content type
struct JsonResponse(String);

impl TypedBody for JsonResponse {
    type Body = Full<Bytes>;

    fn content_type(&self) -> HeaderValue {
        HeaderValue::from_static("application/json")
    }

    fn body(self) -> Self::Body {
        Full::new(self.0.into())
    }
}

// error handler
#[error_handler]
pub fn api_error2response(error: &ApiError) -> Response {
    let status = match error {
        ApiError::TemplateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        ApiError::StaticFileError(serve_error) => {
            // Check if the error message contains "not found" or similar
            let error_msg = serve_error.to_string().to_lowercase();
            if error_msg.contains("not found") || error_msg.contains("no such file") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    };

    let payload = ApiErrorResponse {
        msg: "Error".to_string(),
        status: status.as_u16(),
        details: error.to_string(),
    };

    let json = serde_json::to_string(&payload).unwrap_or_else(|_| {
        r#"{"msg":"Error","status":500,"details":"Internal server error formatting error response"}"#.to_string()
    });

    Response::new(status).set_typed_body(JsonResponse(json))
}
