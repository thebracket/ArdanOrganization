# A Telemetry Sidecar

It's pretty common to make a Rust program that focuses heavily on calculation. You take input, crunch the numbers (maybe using Rayon or hand-rolled threading) and produce an output. The program runs on its own, launched as part of a pipeline.

Suddenly, you need to submit telemetry data during execution (it may take a while). But you also want to minimize the impact of the telemetry process, and mimiize the *cognitive load* of the data-science people building the overall calculation.

> Load the demo, `ex03_thread_telemetry`

The telemetry module is pretty self contained, and easy to turn into a shared module:

```rust
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
```

The main program highlights how you can use the channel to "get out of the way", and still ensure that all events are sent before you stop:

```rust
mod telemetry;

fn main() {
    let telemetry = telemetry::start_telemetry();

    // Your Heavy Processing Happens Here
    for i in 0 .. 100 {
        telemetry.send(telemetry::Telemetry::Event(i)).unwrap();
    }

    // Now we stop gracefully
    telemetry::stop_telemetry(telemetry);
}
```

## OneShot

MPSC channels are multi-producer, single-consumer. They are designed for multiple senders to locklessly submit data into the channel - which is serialized by a single receiver. This is great for telemetry type scenarios - and avoids any potential delays caused by doing something more complicated than printing messages!

OneShot is a special channel type (from the `oneshot` crate - `tokio` has it for async, also) designed to send one message and then close. It's super-efficient, and really handy when you need a reply.

We use `oneshot` for closing down the telemetry system. It won't reply until the quit message has been received, so it's safe to quit the program - knowing that all previously enqueued messages are processed.
