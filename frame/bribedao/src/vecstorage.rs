#![allow(dead_code)]

// Sorted Vec storage, import BribeStorage and ComplexMap

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sortedvec::sortedvec;
use std::convert::TryInto;

#[derive(Debug, Default, TypeInfo, PartialEq, Hash, Clone, Encode, Decode)]
pub struct BribesStorage {
	pub p_id: u32,
	pub amount: u32,
	pub votes: u32,
}

sortedvec! {
	/// lookup by (amount, votes) keys
	#[derive(Debug, Encode, Decode, TypeInfo)]
	pub struct FastMap {
		fn derive_key(val: &BribesStorage) -> (u32, u32) {
			(val.amount, val.votes)
		}
	}
}

impl FastMap {
	pub fn fastsearch(&self, key: u32) -> (u32, u32, u32) {
		let myinner = &self.inner;
		let out = myinner.binary_search_by_key(&key, |n| n.amount);
		(out.unwrap().try_into().unwrap(), 2, 3)
	} // binary search here;

	// make it easier to add things
	pub fn add(&mut self, amounts: u32, pid: u32, vots: u32) -> bool {
		self.insert(BribesStorage { p_id: pid, amount: amounts, votes: vots });
		true
	}

	pub fn new() -> FastMap {
		FastMap::default()
	}
}

/*
// Extend storage value | based on: https://github.com/totem-tech/totem-lego/blob/1759cc080ce413a9815bb07aa3658d9a07c9f3ef/frame/totem/utils/src/lib.rs

pub trait StorageValueExt<V>
where
	Self: StorageValue<V>,
//    K: FullEncode + Encode + EncodeLike,
	V: FullCodec + Decode + FullEncode + Encode + EncodeLike + WrapperTypeEncode,
{
	/// If the key exists in the map, modifies it with the provided function, and returns `Update::Done`.
	/// Otherwise, it does nothing and returns `Update::KeyNotFound`.
//    fn mutate_<KeyArg: EncodeLike<K>, F: FnOnce(&mut V)>(key: KeyArg, f: F) -> Update {
//        Self::mutate_exists(key, |option| match option.as_mut() {
//            Some(value) => {
//                f(value);
//                Update::Done
//            }
//            None => Update::KeyNotFound,
//        })
   // }
}

impl<T, V> StorageValueExt<V> for T
where
	T: StorageValue<V>,
//    K: FullEncode + Encode + EncodeLike,
	V: FullCodec + Decode + FullEncode + Encode + EncodeLike + WrapperTypeEncode,
{
}

*/
