extern crate alloc;

use alloc::vec::Vec;
use arceos_api::imp::get_random_number;
use core::hash::{Hash, Hasher};

 #[derive(Clone)]
 pub struct Entry<K, V> {
    key: K,
    value: V,
    occupied: bool,
}


pub struct HashMap<K, V> {
    buckets: Vec<Option<Entry<K, V>>>,
    capacity: usize,
    size: usize,
    seed: u64,
}

struct NewHasher {
    state: u64,
}

impl NewHasher {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl Hasher for NewHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.rotate_left(5).wrapping_add(byte as u64);
            self.state ^= self.state >> 16;
            self.state = self.state.wrapping_mul(0x45d9f3b);
        }
    }

    fn finish(&self) -> u64 {
        let mut hash = self.state;
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xff51afd7ed558ccd);
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xc4ceb9fe1a85ec53);
        hash ^= hash >> 33;
        hash
    }
}

impl<K: Hash, V> HashMap<K, V> {
    fn hash_u64(&self, key: &K) -> usize {
        let mut hasher = NewHasher::new(self.seed);
        key.hash(&mut hasher);
        let hash = hasher.finish();
        (hash as usize) % self.capacity
    }

    pub fn new() -> Self {
        let seed = get_random_number() as u64;
        let capacity = 16;
        let mut buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(None);
        }
        Self { buckets, capacity, size: 0, seed}
    }
}

impl<K: Eq + Clone + Hash, V: Clone> HashMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) {
        if self.size * 2 >= self.capacity {
            self.resize();
        }

        let mut index = self.hash_u64(&key);
        let new_entry = Entry { key, value, occupied: true };

        for _ in 0..self.capacity {
            match &self.buckets[index] {
                Some(entry) if entry.key == new_entry.key => {
                    self.buckets[index] = Some(new_entry);
                    return;
                }
                None => {
                    self.buckets[index] = Some(new_entry);
                    self.size += 1;
                    return;
                }
                _ => {
                    index = (index + 1) % self.capacity;
                }
            }
        }
    }

    fn resize(&mut self) {
        let old_buckets = core::mem::take(&mut self.buckets);
        self.capacity *= 2;
        self.buckets = Vec::with_capacity(self.capacity);
        for _ in 0..self.capacity {
            self.buckets.push(None);
        }

        for entry_opt in old_buckets.into_iter() {
            if let Some(entry) = entry_opt {
                let mut index = self.hash_u64(&entry.key);
                while self.buckets[index].is_some() {
                    index = (index + 1) % self.capacity;
                }
                self.buckets[index] = Some(entry);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets.iter().filter_map(|entry_opt| {
            entry_opt.as_ref().map(|entry| (&entry.key, &entry.value))
        })
    }

}