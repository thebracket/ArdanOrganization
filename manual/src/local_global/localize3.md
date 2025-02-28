# Localize the Global 3

Finally in `main.rs` we can just use the cache:

```rust
use crate::master_cache::Document;

mod master_cache;

fn main() {
    let doc = Document {
        id: 1,
        tags: vec!["test".to_string()],
        author: "Test".to_string(),
        body: "Test".to_string(),
    };
    master_cache::insert_snippet(1, doc);

    let doc = master_cache::snipped_by_id(1);
    println!("{doc:?}");
}
```

So this should be pretty straightforward, but you've solved some of the problems already:

* The cache is still static and thread safe.
* It's impossible to access it directly from other modules.
* It's much harder to abuse the system now that you are strongly typing what you put into the cache.

