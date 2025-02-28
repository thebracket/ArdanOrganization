mod telemetry;

#[tokio::main]
async fn main() {
    let telemetry = telemetry::start_telemetry().await;

    // Your Heavy Processing Happens Here
    for i in 0 .. 100 {
        telemetry.send(telemetry::Telemetry::Event(i)).await.unwrap();
    }

    // Now we stop gracefully
    telemetry::stop_telemetry(telemetry).await;
}
