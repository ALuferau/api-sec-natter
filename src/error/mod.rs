use serde::Serialize;

#[derive(Debug)]
pub enum Error {
    ConfigurationError(String),
    DatabaseQueryError(sqlx::Error),
    IllegalArgumentException(String),
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

impl warp::reject::Reject for Error {}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn return_error(r: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(Error::DatabaseQueryError(_)) = r.find() {
        Ok(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: "Database Query Error".to_string(),
            }),
            warp::hyper::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::IllegalArgumentException(e)) = r.find() {
        Ok(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: format!("Invalid input: {}", &e),
            }),
            warp::hyper::StatusCode::BAD_REQUEST,
        ))
    } else if let Some(Error::ServerError(_)) = r.find() {
        Ok(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: "Internal server error".to_string(),
            }),
            warp::hyper::StatusCode::BAD_REQUEST,
        ))
    } else {
        tracing::event!(tracing::Level::ERROR, "response::unspecified error {:?}", &r);
        Ok(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: String::from("Bad request"),
            }),
            warp::hyper::StatusCode::BAD_REQUEST,
        ))
    }
}
