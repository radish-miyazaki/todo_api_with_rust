use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to initialize the connection pool")]
    ConnectionPoolError(#[from] r2d2::Error),
    #[error("Failed to get the connection from the pool")]
    SQLiteError(#[from] rusqlite::Error),
    #[error("Failed to parse json")]
    ParseJSONError(#[from] serde_json::Error),
}

impl ResponseError for AppError {}
