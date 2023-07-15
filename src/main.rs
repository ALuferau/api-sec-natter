use axum::{
    middleware::{self, map_response},
    response::Response,
    routing::post,
    Router,
};
use std::sync::Arc;
use std::time::Duration;
use tower::{make::Shared, ServiceBuilder};
use tower_http::compression::CompressionLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod controller;
mod error;
mod model;
mod store;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    // load config
    dotenv::dotenv().ok();

    let log_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "api_sec_natter=info,warp=debug".into());

    tracing_subscriber::registry()
        .with(log_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::new().expect("Invalid configuration");

    // initialize store
    let store = store::Store::new_from_config(&config).await;

    // create routes
    let store_filter = Arc::new(store);

    let space_routes = Router::new()
        .route("/", post(controller::space::create_space))
        .route_layer(middleware::from_fn_with_state(
            store_filter.clone(),
            controller::user::authenticate,
        ));

    let message_routes = Router::new()
        .route("/", post(controller::message::create_message))
        .route_layer(middleware::from_fn_with_state(
            store_filter.clone(),
            controller::user::authenticate,
        ));

    let user_routes = Router::new().route("/", post(controller::user::register_user));

    let api_routes = Router::new()
        .nest("/spaces", space_routes)
        .nest("/users", user_routes)
        .nest("/messages", message_routes)
        .with_state(store_filter);

    let web_service = ServiceBuilder::new()
        .concurrency_limit(5)
        .buffer(5)
        .rate_limit(100, std::time::Duration::from_secs(1))
        .timeout(Duration::from_secs(10))
        .layer(CompressionLayer::new())
        .layer(map_response(set_general_headers))
        .service(api_routes);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = std::net::TcpListener::bind(addr).unwrap();
    // run server
    hyper::Server::from_tcp(listener)
        .unwrap()
        .serve(Shared::new(web_service))
        .await?;

    Ok(())
}

async fn set_general_headers<B>(mut response: Response<B>) -> Response<B> {
    let headers = response.headers_mut();
    headers.insert(
        "X-Content-Type-Options",
        hyper::header::HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        "X-Frame-Options",
        hyper::header::HeaderValue::from_static("DENY"),
    );
    headers.insert(
        "X-XSS-Protection",
        hyper::header::HeaderValue::from_static("0"),
    );
    headers.insert(
        "Cache-Control",
        hyper::header::HeaderValue::from_static("no-store"),
    );
    headers.insert(
        "Content-Security-Policy",
        hyper::header::HeaderValue::from_static(
            "default-src 'none'; frame-ancestors 'none'; sandbox",
        ),
    );
    headers.insert(
        "Strict-Transport-Security",
        hyper::header::HeaderValue::from_static("max-age=31536000"),
    );
    headers.insert("Server", hyper::header::HeaderValue::from_static(""));

    response
}
