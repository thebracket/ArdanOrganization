use std::hash::Hash;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::TimedLruCache;

/// A type-erased timed LRU (Least Recently Used) cache that stores serialized values.
///
/// This cache wraps a [`TimedLruCache`](crate::TimedLruCache) and allows storing any value that
/// implements [`Serialize`] and [`DeserializeOwned`]. The values are serialized using
/// [`bincode`] and stored as a `Vec<u8>`.
pub struct TypeErasedTimedLruCache<KEY> {
    cache: TimedLruCache<KEY, Vec<u8>>,
}

impl<KEY: Hash + Eq + Clone> TypeErasedTimedLruCache<KEY> {
    /// Creates a new `TypeErasedTimedLruCache`.
    ///
    /// # Parameters
    ///
    /// - `max_size`: The maximum number of items the cache can hold.
    /// - `expiration_seconds`: The number of seconds after which an item expires.
    ///
    /// # Examples
    ///
    /// ```
    /// # use timed_lru_cache::TypeErasedTimedLruCache;
    /// let cache = TypeErasedTimedLruCache::<i32>::new(10, 1.0);
    /// ```
    pub fn new(max_size: usize, expiration_seconds: f32) -> Self {
        TypeErasedTimedLruCache {
            cache: TimedLruCache::new(max_size, expiration_seconds),
        }
    }

    /// Checks whether the cache contains the specified key.
    ///
    /// # Parameters
    ///
    /// - `key`: A reference to the key to search for.
    ///
    /// # Returns
    ///
    /// `true` if the key exists in the cache, otherwise `false`.
    pub fn contains_key(&mut self, key: &KEY) -> bool {
        self.cache.contains_key(key)
    }

    /// Inserts a value into the cache associated with the specified key.
    ///
    /// The value is serialized before being stored. If the key already existed,
    /// the previous value is returned after deserialization.
    ///
    /// # Type Parameters
    ///
    /// - `VALUE`: The type of the value. Must implement [`Serialize`] and [`DeserializeOwned`].
    ///
    /// # Parameters
    ///
    /// - `key`: The key with which to associate the value.
    /// - `value`: The value to be inserted.
    ///
    /// # Returns
    ///
    /// An `Option<VALUE>` containing the previous value if it existed.
    pub fn insert<VALUE>(&mut self, key: KEY, value: VALUE) -> Option<VALUE>
    where
        VALUE: Serialize + DeserializeOwned,
    {
        let value = bincode::serialize(&value).unwrap();
        self.cache.insert(key, value)
            .map(|value| {
                bincode::deserialize(&value).unwrap()
            })
    }

    /// Retrieves a value from the cache corresponding to the specified key.
    ///
    /// The value is deserialized before being returned.
    ///
    /// # Type Parameters
    ///
    /// - `VALUE`: The type of the value. Must implement [`DeserializeOwned`].
    ///
    /// # Parameters
    ///
    /// - `key`: A reference to the key whose value should be retrieved.
    ///
    /// # Returns
    ///
    /// An `Option<VALUE>` containing the deserialized value if the key exists, otherwise `None`.
    pub fn get<VALUE>(&mut self, key: &KEY) -> Option<VALUE>
    where
        VALUE: DeserializeOwned,
    {
        self.cache.get(key).map(|value| {
            bincode::deserialize(value).unwrap()
        })
    }

    /// Returns the number of items currently stored in the cache.
    pub fn len(&mut self) -> usize {
        self.cache.len()
    }

    /// Checks if the cache is empty.
    ///
    /// # Returns
    ///
    /// `true` if the cache is empty, otherwise `false`.
    pub fn is_empty(&mut self) -> bool {
        self.cache.is_empty()
    }

    pub fn find_many<VALUE>(&mut self, mut predicate: impl FnMut((&KEY, &VALUE))->bool)
        -> Vec<(&KEY, VALUE)>
    where VALUE: DeserializeOwned
    {
        self.cache.find_many(|(key, value)| {
            let value: VALUE = bincode::deserialize(value).unwrap();
            predicate((key, &value))
        }).into_iter().map(|(key, value)| {
            let value: VALUE = bincode::deserialize(value).unwrap();
            (key, value)
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use super::*;

    #[test]
    fn test_new() {
        let mut cache = TypeErasedTimedLruCache::<i32>::new(10, 1.0);
        assert!(!cache.contains_key(&1));
    }

    #[test]
    fn test_insert_retrieve() {
        let mut cache = TypeErasedTimedLruCache::<i32>::new(10, 1.0);
        cache.insert(1, 2);
        assert_eq!(cache.len(), 1);
        let result: Option<i32> = cache.get(&1);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_fill() {
        let mut cache = TypeErasedTimedLruCache::<i32>::new(10, 1.0);
        for i in 0..15 {
            cache.insert(i, i);
        }
        assert_eq!(cache.len(), 10);
    }

    #[test]
    fn test_struct() {
        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            a: i32,
            b: String,
        }

        let mut cache = TypeErasedTimedLruCache::<i32>::new(10, 1.0);
        let value = TestStruct { a: 1, b: "hello".to_string() };
        cache.insert(1, value);
        let result: Option<TestStruct> = cache.get(&1);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.a, 1);
        assert_eq!(result.b, "hello");
    }

    #[test]
    fn test_update() {
        let mut cache = TypeErasedTimedLruCache::<i32>::new(10, 1.0);
        // Insert initial value.
        assert!(cache.insert(1, 10).is_none());
        // Insert again with the same key; should return the old value.
        assert_eq!(cache.insert(1, 20), Some(10));
        let result: Option<i32> = cache.get(&1);
        assert_eq!(result, Some(20));
    }

    #[test]
    fn test_is_empty() {
        let mut cache = TypeErasedTimedLruCache::<i32>::new(10, 1.0);
        assert!(cache.is_empty());
        cache.insert(1, 100);
        assert!(!cache.is_empty());
    }

    #[test]
    fn test_string_key() {
        let mut cache = TypeErasedTimedLruCache::<String>::new(10, 1.0);
        cache.insert("key".to_string(), "value".to_string());
        let result: Option<String> = cache.get(&"key".to_string());
        assert_eq!(result, Some("value".to_string()));
    }
}
