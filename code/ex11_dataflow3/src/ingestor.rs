use tokio::time::sleep;

#[derive(Debug, Clone, Copy)]
pub struct IngestorData {
    pub id: usize,
    pub counter: u32,
}

pub async fn start_ingestors(num: usize, tx: flume::Sender<IngestorData>) {
    for id in 0..num {
        let my_tx = tx.clone();
        tokio::spawn(ingest_data(id, my_tx));
    }
}

async fn ingest_data(id: usize, tx: flume::Sender<IngestorData>) {
    let mut counter: u32 = 0;
    loop {
        tx.send_async(IngestorData {
            id,
            counter,
        }).await.unwrap();
        counter = counter.wrapping_add(1);
        sleep(std::time::Duration::from_secs(1)).await;
    }
}