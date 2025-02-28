# The Temptation of Global State

We've all written global state. The Rust Foundations class even includes a few examples of it---when you're learning, making a small tool that isn't designed to become a big tool, throwing something together in a hurry... sometimes, global state happens.

> Global state isn't inherently bad, but it requires that everyone who touches your project be disciplined, strong-willed and never forgets their morning coffee.

## A Simple Example

In Rust Foundations, we used global state (before discussing how to use `Arc` and `Tower` with Axum to share it, instead). Let's build a quick key-value cache---a pattern that happens *all the time* (and may or may not have something like Redis or Memcached on the back-end).

```rust
pub static MY_CACHE: Lazy<RwLock<HashMap<String, Document>>>
```

You can access cache entries from anywhere with:

```rust
let lock = MY_CACHE.read().unwrap();
if let Some(value) = lock.get("My Key") {
    // Use value
}
```

You can insert into the cache with:

```rust
let mut lock = MY_CACHE.write().unwrap();
lock.insert("Hello".to_string(), Document { .. });
```

It's nice and easy to use, it's available everywhere, and what could possibly go wrong?