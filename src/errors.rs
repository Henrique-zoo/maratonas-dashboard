use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub enum AppError {
    Database(sqlx::Error),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}