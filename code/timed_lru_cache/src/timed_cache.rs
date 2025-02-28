use std::{collections::HashMap, hash::Hash, time::Instant};

struct Entry<VALUE> {
    last_access: Instant,
    entry: VALUE
}

/// Provides a generic least-recently-used cache, with maximum
/// capacity.
///
/// ## Example
///
/// ```
/// use timed_lru_cache::TimedLruCache;
///
/// let mut cache = TimedLruCache::new(10, 30.0);
/// cache.insert(1, "Hello".to_string());
/// if let Some(message) = cache.get(&1) {
///     println!("{message}");
/// }
/// ````
pub struct TimedLruCache <KEY, VALUE>
{
    expiration_seconds: f32,
    max_size: usize,
    entries: HashMap<KEY, Entry<VALUE>>,
}

impl <KEY: Hash+Eq+Clone, VALUE> TimedLruCache<KEY, VALUE> {
    /// Create a new TimedLruCache.
    ///
    /// ## Arguments
    ///
    /// * `max_size`: the maximum number of elements to store. Should be >0.
    /// * `expiration_seconds`: the (fractional) number of seconds to retain entries.
    ///
    /// ## Returns
    ///
    /// A newly initialized (capacity allocated but empty) TimedLruCache.
    pub fn new(max_size: usize, expiration_seconds: f32) -> TimedLruCache<KEY, VALUE>
    {
        TimedLruCache::<KEY, VALUE> {
            max_size,
            expiration_seconds,
            entries: HashMap::with_capacity(max_size),
        }
    }

    /// Does the cache contain a key? Performs an age-based removal
    /// to prevent checking stale data.
    ///
    /// ## Arguments
    /// * `key` - a reference to the key for which you wish to search.
    ///
    /// ## Returns
    /// `true` if the key exists, `false` otherwise.
    pub fn contains_key(&mut self, key: &KEY) -> bool {
        self.remove_old();
        self.entries.contains_key(key)
    }

    pub(crate) fn remove_old(&mut self) {
        self.entries.retain(|_key, value| value.last_access.elapsed().as_secs_f32() < self.expiration_seconds);
    }

    /// Inserts a new entry into the cache.
    ///
    /// ## Arguments
    /// * `key`: the new key to insert.
    /// * `value`: the new value to insert.
    ///
    /// ## Returns
    /// `None` if nothing previously used the same key, `Some(old_value)` is an existing
    /// value previoously occupied the same key (which will be updated to the new value).
    pub fn insert(&mut self, key: KEY, value: VALUE) -> Option<VALUE> {
        self.remove_old();

        if self.entries.len() == self.max_size {
            let mut oldest = (f32::MIN, None);
            self.entries.iter().for_each(|(k, v)| {
                let entry_age = v.last_access.elapsed().as_secs_f32();
                if entry_age > oldest.0 {
                    oldest.0 = entry_age;
                    oldest.1 = Some(k.clone());
                }
            });
            if let (_, Some(oldest)) = oldest {
                self.entries.remove(&oldest);
            }
        }

        self.entries
            .insert(key, Entry { last_access: Instant::now(), entry: value})
            .map(|e| e.entry)
    }

    /// Retreive a referebce to a value from the cache.
    ///
    /// ## Arguments
    /// * `key`: the key to retrieve.
    ///
    /// ## Returns
    /// * `None` if no entry is found for  the requested key.
    /// * `Some(&value)` if an entry is found.
    pub fn get(&mut self, key: &KEY) -> Option<&VALUE> {
        self.remove_old();
        let Some(entry) = self.entries.get_mut(key) else {
            return None;
        };
        entry.last_access = Instant::now();
        Some(&entry.entry)
    }

