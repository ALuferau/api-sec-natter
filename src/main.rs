use reqwest::header::{HeaderMap, HeaderValue};
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{Filter};

mod config;
mod controller;
mod error;
mod model;
mod store;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    // load config
    dotenv::dotenv().ok();

    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "api_sec_natter=info,warp=debug".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let config = config::Config::new().expect("Invalid configuration");

    // initialize store
    let store = store::Store::new_from_config(&config).await;

    // create routes
    let store_filter = warp::any().map(move || store.clone());

    let spaces_path = warp::path("spaces");
    let create_space = warp::post()
        .and(spaces_path)
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(controller::space::create_space);

    let mut headers = HeaderMap::new();
    headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert("X-XSS-Protection", HeaderValue::from_static("0"));
    headers.insert("Cache-Control", HeaderValue::from_static("no-store"));
    headers.insert("Content-Security-Policy", HeaderValue::from_static("default-src 'none'; frame-ancestors 'none'; sandbox"));
    headers.insert("Server", HeaderValue::from_static(""));

    let routes = create_space
        .with(warp::trace::request())
        .recover(error::return_error)
        .with(warp::reply::with::headers(headers));

    // run server
    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;

    Ok(())
}
