use flume::Sender;

pub enum TelemetryEvent {
    IngestorCapacity ( f32 ),
    IngestorMessagesPerSecond { id: usize, mps: f32 },
}

pub async fn start_telemetry() -> Sender<TelemetryEvent> {
    let (tx, rx) = flume::unbounded();
    tokio::spawn(telemetry(rx));
    tx
}

async fn telemetry(
    rx: flume::Receiver<TelemetryEvent>,
) {
    while let Ok(event) = rx.recv_async().await {
        match event {
            TelemetryEvent::IngestorCapacity(capacity) => {
                println!("Ingestor capacity: {:.2}%", capacity);
            }
            TelemetryEvent::IngestorMessagesPerSecond { id, mps } => {
                println!("Ingestor {id} messages per second: {:.2}", mps);
            }
        }
    }
    println!("Telemetry shutting down");
}