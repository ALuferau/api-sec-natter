use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum Error {
    ConfigurationError(String),
    DatabaseQueryError(sqlx::Error),
    IllegalArgumentException(String),
    AuthenticationError(String),
    AuthorizationError(String),
    ServerError(hyper::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::ConfigurationError(ref err) => {
                write!(f, "Invalid or missed configuration parameter: {}", err)
            }
            Error::DatabaseQueryError(ref err) => {
                write!(f, "Query could not be executed: {}", err)
            }
            Error::IllegalArgumentException(ref err) => {
                write!(f, "Invalid input: {}", err)
            }
            Error::AuthenticationError(ref err) => {
                write!(f, "Forbidden: {}", err)
            }
            Error::AuthorizationError(ref err) => {
                write!(f, "Unauthorized: {}", err)
            }
            Error::ServerError(ref err) => {
                write!(f, "Server error: {}", err)
            }
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(value: hyper::Error) -> Self {
        Error::ServerError(value)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::ConfigurationError(ref _err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            Error::DatabaseQueryError(ref _err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database Query Error".to_string(),
            ),
            Error::IllegalArgumentException(ref err) => {
                (StatusCode::BAD_REQUEST, format!("Invalid input: {}", err))
            }
            Error::AuthenticationError(ref err) => {
                (StatusCode::FORBIDDEN, format!("Forbidden: {}", err))
            }
            Error::AuthorizationError(ref _err) => {
                (StatusCode::UNAUTHORIZED, "Unauthorized".to_string())
            }
            Error::ServerError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };
        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
