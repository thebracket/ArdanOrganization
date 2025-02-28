# Problem 2: Dodgy Data

> Let's take a quick look at the `timed_lru_cache` crate in the code folder. This is a remarkably common pattern. It's a lot like a local Redis.

A ticket arrives that a customer has some really bizarre issues with their data. You check the database, and their data is intact. What's going on? You can't have a data race (and you don't!). It's worse than that, someone didn't read the key naming policy---and customer X is seeing customer Y's data.

There's a lesson here: strongly type your key (NOT a string, please?). Enums are great cache keys, and can embed things like customer IDs to ensure that this *never* happens.

But now you have to meet with every other consumer of your little throw-away cache from day 1---and achieve agreement on how to use it. With any luck, you may even agree on more than one cache for data, and a standardized key system.

So maybe instead of:

```rust
static MY_CACHE: Lazy<Mutex<TypeErasedTimedLruCache<String>>>
```

You could go with:

```rust
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum CacheKey { Document(usize) }

static MY_CACHE: Lazy<Mutex<TypeErasedTimedLruCache<CacheKey>>> = Lazy::new(|| Mutex::new(
    TypeErasedTimedLruCache::new(1024, 60.0)
));
```

You might want to consider that different parts of the program have different cache needs - and divide your cache into smaller caches addressing each part of the program's needs. We'll talk a bit more about that later.

> This problem hits Redis users, too. Your key in a central cache is *really* important!
