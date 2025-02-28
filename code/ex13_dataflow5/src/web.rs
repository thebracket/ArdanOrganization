use axum::{Extension, Json, Router};
use axum::routing::get;
use crate::telemetry::{TelemetryEvent, TelemetrySummary};

pub async fn webserver(
    telemetry_tx: flume::Sender<TelemetryEvent>,
) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    let router = Router::new()
        .route("/health", get(health))
        .route("/telemetry", get(telemetry_display))
        .layer(Extension(telemetry_tx)); // We add the transmitter to as a resource
    axum::serve(listener, router).await.unwrap();
}

async fn health() -> &'static str {
    "OK"
}

async fn telemetry_display(
    Extension(tx): Extension<flume::Sender<TelemetryEvent>>,
) -> Json<TelemetrySummary> {
    let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
    let _ = tx.send(TelemetryEvent::GetSummary(reply_tx));
    let summary = reply_rx.await.unwrap();
    Json(summary)
}