# The Pitfalls of Global State

Some time later, your service is thriving! The money is rolling in, and a whole new batch of developers start adding to it. Now your central state store is being used from 10 different places.

## Problem 1: Joey Deadlocked The System, Again

All of a sudden, you are receiving alerts at 2am. The system just stopped responding altogether. Rebooting always fixes it, and you remember that this guy named Herbert mentioned in his Foundations class:

> Rust does not in any protect you from deadlocks, and RwLock is so much easier to deadlock than a Mutex!

Oh dear. Anyway, you set the service to restart after the health-check fails---and at least it's kinda working now. But as it gets busier, it's restarting more and more often. You *could* just shard it and hope that some survive, but that's *really* not a great solution.

Instead, you embark upon a wilderness safari of validating every other team-member's usage of your locks. And you find a (fictional) Joey has made a locking mistake in an infrequently-used (and poorly tested) region of the program

It's really easy to make this mistake:

```rust
fn my_function() {
    let lock = MY_CACHE.read().unwrap();
    if let Some(my_data) = lock.get("Document 12") {
        // Use the variable
    } else {
        let mut lock = MY_CACHE.write().unwrap();
        // Fetch the data and write it
    }
}
```

OOPS. You just deadlocked your thread! The writer will wait *forever* for the first read lock to expire - which will never happen.

So you fix it, send Joey to the Foundations class, and move on.
