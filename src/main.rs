use tracing_subscriber::fmt::format::FmtSpan;

mod config;
mod errors;
mod store;
mod types;

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
    dotenv::dotenv().ok();

    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "api_sec_natter=info,warp=error".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let config = config::Config::new().expect("Invalid configuration");

    let store = store::Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
    ))
    .await;

    println!("Hello, world!");

    Ok(())
}
