# Localize the Global 2

We're half way there. The global variable is no longer accessible from outside of the cache module, so it's impossible for other users to mess up locking (that's our job!). We've strongly typed the access key, so it's much harder for people to abuse the cache.

Now let's define some functions to allow users to add or remove data from the cache:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: usize,
    pub tags: Vec<String>,
    pub author: String,
    pub body: String,
}

pub fn snipped_by_id(id: usize) -> Option<Document> {
    let key = DocumentKey::PageSnippet(id);
    let mut read_lock = MASTER_CACHE.lock().unwrap();
    read_lock.get(&key)
}

pub fn insert_snippet(id: usize, doc: Document) {
    let key = DocumentKey::PageSnippet(id);
    let mut write_lock = MASTER_CACHE.lock().unwrap();
    write_lock.insert(key, doc);
}
```

And now we have nice access functions --- with no easy way to mess up the locking.