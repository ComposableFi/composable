use crate::{asset::Asset, feed::TimeStampedPrice};
use std::{
	collections::HashMap,
	hash::Hash,
	sync::{Arc, RwLock},
};

pub type PriceCache = HashMap<Asset, TimeStampedPrice>;

pub type ThreadSafePriceCache = Arc<RwLock<PriceCache>>;

pub trait Cache<K, V> {
	fn insert(&mut self, k: K, v: V);
	fn get(&self, k: &K) -> Option<V>;
}

impl<K: Eq + Hash, V: Copy> Cache<K, V> for HashMap<K, V> {
	fn insert(&mut self, k: K, v: V) {
		self.insert(k, v);
	}

	fn get(&self, k: &K) -> Option<V> {
		self.get(k).copied()
	}
}

impl<C: Cache<K, V>, K: Eq + Hash, V: Copy> Cache<K, V> for Arc<RwLock<C>> {
	fn insert(&mut self, k: K, v: V) {
		self.write().expect("could not acquire write lock").insert(k, v);
	}

	fn get(&self, k: &K) -> Option<V> {
		self.read().expect("could not acquire read lock").get(k)
	}
}
