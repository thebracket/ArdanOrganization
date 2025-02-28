pub async fn start_logger(rx: flume::Receiver<crate::ingestor::IngestorData>) {
    tokio::spawn(logger(rx));
}

async fn logger(rx: flume::Receiver<crate::ingestor::IngestorData>) {
    while let Some(data) = rx.recv_async().await.ok() {
        println!("Logger got {:?}", data);
    }
}