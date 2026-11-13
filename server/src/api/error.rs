use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;

/// Errors returned from handlers, rendered as a status code + short message.
pub enum ApiError {
    NotFound,
    Db(DbErr),
}

pub type ApiResult<T> = Result<T, ApiError>;

impl From<DbErr> for ApiError {
    fn from(e: DbErr) -> Self {
        ApiError::Db(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found").into_response(),
            ApiError::Db(e) => {
                // Log the details, but don't leak them to clients.
                tracing::error!("database error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
            }
        }
    }
}
