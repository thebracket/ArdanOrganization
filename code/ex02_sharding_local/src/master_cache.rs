use std::sync::{LazyLock, Mutex};
use serde::{Deserialize, Serialize};
use timed_lru_cache::TypeErasedTimedLruCache;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum DocumentKey {
    PageSnippet(usize),
    User(usize),
}

static DOCUMENT_CACHE: LazyLock<Mutex<TypeErasedTimedLruCache<DocumentKey>>> = LazyLock::new(|| {
    Mutex::new(
        TypeErasedTimedLruCache::new(1024, 60.0)
    )
});

static USER_CACHE: LazyLock<Mutex<TypeErasedTimedLruCache<DocumentKey>>> = LazyLock::new(|| {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub username: String,
    pub email: String,
}

pub fn snippet_by_id(id: usize) -> Option<Document> {
    let key = DocumentKey::PageSnippet(id);
    let mut read_lock = DOCUMENT_CACHE.lock().unwrap();
    read_lock.get(&key)
}

pub fn insert_snippet(id: usize, doc: Document) {
    let key = DocumentKey::PageSnippet(id);
    let mut write_lock = DOCUMENT_CACHE.lock().unwrap();
    write_lock.insert(key, doc);
}

pub fn user_by_id(id: usize) -> Option<User> {
    let key = DocumentKey::User(id);
    let mut read_lock = USER_CACHE.lock().unwrap();
    read_lock.get(&key)
}

pub fn insert_user(id: usize, user: User) {
    let key = DocumentKey::User(id);
    let mut write_lock = USER_CACHE.lock().unwrap();
    write_lock.insert(key, user);
}