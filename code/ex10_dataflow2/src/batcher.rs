use crate::ingestor::IngestorData;

pub async fn start_batchers(
    num_batchers: usize,
    from_ingestors: flume::Receiver<IngestorData>,
) {
    for id in 0..num_batchers {
        let my_rx = from_ingestors.clone();
        tokio::spawn(batch_data(id, my_rx));
    }
}

async fn batch_data(id: usize, rx: flume::Receiver<IngestorData>) {
    while let Some(data) = rx.recv_async().await.ok() {
        println!("Batcher {} got {:?}", id, data);
    }
}