    /// Retrieve a mutable reference to an entry in the cache.
    ///
    /// ## Arguments
    /// * `key` - the key to retrieve.
    ///
    /// ## Returns
    /// * `None` if no entry is found for the provided key.
    /// * `Some(&mut value)` if an entry is found.
    pub fn get_mut(&mut self, key: &KEY) -> Option<&mut VALUE> {
        self.remove_old();
        let Some(entry) = self.entries.get_mut(key) else {
            return None;
        };
        entry.last_access = Instant::now();
        Some(&mut entry.entry)
    }

    /// Returns the length of the cache, after an expiration check.
    pub fn len(&mut self) -> usize {
        self.remove_old();
        self.entries.len()
    }

    /// Returns true if the cache is empty after an expiration check.
    pub fn is_empty(&mut self) -> bool {
        self.remove_old();
        self.entries.is_empty()
    }

    /// Searches the cache for an entry, using a find iterator style predicate.
    ///
    /// ## Arguments
    /// * `predicate` - a closure that receives (key, value) pairs and returns true if they meet the
    /// criteria.
    ///
    /// ## Returns
    /// `None` if no record matched, `Some((&key, &value))` for the first located match.
    pub fn find_one(&mut self, mut predicate: impl FnMut((&KEY, &VALUE)) -> bool) -> Option<(&KEY, &VALUE)>
    {
        self.remove_old();
        self
            .entries
            .iter_mut()
            .find(|(key, value)| predicate((key, &value.entry)))
            .map(|(key, value)| {
                value.last_access = Instant::now();
                (key, &value.entry)
            })
    }

    /// Searches the cache for entries that match the predicate function.
    ///
    /// ## Arguments
    /// * `predicate` - a closure that receives (&key, &value) and returns true if it matches.
    ///
    /// ## Returns
    ///
    /// A vector of (&key, &value) references to matching entries.
    pub fn find_many(&mut self, mut predicate: impl FnMut((&KEY, &VALUE))->bool)  -> Vec<(&KEY, &VALUE)>
    {
        self.remove_old();
        self
            .entries
            .iter_mut()
            .filter_map(|(key, value)| {
                if predicate((key, &value.entry)) {
                    value.last_access = Instant::now();
                    Some((key, &value.entry))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Clears the cache, removing all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn test_new() {
        let mut cache = TimedLruCache::<i32, i32>::new(10, 1.0);
        assert!(!cache.contains_key(&1));
    }

    #[test]
    fn test_insert_retrieve() {
        let mut cache = TimedLruCache::new(10, 1.0);
        cache.insert(1, 2);
        assert_eq!(cache.len(), 1);
        let result = cache.get(&1);
        assert!(result.is_some());
        assert_eq!(*result.unwrap(), 2);
    }

    #[test]
    fn test_fill() {
        let mut cache = TimedLruCache::new(10, 1.0);
        for i in 0 .. 15 {
            cache.insert(i, i);
        }
        assert_eq!(cache.len(), 10);
    }

    #[test]
    fn test_expiration() {
        let mut cache = TimedLruCache::new(10, 0.1);
        cache.insert(1, 2);
        assert_eq!(cache.len(), 1);
        sleep(Duration::from_secs_f32(0.1));
        cache.remove_old();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_find_one() {
        let mut cache = TimedLruCache::new(10, 1.0);
        cache.insert(1, 1);
        cache.insert(1, 2);
        assert!(cache.find_one(|(k, _v)| *k==1).is_some());
        assert!(cache.find_one(|(_k, v)| *v==2).is_some());
        assert!(cache.find_one(|(k, _v)| *k==3).is_none());
    }

    #[test]
    fn test_find_many() {
        let mut cache = TimedLruCache::new(10, 1.0);
        cache.insert(1, 1);
        cache.insert(2, 1);
        cache.insert(3, 2);
        cache.insert(4, 1);

        assert_eq!(cache.find_many(|(_k, v)| *v == 1).len(), 3);
        assert_eq!(cache.find_many(|(_k, v)| *v == 2).len(), 1);
        assert!(cache.find_many(|(_k, v)| *v == 3).is_empty());
    }
}
