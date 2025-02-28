# Starting the Batcher

> See `ex10_dataflow2`.

Let's start with a very simple batcher. If you remember our design, the batcher receives messages as they exit from ingestors - and act as a "traffic cop" sending them to various systems.

```rust
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
```

And in `main.rs`, we handle the data-flow:

```rust
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
```

The output shows that our multi-lane system is working: we are coalescing 8 channels down into 4:

```
Batcher 0 got IngestorData { id: 2, counter: 0 }
Batcher 0 got IngestorData { id: 1, counter: 0 }
Batcher 0 got IngestorData { id: 5, counter: 0 }
Batcher 0 got IngestorData { id: 6, counter: 0 }
Batcher 0 got IngestorData { id: 7, counter: 0 }
Batcher 2 got IngestorData { id: 3, counter: 0 }
Batcher 1 got IngestorData { id: 0, counter: 0 }
Batcher 3 got IngestorData { id: 4, counter: 0 }
```

> The assumption is that ingestors have more actual work to do, or may be bursty. For now, it's just a steady stream.
