#![allow(unused)]
use base64::{self, Engine};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
#[derive(Debug, Clone)]
pub struct Meta {
    id: String,
    created_at: u64,
    updated_at: u64,
}

#[derive(Debug, Clone)]
pub struct Human {
    name: String,
    age: u8,
}

#[derive(Debug, Clone)]
pub struct StateObjectHuman {
    meta: Meta,
    human: Human,
}

impl StateObjectHuman {
    pub fn new(name: &str, age: u8) -> Self {
        // let id = base64::encode(Uuid::new_v4().as_bytes());
        let id = base64::engine::general_purpose::STANDARD.encode(Uuid::new_v4().as_bytes());

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        StateObjectHuman {
            meta: Meta {
                id,
                created_at: now,
                updated_at: now,
            },
            human: Human {
                name: name.to_string(),
                age,
            },
        }
    }

    pub fn update(&mut self, name: Option<&str>, age: Option<u8>) {
        if let Some(name) = name {
            self.human.name = name.to_string();
        }
        if let Some(age) = age {
            self.human.age = age;
        }
        self.meta.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

type HumanStorageShard = Arc<RwLock<HashMap<String, StateObjectHuman>>>;
type HumanTreeShard = Arc<RwLock<BTreeMap<u64, String>>>;

#[derive(Clone)]
struct ShardedHumanStorage {
    storage_shards: Vec<HumanStorageShard>,
    tree_shards: Vec<HumanTreeShard>,
}

impl ShardedHumanStorage {
    pub fn new(shard_count: usize) -> Self {
        let mut storage_shards = Vec::with_capacity(shard_count);
        let mut tree_shards = Vec::with_capacity(shard_count);

        for _ in 0..shard_count {
            storage_shards.push(Arc::new(RwLock::new(HashMap::new())));
            tree_shards.push(Arc::new(RwLock::new(BTreeMap::new())));
        }

        ShardedHumanStorage {
            storage_shards,
            tree_shards,
        }
    }

    fn calculate_shard(&self, id: &str) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        id.hash(&mut hasher);
        let hash_value = hasher.finish();
        (hash_value % self.storage_shards.len() as u64) as usize
    }

    pub fn insert(&self, human: StateObjectHuman) {
        let id = human.meta.id.clone();
        let created_at = human.meta.created_at;

        let shard_index = self.calculate_shard(&id);
        let storage_shard = &self.storage_shards[shard_index];
        let tree_shard = &self.tree_shards[shard_index];

        {
            let mut storage_lock = storage_shard.write().unwrap();
            let mut tree_lock = tree_shard.write().unwrap();
            storage_lock.insert(id.clone(), human);
            tree_lock.insert(created_at, id);
        }
    }

    pub fn get_by_id(&self, id: &str) -> Option<StateObjectHuman> {
        let shard_index = self.calculate_shard(id);
        let storage_shard = &self.storage_shards[shard_index];
        let storage_lock = storage_shard.read().unwrap();
        storage_lock.get(id).cloned()
    }

    pub fn get_by_time_range(&self, start: u64, end: u64, limit: usize) -> Vec<StateObjectHuman> {
        let mut result = Vec::new();

        for shard_index in 0..self.storage_shards.len() {
            let tree_shard = &self.tree_shards[shard_index];
            let storage_shard = &self.storage_shards[shard_index];

            let tree_lock = tree_shard.read().unwrap();
            for (_time, id) in tree_lock.range(start..=end).take(limit) {
                if let Some(human) = storage_shard.read().unwrap().get(id) {
                    result.push(human.clone());
                    if result.len() == limit {
                        break;
                    }
                }
            }
            if result.len() == limit {
                break;
            }
        }

        result
    }

    pub fn update_by_id(&self, id: &str, name: Option<&str>, age: Option<u8>) -> bool {
        let shard_index = self.calculate_shard(id);
        let storage_shard = &self.storage_shards[shard_index];
        let mut storage_lock = storage_shard.write().unwrap();

        if let Some(human) = storage_lock.get_mut(id) {
            human.update(name, age);
            return true;
        }
        false
    }

    pub fn delete_by_id(&self, id: &str) -> bool {
        let shard_index = self.calculate_shard(id);
        let storage_shard = &self.storage_shards[shard_index];
        let tree_shard = &self.tree_shards[shard_index];

        let mut storage_lock = storage_shard.write().unwrap();
        let mut tree_lock = tree_shard.write().unwrap();

        if let Some(human) = storage_lock.remove(id) {
            tree_lock.remove(&human.meta.created_at);
            return true;
        }
        false
    }
}

fn main() {
    let shard_count = 4;
    let storage = ShardedHumanStorage::new(shard_count);

    let (sender, receiver): (Sender<StateObjectHuman>, Receiver<StateObjectHuman>) = unbounded();

    let _storage_clone = storage.clone();
    let handle = thread::spawn(move || {
        let human = StateObjectHuman::new("sarah", 30); // todo: add a better way to seed mock data
        sender.send(human).unwrap();

        let human2 = StateObjectHuman::new("cynthia", 25);
        sender.send(human2).unwrap();
    });

    handle.join().unwrap();

    for human in receiver {
        storage.insert(human);
    }

    // Read operations concurrently
    let storage_reader = storage.clone();
    let reader_thread = thread::spawn(move || {
        if let Some(human) = storage_reader.get_by_id("some-id") {
            println!("Found human: {:?}", human);
        }
    });

    reader_thread.join().unwrap();
}
