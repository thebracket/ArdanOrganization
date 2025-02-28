# Multi-Lane Actor

We can use the `flume` or `crossbeam` crates for a threaded Multi-Producer, Multi-Consumer channel that lets you have a scalable number of lanes for input, as well as readily clonable submission channels.

> See the example: `ex05_thread_flume`.

This almost exactly the same, but we're using *flume* instead - and we're firing up 5 threads:

```rust
pub fn start_cache_actor() {
    let (tx, rx) = flume::unbounded();

    for n in 0 .. 5 {
        let rx = rx.clone();
        std::thread::spawn(move || {
            cache_actor(rx, n);
        });
    }

    let _ = ACTOR_STORE.set(tx);
}
```

And now when you run it, the output shows that the load is being equally divided:

```
Command received by actor 0
Command received by actor 1
0: None
Command received by actor 2
Command received by actor 3
1: None
Command received by actor 4
Command received by actor 0
(etc)
```

Now, there's a major issue with using this as a cache design: each cache will only have 1/5th of the cache - and it's not at all clear to which cache we will be routed.

That's not a problem if you are using a remote cache (Redis, Memcached, etc.) and just want to divide the workload. If you want a single cache - you can drop in the same locking we used before.