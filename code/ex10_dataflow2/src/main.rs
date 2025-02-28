mod ingestor;
mod batcher;

const NUM_INGESTORS: usize = 8;
const INGESTOR_CHANNEL_DEPTH: usize = 100;
const NUM_BATCHERS: usize = 4;

#[tokio::main]
async fn main() {
    // Make the channel for ingestors to feed the next layer
    let (ingest_tx, ingest_rx) = flume::bounded(INGESTOR_CHANNEL_DEPTH);
    ingestor::start_ingestors(NUM_INGESTORS, ingest_tx).await;
    batcher::start_batchers(NUM_BATCHERS, ingest_rx).await;

    // Sleep for a couple of minutes to see the output
    tokio::time::sleep(std::time::Duration::from_secs(120)).await;
}
