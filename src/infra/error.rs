use actix_web::{ResponseError, body::BoxBody, http::StatusCode};
use sqlx::postgres::PgDatabaseError;
use thiserror::Error;

use super::logging;

#[derive(Debug, Error)]
pub enum AppError {
    NotFound,
    InternalServerError,
    BadRequest,
    Unauthorized,
    Forbidden,
    Conflict,
    UnprocessableEntity,
    TooManyRequests,
    ServiceUnavailable,
    GatewayTimeout,
    Unknown,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            AppError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            AppError::GatewayTimeout => StatusCode::GATEWAY_TIMEOUT,
            AppError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        let res = actix_web::HttpResponse::new(self.status_code());
        logging::log_error(&format!("Error: {:?}", self));
        res.set_body(BoxBody::new(format!("{}", &self)))
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AppError::NotFound => write!(f, "Not Found"),
            AppError::InternalServerError => write!(f, "Internal Server Error"),
            AppError::BadRequest => write!(f, "Bad Request"),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::Forbidden => write!(f, "Forbidden"),
            AppError::Conflict => write!(f, "Conflict"),
            AppError::UnprocessableEntity => write!(f, "Unprocessable Entity"),
            AppError::TooManyRequests => write!(f, "Too Many Requests"),
            AppError::ServiceUnavailable => write!(f, "Service Unavailable"),
            AppError::GatewayTimeout => write!(f, "Gateway Timeout"),
            AppError::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Error representing a failure at the database layer.
#[derive(Debug, Error)]
pub enum DbError {
    /// Not found.
    #[error("entity not found")]
    NotFound,
    /// Conflict.
    #[error("entity already exists")]
    Conflict,
    /// Connection error.
    #[error("could not connect to database")]
    ConnectionError,
    /// Connection error.
    #[error("postgres error: {0}")]
    PgDatabaseError(Box<PgDatabaseError>),
    /// Other error.
    #[error("{0}")]
    Other(sqlx::Error),
}

impl ResponseError for DbError {
    fn status_code(&self) -> StatusCode {
        match self {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::Conflict => StatusCode::CONFLICT,
            DbError::ConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::PgDatabaseError(e) => match e.code() {
                "23514" => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            DbError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for DbError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => DbError::NotFound,
            sqlx::Error::Io(_) => DbError::ConnectionError,
            sqlx::Error::Database(e) => {
                // Check if PostgreSQL error
                let pg_error = e.try_downcast::<PgDatabaseError>();
                match pg_error {
                    Ok(pg_error) => DbError::PgDatabaseError(pg_error),
                    Err(e) => DbError::Other(sqlx::Error::Database(e)),
                }
            }
            e => DbError::Other(e),
        }
    }
}
