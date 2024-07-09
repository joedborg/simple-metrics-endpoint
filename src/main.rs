use axum::{routing::get, Json, Router};
use axum_prometheus::PrometheusMetricLayer;
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Serialize)]
struct Health {
    status: String,
}

#[derive(Serialize)]
struct Ready {
    status: String,
}

async fn hello() -> &'static str {
    "World!"
}

async fn health() -> Json<Health> {
    Json(Health {
        status: "ok".to_string(),
    })
}

async fn ready() -> Json<Ready> {
    Json(Ready {
        status: "ok".to_string(),
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let bind_address = SocketAddr::from(([0, 0, 0, 0], 8080));

    tracing::info!("Listening on {}", &bind_address);

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app = Router::new()
        .route("/hello", get(hello))
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::DEBUG))
                .on_response(trace::DefaultOnResponse::new().level(Level::DEBUG)),
        )
        .layer(prometheus_layer);

    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
