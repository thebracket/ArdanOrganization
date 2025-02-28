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
