use std::env;

#[derive(Debug)]
pub struct Config {
    pub log_level: String,
    pub port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
}

impl Config {
  pub fn new() -> Result<Config, crate::errors::Error> {
        let port = env::var("PORT")
            .map_err(|_| crate::errors::Error::ConfigurationError(String::from("PORT")))?
            .parse::<u16>()
            .map_err(|_| crate::errors::Error::ConfigurationError(String::from("PORT")))?;

        let db_user = env::var("POSTGRES_USER")
            .map_err(|_| crate::errors::Error::ConfigurationError(String::from("POSTGRES_USER")))?;
        let db_password = env::var("POSTGRES_PASSWORD")
            .map_err(|_| crate::errors::Error::ConfigurationError(String::from("POSTGRES_PASSWORD")))?;
        let db_host = env::var("POSTGRES_HOST")
            .map_err(|_| crate::errors::Error::ConfigurationError(String::from("POSTGRES_HOST")))?;
        let db_port = env::var("POSTGRES_PORT")
            .map_err(|_| crate::errors::Error::ConfigurationError(String::from("POSTGRES_PORT")))?
            .parse::<u16>()
            .map_err(|_| crate::errors::Error::ConfigurationError(String::from("POSTGRES_PORT")))?;
        let db_name = env::var("POSTGRES_DB")
            .map_err(|_| crate::errors::Error::ConfigurationError(String::from("POSTGRES_DB")))?;

        let log_level = env::var("LOG_LEVEL").unwrap_or(String::from("warn"));

      Ok(Config {
          log_level,
          port,
          db_user,
          db_password,
          db_host,
          db_port,
          db_name,
      })
  }
}
