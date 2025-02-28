use tokio::sync::mpsc::Sender;

pub enum Telemetry {
    Event(usize),
    Stop(tokio::sync::oneshot::Sender<()>),
}

pub async fn start_telemetry() -> Sender<Telemetry> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1024);
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Some(Telemetry::Event(event)) => {
                    println!("Event: {}", event);
                }
                Some(Telemetry::Stop(confirm)) => {
                    confirm.send(()).unwrap();
                    break;
                }
                None => {
                    break;
                }
            }
        }
        println!("Telemetry stopped");
    });
    tx
}

pub async fn stop_telemetry(tx: Sender<Telemetry>) {
    let (confirm_tx, confirm_rx) = tokio::sync::oneshot::channel();
    tx.send(Telemetry::Stop(confirm_tx)).await.unwrap();
    confirm_rx.await.unwrap();
}