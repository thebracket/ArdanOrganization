use tokio::time::sleep;
use crate::telemetry::TelemetryEvent;

#[derive(Debug, Clone, Copy)]
pub struct IngestorData {
    pub id: usize,
    pub counter: u32,
}

pub async fn start_ingestors(
    num: usize,
    tx: flume::Sender<IngestorData>,
    telemetry: flume::Sender<TelemetryEvent>,
) {
    for id in 0..num {
        let my_tx = tx.clone();
        tokio::spawn(ingest_data(id, my_tx, telemetry.clone()));
    }
    tokio::spawn(monitor_ingestors(telemetry, tx.clone()));
}

async fn ingest_data(
    id: usize,
    tx: flume::Sender<IngestorData>,
    telemetry: flume::Sender<TelemetryEvent>,
) {
    let mut counter: u32 = 0;
    let mut count_time = std::time::Instant::now();
    loop {
        tx.send_async(IngestorData {
            id,
            counter,
        }).await.unwrap();
        counter = counter.wrapping_add(1);

        let time = count_time.elapsed().as_secs_f32();
        if time > 1.0 {
            telemetry.send_async(TelemetryEvent::IngestorMessagesPerSecond {
                mps: counter as f32 / time,
                id,
            }).await.unwrap();
            count_time = std::time::Instant::now();
            counter = 0;
        }

        //sleep(std::time::Duration::from_secs(1)).await;
    }
}

async fn monitor_ingestors(
    telemetry: flume::Sender<TelemetryEvent>,
    tx: flume::Sender<IngestorData>,
) {
    let mut ticker = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        ticker.tick().await;
        let capacity = tx.len() as f32 / tx.capacity().unwrap_or(1) as f32;
        telemetry.send_async(TelemetryEvent::IngestorCapacity(capacity * 100.0)).await.unwrap();
    }
}