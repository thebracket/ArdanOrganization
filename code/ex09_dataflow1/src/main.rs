mod ingestor;

const NUM_INGESTORS: usize = 4;
const INGESTOR_CHANNEL_DEPTH: usize = 100;

#[tokio::main]
async fn main() {
    // Make the channel for ingestors to feed the next layer
    let (ingest_tx, ingest_rx) = flume::bounded(INGESTOR_CHANNEL_DEPTH);
    ingestor::start_ingestors(NUM_INGESTORS, ingest_tx).await;

    // Print the data stream
    while let Ok(data) = ingest_rx.recv_async().await {
        println!("{:?}", data);
    }
}
