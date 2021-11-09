#![allow(dead_code)]

// Sorted Vec storage, import BribeStorage and ComplexMap

use sortedvec::sortedvec;
use std::convert::TryInto;

#[derive(Debug, Default, PartialEq, Hash, Clone)]
pub struct BribesStorage {
	pub p_id: u32,
	pub amount: u32,
	pub votes: u32,
}

sortedvec! {
	/// lookup by (amount, votes) keys
	#[derive(Debug)]
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



