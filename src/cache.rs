use std::collections::HashMap;
use tokio::sync::RwLock;

use std::hash::Hash;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Item<T> {
    pub object: T,
    expiry: Option<Instant>,
}

impl<T> Item<T> {
    // Creates a new cache item.
    pub fn new(object: T, item_duration: Option<Duration>) -> Self {
        let expiry = item_duration.map(|duration| Instant::now() + duration);
        Item { object, expiry }
    }

    // Returns true if the item has expired.
    pub fn expired(&self) -> bool {
        self.expiry
            .map(|expiry| expiry < Instant::now())
            .unwrap_or(false)
    }
}

pub struct Cache<T, V> {
    items: RwLock<HashMap<T, Item<V>>>,
}

impl<T, V> Cache<T, V> {
    pub fn new() -> Self {
        Cache {
            items: RwLock::new(HashMap::new()),
        }
    }

    /// Get a cache item associated with a given key.
    pub async fn get(&self, key: &T) -> Option<V>
    where
        T: Eq + Hash,
        V: Clone,
    {
        self.items
            .read()
            .await
            .get(key)
            .filter(|item| !item.expired())
            .map(|item| item.object.clone())
    }

    /// Set an item in the cache with an associated key.
    pub async fn set(&self, key: T, value: V, custom_duration: Option<Duration>) -> Option<V>
    where
        T: Eq + Hash,
    {
        self.items
            .write()
            .await
            .insert(key, Item::new(value, custom_duration))
            .map(|item| item.object)
    }

    /// Remove all expired items from the cache.
    pub async fn remove_expired(&self)
    where
        T: Eq + Hash + Clone,
    {
        let expired_keys = self
            .items
            .read()
            .await
            .iter()
            .filter(|(_, item)| item.expired())
            .map(|(k, _)| k.clone())
            .collect::<Vec<T>>();

        for key in expired_keys {
            self.items.write().await.remove(&key);
        }
    }

    /// Remove an item from the cache associated with a given key.
    pub async fn remove(&self, key: &T) -> Option<V>
    where
        T: Eq + Hash,
    {
        self.items.write().await.remove(key).map(|item| item.object)
    }

    /// Clear the entire cache of all items regardless of expiry times.
    pub async fn clear(&self) {
        self.items.write().await.clear()
    }
}
