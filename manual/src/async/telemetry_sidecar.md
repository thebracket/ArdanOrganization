# An Async Telemetry Sidecare

Let's go through `ex06_async_telemetry`. It's pretty much the same as the threaded version, except:

* We have an `async` main function using `tokio::main`.
* We have to remember to `await` our async calls.
* We've replaced `std::sync::mpsc` with `tokio::sync::mpsc`, and `oneshot` with `tokio::sync::oneshot`.
* We're awaiting all our async calls.
* Instead of `thread::spawn`, we have `tokio::spawn`.

And best of all - it's slightly faster, and still relatively light on resources. Async really shines with this pattern.