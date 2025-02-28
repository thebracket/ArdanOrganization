use std::collections::HashMap;
use flume::Sender;
use serde::Serialize;

pub enum TelemetryEvent {
    IngestorCapacity ( f32 ),
    IngestorMessagesPerSecond { id: usize, mps: f32 },
    GetSummary ( tokio::sync::oneshot::Sender<TelemetrySummary> ),
}

#[derive(Serialize, Clone)]
pub struct TelemetrySummary {
    pub ingestor_capacity_percent: f32,
    pub ingestor_messages_per_second: HashMap<usize, f32>,
}

pub async fn start_telemetry() -> Sender<TelemetryEvent> {
    let (tx, rx) = flume::unbounded();
    tokio::spawn(telemetry(rx));
    tx
}

async fn telemetry(
    rx: flume::Receiver<TelemetryEvent>,
) {
    let mut stats = TelemetrySummary {
        ingestor_capacity_percent: 0.0,
        ingestor_messages_per_second: Default::default(),
    };
    while let Ok(event) = rx.recv_async().await {
        match event {
            TelemetryEvent::IngestorCapacity(capacity) => {
                println!("Ingestor capacity: {:.2}%", capacity);
                stats.ingestor_capacity_percent = capacity;
            }
            TelemetryEvent::IngestorMessagesPerSecond { id, mps } => {
                println!("Ingestor {id} messages per second: {:.2}", mps);
                stats.ingestor_messages_per_second.insert(id, mps);
            }
            TelemetryEvent::GetSummary(reply) => {
                let _ = reply.send(stats.clone());
            }
        }
    }
    println!("Telemetry shutting down");
}