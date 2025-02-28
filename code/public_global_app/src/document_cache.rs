use std::sync::{LazyLock, Mutex};
use serde::{Deserialize, Serialize};
use timed_lru_cache::TypeErasedTimedLruCache;

pub static DOCUMENTS: LazyLock<Mutex<TypeErasedTimedLruCache<String>>>
    = LazyLock::new(|| Mutex::new(TypeErasedTimedLruCache::new(1024, 60.0)));

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: usize,
    pub tags: Vec<String>,
    pub author: String,
    pub body: String,
}
