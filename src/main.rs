use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;
use tower::{make::Shared, ServiceBuilder};
use tower_http::{compression::CompressionLayer};
use std::time::Duration;


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

    let users_path = warp::path("users");
    let register_user = warp::post()
        .and(users_path)
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(controller::user::register_user);

    let mut headers = hyper::header::HeaderMap::new();
    headers.insert("X-Content-Type-Options", hyper::header::HeaderValue::from_static("nosniff"));
    headers.insert("X-Frame-Options", hyper::header::HeaderValue::from_static("DENY"));
    headers.insert("X-XSS-Protection", hyper::header::HeaderValue::from_static("0"));
    headers.insert("Cache-Control", hyper::header::HeaderValue::from_static("no-store"));
    headers.insert("Content-Security-Policy", hyper::header::HeaderValue::from_static("default-src 'none'; frame-ancestors 'none'; sandbox"));
    headers.insert("Server", hyper::header::HeaderValue::from_static(""));

    let routes = register_user
        .or(create_space)
        .with(warp::trace::request())
        .recover(error::return_error)
        .with(warp::reply::with::headers(headers));

    let warp_service = warp::service(routes);

    let web_service = ServiceBuilder::new()
        .concurrency_limit(5)
        .buffer(5)
        .rate_limit(100, std::time::Duration::from_secs(1))
        .timeout(Duration::from_secs(10))
        .layer(CompressionLayer::new())
        .service(warp_service);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = std::net::TcpListener::bind(addr).unwrap();
    // run server
    hyper::Server::from_tcp(listener)
        .unwrap()
        .serve(Shared::new(web_service))
        .await?;

    Ok(())
}
