//! Generic Least-Recently Used Cache wtih a maximum size.

mod timed_cache;
mod type_erased_cache;

pub use timed_cache::TimedLruCache;
pub use type_erased_cache::TypeErasedTimedLruCache;
