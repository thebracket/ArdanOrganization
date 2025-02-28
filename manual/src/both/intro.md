# Part 3: Async AND Threads!

In Rust Foundations, we talked a bit about the need to mix threaded and async together. You can often get by with no async, but async is "viral" - it tends to color parts of your program with async tags that can only be called from an async runtime. At the same time, synchronous/threaded code is often more natural to write.

So it's really common to have a program with a little bit of async attached to its primary mission. For example:

* Many of the Kubernetes integrations are async.
* If you need to attach a webserver, gRPC server or network server for control or monitoring, the better implementations assume async.
* If you are using FFI, and need async on the Rust side - Rust async usually isn't compatible with every other languages' async implementation. It's common to have some channels receiving messages from external synchronous calls that inject tasks into the async core on the Rust side. You can even have async on both and pass messages through intermediaries.

Reaching directly inside a Tokio runtime can be a little perilous - channels provide a great abstraction.
