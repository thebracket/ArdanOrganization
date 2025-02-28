use std::sync::mpsc::Sender;

pub enum Telemetry {
    Event(usize),
    Stop(oneshot::Sender<()>),
}

pub fn start_telemetry() -> Sender<Telemetry> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(Telemetry::Event(event)) => {
                    println!("Event: {}", event);
                }
                Ok(Telemetry::Stop(confirm)) => {
                    confirm.send(()).unwrap();
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        }
        println!("Telemetry stopped");
    });
    tx
}

pub fn stop_telemetry(tx: Sender<Telemetry>) {
    let (confirm_tx, confirm_rx) = oneshot::channel();
    tx.send(Telemetry::Stop(confirm_tx)).unwrap();
    confirm_rx.recv().unwrap();
}