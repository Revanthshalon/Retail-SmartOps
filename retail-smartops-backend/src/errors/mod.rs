use axum::http;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalServerError(String),
    #[error("Conflict")]
    Conflict,
    #[error("Bad Request")]
    BadRequest,
    #[error("Service Unavailable")]
    ServiceUnavailable,
    #[error("Too Many Requests")]
    TooManyRequests,
    #[error("Unprocessable Entity")]
    UnprocessableEntity,
    #[error("Not Found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Database Error: {0}")]
    DbError(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::InternalServerError(e) => {
                error!("Internal Server Error: {}", e);
                http::StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::Conflict => http::StatusCode::CONFLICT,
            AppError::BadRequest => http::StatusCode::BAD_REQUEST,
            AppError::ServiceUnavailable => http::StatusCode::SERVICE_UNAVAILABLE,
            AppError::TooManyRequests => http::StatusCode::TOO_MANY_REQUESTS,
            AppError::UnprocessableEntity => http::StatusCode::UNPROCESSABLE_ENTITY,
            AppError::NotFound => http::StatusCode::NOT_FOUND,
            AppError::Forbidden => http::StatusCode::FORBIDDEN,
            AppError::Unauthorized => http::StatusCode::UNAUTHORIZED,
            AppError::DbError(e) => {
                error!("Database Error: {}", e);
                http::StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        status_code.into_response()
    }
}
