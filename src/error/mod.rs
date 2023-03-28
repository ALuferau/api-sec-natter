#[derive(Debug)]
pub enum Error {
    ConfigurationError(String),
    DatabaseQueryError(sqlx::Error),
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
        }
    }
}

impl warp::reject::Reject for Error {}

pub async fn return_error(r: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(Error::DatabaseQueryError(e)) = r.find() {
        Ok(warp::reply::with_status(
            "Database Query Error".to_string(),
            warp::hyper::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            warp::hyper::StatusCode::NOT_FOUND,
        ))
    }
}
