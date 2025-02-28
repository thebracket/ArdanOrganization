# Async and Channels

Channels work really well with Async. If you're familiar with Go and Bill's execellent classes, Rust + Tokio in Async is very Go-Like:

* 1 thread per core.
* Each core has a work queue, with work stealing.
* Tasks can spread across cores.

Tokio doesn't interrupt tasks as frequently as Go, but can provide a highly scalable system - devoid of context switches, it's basically the same green threading setup as Go and Erlang.

Channels in async don't have to wait for a thread context switch (sometimes it happens anyway), so sometimes the latency for channels in async Rust is significantly lower. There's also less overhead to spawning a task than spawning a thread - you can spawn tens of thousands of async tasks.

Even better: Tokio provides an `mpsc` channel very similar to the threaded one, and `Flume` supports both models of operation. So we won't have to change very much.
