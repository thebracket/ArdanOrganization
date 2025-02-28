# An Async Single-Lane Cache

Once again, there's not a lot changed in `ex07_async_actor`. We switched channel types to use `tokio`, added some `await` and spawned a Tokio task rather than a thread.

And again - it's fast and lightweight!

> The actor model is more common in async code. Lower latency, and it avoids many of the difficulties of trying to pass references around.