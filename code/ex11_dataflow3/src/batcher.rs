use crate::BATCH_SIZE;
use crate::ingestor::IngestorData;

pub async fn start_batchers(
    num_batchers: usize,
    from_ingestors: flume::Receiver<IngestorData>,
    to_logger: flume::Sender<IngestorData>,
    to_average: flume::Sender<Vec<IngestorData>>,
) {
    for id in 0..num_batchers {
        let my_rx = from_ingestors.clone();
        tokio::spawn(batch_data(id, my_rx, to_logger.clone(), to_average.clone()));
    }
}

async fn batch_data(
    _id: usize,
    rx: flume::Receiver<IngestorData>,
    to_logger: flume::Sender<IngestorData>,
    to_average: flume::Sender<Vec<IngestorData>>,
) {
    let mut batch = Vec::with_capacity(BATCH_SIZE);
    while let Some(data) = rx.recv_async().await.ok() {
        // Log the data
        to_logger.send_async(data.clone()).await.unwrap();

        // Build the batch
        batch.push(data);
        // If the batch is full, send it
        if batch.len() >= BATCH_SIZE {
            to_average.send_async(batch.clone()).await.unwrap();
            batch.clear();
        }
    }
}