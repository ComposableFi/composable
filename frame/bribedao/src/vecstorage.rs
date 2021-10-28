// Sorted Vec storage, import BribeStorage and ComplexMap

use sortedvec::sortedvec;
use std::convert::TryInto;

#[derive(Debug, Default, PartialEq, Hash, Clone)]
pub struct BribesStorage {
	p_id: u32,
	amount: u32,
	votes: u32,
}

sortedvec! {
	/// lookup by (amount, votes) keys
	#[derive(Debug)]
	pub struct ComplexMap {
		fn derive_key(val: &BribesStorage) -> (u32, u32) {
			(val.amount, val.votes)
		}
	}
}

impl ComplexMap {
	fn fastsearch(&self, key: u32) -> (u32, u32, u32) {
		let myinner = &self.inner;
		let out = myinner.binary_search_by_key(&key, |n| n.amount);
		(out.unwrap().try_into().unwrap(), 2, 3)
	} // binary search here;


}

pub fn new() -> ComplexMap {
	ComplexMap::default()
}
