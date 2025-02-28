# Shared Data Actor

Going back to our big mutable document cache, one approach to manage it and reduce locking would be to setup an "actor" system - in which the cache actor is responsible for managing the cache, and interacts with consumers through channelized message passing. This is more common in the async world, but is a common pattern.

> Let's take a look at `ex04_thread_actor_cache`.

In the `cache_actor`, there's a few things to look at:

* We run the actor in its own thread.
* We've added helper functions to not pollute the main program with confusing channel logic.
* We're using `oneshot` to make access synchronous. That doesn't always scale - *don't* do that if you don't need to access the data immediately.

In the main thread, our access is very straightforward:

```rust
mod cache_actor;

fn main() {
    cache_actor::start_cache_actor();

    for i in 0..10 {
        cache_actor::store_integer(i, i * 2);
        let n = cache_actor::get_integer(i);
        println!("{}: {:?}", i, n);
    }

    cache_actor::stop_cache_actor();
}
```
