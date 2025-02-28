mod web;

fn main() {
    let mut command_rx = web::start_controller();

    // Now we're writing the bulk of the program as synchronous code
    while let Some(cmd) = command_rx.blocking_recv() {
        // Pretend that your heavy duty synchronous code goes here
        if let web::CommandMessage::DoSomething(command, reply_tx) = cmd {
            let reply = format!("You sent me: {}", command);
            reply_tx.send(reply).unwrap();
        }
    }
}
