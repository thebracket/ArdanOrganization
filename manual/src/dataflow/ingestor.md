# A Simple Ingestor

Let's look at `ex09_dataflow1`. We've built a very simple ingestor - basically a simulator.

The ingestor code:

```rust
use tokio::time::sleep;

#[derive(Debug, Clone, Copy)]
pub struct IngestorData {
    id: usize,
    counter: u32,
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
```

And a simple client that prints results:

```rust
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
```

See how we're setting constants? We'll be playing with values as the system deeend. Right now, it's pretty useless - it prints out counters.