use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{BoxError, Router};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_ratelimit::RateLimitLayer;

#[tokio::main]
async fn main() {
    let redis = redis::Client::open("redis://127.0.0.1/").unwrap();
    let redis_connection = redis.get_multiplexed_async_connection().await.unwrap();

    let make_service = |key: &str, rate: u32, window: Duration| {
        let ratelimit = RateLimitLayer::new(
            rate,
            window,
            redis_connection.clone(),
            format!("ratelimit_{key}"),
        );
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|_: BoxError| async {
                StatusCode::TOO_MANY_REQUESTS
            }))
            .layer(ratelimit)
    };

    let app = Router::new()
        .route(
            "/a",
            get(handler).layer(make_service("a", 5, Duration::from_secs(10))),
        )
        .route(
            "/b",
            get(handler).layer(make_service("b", 10, Duration::from_secs(60))),
        )
        .route(
            "/c",
            get(handler).layer(make_service("c", 60, Duration::from_secs(60))),
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> StatusCode {
    StatusCode::NO_CONTENT
}
