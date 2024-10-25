use base64::encode;
use chrono::{DateTime, Utc};
use crossbeam::channel::{unbounded, Receiver, Sender};
use dashmap::DashMap;
use log::{info, warn};
use rayon::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

const DEFAULT_NUM_SHARDS: usize = 16;

/// A data structure to store and manage entities with unique IDs, supporting concurrent access and operations.
#[derive(Default)]
pub struct Store<T> {
    /// Sharded concurrent maps to store data based on timestamp.
    shards: Vec<Arc<DashMap<DateTime<Utc>, DashMap<String, T>>>>,
    /// Channel sender for background insertion processing.
    sender: Sender<StoreMessage<T>>,
}

/// Messages to control store operations
enum StoreMessage<T> {
    Insert(T, String), // Item and key param name for ID generation
}

impl<T> Store<T>
where
    T: Clone + Default + Send + Sync + 'static,
{
    /// Creates a new instance of `Store` with configurable sharding.
    ///
    /// # Arguments
    ///
    /// * `num_shards` - Optional number of shards for the store. Defaults to 16.
    pub fn new(num_shards: Option<usize>) -> Self {
        let (sender, receiver) = unbounded();
        let shards = (0..num_shards.unwrap_or(DEFAULT_NUM_SHARDS))
            .map(|_| Arc::new(DashMap::new()))
            .collect();

        let store = Self { shards, sender };
        store.spawn_worker(receiver);
        store
    }

    /// Spawns a background worker to handle insert messages.
    fn spawn_worker(&self, receiver: Receiver<StoreMessage<T>>) {
        let shards = self.shards.clone();
        std::thread::spawn(move || {
            for message in receiver {
                match message {
                    StoreMessage::Insert(item, param) => {
                        let shard_idx = item_hash(&item) % shards.len();
                        let shard = &shards[shard_idx];
                        let timestamp = Utc::now();
                        let uuid = Uuid::new_v10();
                        let encoded_uuid = encode(uuid.as_bytes());
                        let id = format!("{}:{}:{}", timestamp, encoded_uuid, param);

                        info!("Inserting item with ID: {}", &id);
                        shard.entry(timestamp).or_default().insert(id, item);
                    }
                }
            }
        });
    }

    /// Inserts a single item into the store, generating a unique ID.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to be inserted.
    /// * `param` - A parameter used in the unique ID generation.
    ///
    /// # Returns
    ///
    /// A string ID representing the newly inserted item.
    pub fn insert(&self, item: T, param: &str) -> String {
        let shard_idx = item_hash(&item) % self.shards.len();
        let shard = &self.shards[shard_idx];
        let timestamp = Utc::now();
        let uuid = Uuid::new_v10();
        let encoded_uuid = encode(uuid.as_bytes());
        let id = format!("{}:{}:{}", timestamp, encoded_uuid, param);

        info!("Inserting item with ID: {}", &id);
        shard.entry(timestamp).or_default().insert(id.clone(), item);

        id
    }

    /// Inserts multiple items into the store concurrently.
    ///
    /// # Arguments
    ///
    /// * `items` - A vector of items to be inserted.
    /// * `param` - A parameter used in the unique ID generation.
    pub fn insert_many(&self, items: Vec<T>, param: &str) {
        for item in items {
            if self
                .sender
                .send(StoreMessage::Insert(item, param.to_string()))
                .is_err()
            {
                warn!("Failed to enqueue item for background insertion.");
            }
        }
    }

    /// Retrieves an item from the store by its unique ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique ID of the item.
    ///
    /// # Returns
    ///
    /// An option containing the item if found, or `None` if not found.
    pub fn get(&self, id: &str) -> Option<T> {
        self.shards.par_iter().find_map_any(|shard| {
            shard
                .iter()
                .find_map(|map_entry| map_entry.value().get(id).cloned())
        })
    }

    /// Updates an existing item in the store by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique ID of the item to be updated.
    /// * `item` - The new item to replace the existing item.
    ///
    /// # Returns
    ///
    /// A boolean indicating if the update was successful.
    pub fn update(&self, id: &str, item: T) -> bool {
        self.shards.par_iter().any(|shard| {
            shard.iter_mut().any(|mut map_entry| {
                if let Some(existing_item) = map_entry.value_mut().get_mut(id) {
                    *existing_item = item.clone();
                    true
                } else {
                    false
                }
            })
        })
    }

    /// Deletes an item from the store by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique ID of the item to be deleted.
    ///
    /// # Returns
    ///
    /// A boolean indicating if the deletion was successful.
    pub fn delete(&self, id: &str) -> bool {
        self.shards.par_iter().any(|shard| {
            shard
                .iter_mut()
                .any(|mut map_entry| map_entry.value_mut().remove(id).is_some())
        })
    }

    /// Retrieves a paginated list of items.
    ///
    /// # Arguments
    ///
    /// * `page` - The page number (starting from 0).
    /// * `per_page` - Number of items per page.
    ///
    /// # Returns
    ///
    /// A vector containing the paginated items.
    pub fn paginate(&self, page: usize, per_page: usize) -> Vec<T> {
        let total_items = self
            .shards
            .par_iter()
            .flat_map(|shard| {
                shard.iter().flat_map(|map_entry| {
                    map_entry
                        .value()
                        .par_iter()
                        .map(|entry| entry.value().clone())
                })
            })
            .skip(page * per_page)
            .take(per_page)
            .collect::<Vec<_>>();

        total_items
    }
}

/// Computes a hash for sharding based on item pointer address.
///
/// # Arguments
///
/// * `item` - The item to hash.
///
/// # Returns
///
/// A usize hash value.
fn item_hash<T: Clone>(item: &T) -> usize {
    let ptr = item as *const _ as usize;
    ptr
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_single_insert() {
        let store = Store::<Human>::new(Some(8));
        let emma = Human {
            id: None,
            name: "Emma".to_string(),
            created_at: Utc::now(),
        };
        let id = store.insert(emma.clone(), "Emma");

        assert!(store.get(&id).is_some());
    }

    #[test]
    fn test_update_and_delete() {
        let store = Store::<Human>::new(Some(8));
        let emma = Human {
            id: None,
            name: "Emma".to_string(),
            created_at: Utc::now(),
        };
        let id = store.insert(emma.clone(), "Emma");

        assert!(store.update(&id, emma.clone()));
        assert!(store.delete(&id));
        assert!(store.get(&id).is_none());
    }

    #[test]
    fn test_pagination() {
        let store = Store::<Human>::new(Some(8));
        for i in 0..20 {
            store.insert(
                Human {
                    id: None,
                    name: format!("Human {}", i),
                    created_at: Utc::now(),
                },
                "Test",
            );
        }
        let page1 = store.paginate(0, 10);
        let page2 = store.paginate(1, 10);

        assert_eq!(page1.len(), 10);
        assert_eq!(page2.len(), 10);
    }
}
