# Localize the Global

So the first step is pretty obvious if you've been using Rust for a bit. You isolate the global state to a single module, and provide well-defined access functions.

> The code for this is in `ex01_local_globals`.

Let's start by creating a project, and immediately adding a module - `master_cache.rs`:

We'll solve the key-collision problem by strongly typing our access key:

```rust
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum DocumentKey {
    PageSnippet(usize),
}
```

And then define our cache:

```rust
static MASTER_CACHE: LazyLock<Mutex<TypeErasedTimedLruCache<DocumentKey>>> = LazyLock::new(|| {
    Mutex::new(
        TypeErasedTimedLruCache::new(1024, 60.0)
    )
});
```

Notice that the cache is no longer global.