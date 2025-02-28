use std::sync::{LazyLock, Mutex};
use serde::{Deserialize, Serialize};
use timed_lru_cache::TypeErasedTimedLruCache;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum DocumentKey {
    PageSnippet(usize),
}

static MASTER_CACHE: LazyLock<Mutex<TypeErasedTimedLruCache<DocumentKey>>> = LazyLock::new(|| {
    Mutex::new(
        TypeErasedTimedLruCache::new(1024, 60.0)
    )
});

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