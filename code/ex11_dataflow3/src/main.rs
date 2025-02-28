mod ingestor;
mod batcher;
mod logger;
mod moving_average;

const NUM_INGESTORS: usize = 8;
const INGESTOR_CHANNEL_DEPTH: usize = 100;
const NUM_BATCHERS: usize = 4;
const LOGGER_CHANNEL_DEPTH: usize = 65536;
const BATCH_SIZE: usize = 10;
const MOVING_AVERAGE_DEPTH: usize = 1;

#[tokio::main]
async fn main() {
    // Make the channel for ingestors to feed the next layer
    let (ingest_tx, ingest_rx) = flume::bounded(INGESTOR_CHANNEL_DEPTH);
    ingestor::start_ingestors(NUM_INGESTORS, ingest_tx).await;

    // The batcher will be sending data to other locations
    let (logger_tx, logger_rx) = flume::bounded(LOGGER_CHANNEL_DEPTH);
    let (average_tx, average_rx) = flume::bounded(MOVING_AVERAGE_DEPTH);
    batcher::start_batchers(NUM_BATCHERS, ingest_rx, logger_tx, average_tx).await;

    // Launch the logger
    logger::start_logger(logger_rx).await;

    // Launch the moving average
    moving_average::start_moving_average(1, average_rx).await;

    // Sleep for a couple of minutes to see the output
    tokio::time::sleep(std::time::Duration::from_secs(120)).await;
}
