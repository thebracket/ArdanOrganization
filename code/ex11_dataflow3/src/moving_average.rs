pub async fn start_moving_average(
    num_averages: usize,
    from_batchers: flume::Receiver<Vec<crate::ingestor::IngestorData>>,
) {
    for id in 0..num_averages {
        let my_rx = from_batchers.clone();
        tokio::spawn(moving_average(id, my_rx));
    }
}

async fn moving_average(
    id: usize,
    rx: flume::Receiver<Vec<crate::ingestor::IngestorData>>,
) {
    let mut sum = 0;
    let mut count = 0;
    while let Some(batch) = rx.recv_async().await.ok() {
        for data in batch {
            sum += data.counter as usize;
            count += 1;
        }
        let average = sum / count;
        println!("Average {}: {}", id, average);
    }
}