mod ingestor;
mod batcher;
mod logger;
mod moving_average;
mod telemetry;
mod web;

const NUM_INGESTORS: usize = 8;
const INGESTOR_CHANNEL_DEPTH: usize = 100;
const NUM_BATCHERS: usize = 4;
const LOGGER_CHANNEL_DEPTH: usize = 65536;
const BATCH_SIZE: usize = 10;
const MOVING_AVERAGE_DEPTH: usize = 1;

#[tokio::main]
async fn main() {
    // Start telemetry
    let telemetry_sender = telemetry::start_telemetry().await;

    // Make the channel for ingestors to feed the next layer
    let (ingest_tx, ingest_rx) = flume::bounded(INGESTOR_CHANNEL_DEPTH);
    ingestor::start_ingestors(NUM_INGESTORS, ingest_tx.clone(), telemetry_sender.clone()).await;

    // The batcher will be sending data to other locations
    let (logger_tx, logger_rx) = flume::bounded(LOGGER_CHANNEL_DEPTH);
    let (average_tx, average_rx) = flume::bounded(MOVING_AVERAGE_DEPTH);
    batcher::start_batchers(NUM_BATCHERS, ingest_rx, logger_tx, average_tx).await;

    // Launch the logger
    logger::start_logger(logger_rx).await;

    // Launch the moving average
    moving_average::start_moving_average(1, average_rx).await;

    // Launch the web manager
    web::webserver(telemetry_sender).await;
}
