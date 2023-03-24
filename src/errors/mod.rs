#[derive(Debug)]
pub enum Error {
    ConfigurationError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::ConfigurationError(ref err) => {
                write!(f, "Invalid or missed configuration parameter: {}", err)
            }
        }
    }
}
