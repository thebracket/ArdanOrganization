use axum::{Extension, Router};
use axum::extract::Path;
use axum::routing::get;

pub enum CommandMessage {
    DoSomething(String, tokio::sync::oneshot::Sender<String>),
}

pub fn start_controller() -> tokio::sync::mpsc::Receiver<CommandMessage> {
    let (command_tx, command_rx) = tokio::sync::mpsc::channel(100);
    std::thread::spawn(|| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(start(command_tx));
    });
    command_rx
}

async fn start(command_tx: tokio::sync::mpsc::Sender<CommandMessage>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    let router = Router::new()
        .route("/health", get(health))
        .route("/command/{command}", get(command_handler))
        .layer(Extension(command_tx)); // We add the transmitter to as a resource
    axum::serve(listener, router).await.unwrap();
}

async fn health() -> String {
    "OK".to_string()
}

async fn command_handler(
    Path(command): Path<String>,
    Extension(tx): Extension<tokio::sync::mpsc::Sender<CommandMessage>>
) -> String {
    let (one_tx, one_rx) = tokio::sync::oneshot::channel();
    let command_message = CommandMessage::DoSomething(command, one_tx);
    tx.send(command_message).await.unwrap();
    let reply = one_rx.await.unwrap();
    reply
}