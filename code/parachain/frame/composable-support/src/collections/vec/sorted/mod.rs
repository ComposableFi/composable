//! Sorted vectors.
//!
//! Cannibalized from [Repository](https://gitlab.com/spearman/sorted-vec) to be no_std and substrate compatible.
//!
//! - `SortedVec` -- sorted from least to greatest, may contain duplicates
//! - `SortedSet` -- sorted from least to greatest, unique elements
use codec::{Decode, Encode, EncodeLike, WrapperTypeEncode};
use core::hash::Hash;
use frame_support::RuntimeDebug;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sp_std::prelude::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", not(feature = "serde-nontransparent")), serde(transparent))]
#[derive(Clone, RuntimeDebug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct SortedVec<T: Ord> {
	#[cfg_attr(feature = "serde", serde(deserialize_with = "parse_vec"))]
	#[cfg_attr(feature = "serde", serde(bound(deserialize = "T : serde::Deserialize <'de>")))]
	vec: Vec<T>,
}

impl<T: Encode + Decode + Ord> EncodeLike<Vec<T>> for SortedVec<T> {}
impl<T: Ord> WrapperTypeEncode for SortedVec<T> {}

impl<T: Decode + Ord> Decode for SortedVec<T> {
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
		use is_sorted::IsSorted;

		let inner = Vec::<T>::decode(input)?;

		if !IsSorted::is_sorted(&mut inner.iter()) {
			Err("input is not sorted".into())
		} else {
			Ok(Self::unchecked_from(inner))
		}
	}

	fn skip<I: codec::Input>(input: &mut I) -> Result<(), codec::Error> {
		Vec::<T>::skip(input)
	}
}

#[cfg(feature = "serde")]
fn parse_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
	D: serde::Deserializer<'de>,
	T: Ord + serde::Deserialize<'de>,
{
	use is_sorted::IsSorted;
	use serde::de::Error;

	let v = Vec::deserialize(deserializer)?;
	if !IsSorted::is_sorted(&mut v.iter()) {
		Err(D::Error::custom("input sequence is not sorted"))
	} else {
		Ok(v)
	}
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", not(feature = "serde-nontransparent")), serde(transparent))]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct SortedSet<T: Ord> {
	set: SortedVec<T>,
}

impl<T: Ord> SortedVec<T> {
	#[inline]
	pub fn new() -> Self {
		SortedVec { vec: Vec::new() }
	}

	pub(crate) fn unchecked_from(vec: Vec<T>) -> Self {
		SortedVec { vec }
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		SortedVec { vec: Vec::with_capacity(capacity) }
	}

	/// Appends the item to the end of the vector. If that would unsort the vector, it returns the
	/// item as an Err.
	#[inline]
	pub fn append(&mut self, t: T) -> Result<usize, T> {
		if let Some(last) = self.last() {
			if last <= &t {
				let idx = self.append_unchecked(t);
				Ok(idx)
			} else {
				Err(t)
			}
		} else {
			let idx = self.insert(t);
			Ok(idx)
		}
	}

	fn append_unchecked(&mut self, t: T) -> usize {
		self.vec.push(t);
		self.len() - 1
	}

	/// Uses `sort_unstable()` to sort in place.
	#[inline]
	pub fn from_unsorted(mut vec: Vec<T>) -> Self {
		vec.sort_unstable();
		SortedVec { vec }
	}

	/// Insert an element into sorted position, returning the order index at which
	/// it was placed.
	#[inline]
	pub fn insert(&mut self, element: T) -> usize {
		let insert_at = match self.binary_search(&element) {
			Ok(insert_at) | Err(insert_at) => insert_at,
		};
		self.vec.insert(insert_at, element);
		insert_at
	}

	/// Find the element and return the index with `Ok`, otherwise insert the
	/// element and return the new element index with `Err`.
	#[inline]
	pub fn find_or_insert(&mut self, element: T) -> Result<usize, usize> {
		self.binary_search(&element).map_err(|insert_at| {
			self.vec.insert(insert_at, element);
			insert_at
		})
	}

	#[inline]
	pub fn remove_item(&mut self, item: &T) -> Option<T> {
		match self.vec.binary_search(item) {
			Ok(remove_at) => Some(self.vec.remove(remove_at)),
			Err(_) => None,
		}
	}

	/// Panics if index is out of bounds
	#[inline]
	pub fn remove(&mut self, index: usize) -> T {
		self.vec.remove(index)
	}

	#[inline]
	pub fn pop(&mut self) -> Option<T> {
		self.vec.pop()
	}

	#[inline]
	pub fn clear(&mut self) {
		self.vec.clear()
	}

	#[inline]
	pub fn dedup(&mut self) {
		self.vec.dedup();
	}

	#[inline]
	pub fn dedup_by_key<F, K>(&mut self, key: F)
	where
		F: FnMut(&mut T) -> K,
		K: PartialEq<K>,
	{
		self.vec.dedup_by_key(key);
	}

	#[inline]
	pub fn drain<R>(&mut self, range: R) -> sp_std::vec::Drain<T>
	where
		R: core::ops::RangeBounds<usize>,
	{
		self.vec.drain(range)
	}

	#[inline]
	pub fn retain<F>(&mut self, f: F)
	where
		F: FnMut(&T) -> bool,
	{
		self.vec.retain(f)
	}

	#[inline]
	pub fn into_inner(self) -> Vec<T> {
		self.vec
	}

	#[inline]
	pub fn as_inner(&self) -> &[T] {
		&self.vec
	}

	/// Apply a closure mutating the sorted vector and use `sort_unstable()`
	/// to re-sort the mutated vector
	#[inline]
	pub fn mutate<F, O>(&mut self, f: F) -> O
	where
		F: FnOnce(&mut Vec<T>) -> O,
	{
		let res = f(&mut self.vec);
		self.vec.sort_unstable();
		res
	}
}

impl<T: Ord> Default for SortedVec<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: Ord> From<Vec<T>> for SortedVec<T> {
	#[inline]
	fn from(unsorted: Vec<T>) -> Self {
		Self::from_unsorted(unsorted)
	}
}

impl<T: Ord> core::ops::Deref for SortedVec<T> {
	type Target = Vec<T>;

	#[inline]
	fn deref(&self) -> &Vec<T> {
		&self.vec
	}
}

impl<T: Ord> Extend<T> for SortedVec<T> {
	#[inline]
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		for t in iter {
			if let Err(t) = self.append(t) {
				self.insert(t);
			}
		}
	}
}

impl<T: Ord> SortedSet<T> {
	#[inline]
	pub fn new() -> Self {
		SortedSet { set: SortedVec::new() }
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		SortedSet { set: SortedVec::with_capacity(capacity) }
	}

	/// Uses `sort_unstable()` to sort in place and `dedup()` to remove
	/// duplicates.
	#[inline]
	pub fn from_unsorted(vec: Vec<T>) -> Self {
		let mut set = SortedVec::from_unsorted(vec);
		set.dedup();
		SortedSet { set }
	}

	/// Insert an element into sorted position, returning the order index at which
	/// it was placed.
	#[inline]
	pub fn insert(&mut self, element: T) -> usize {
		let _ = self.remove_item(&element);
		self.set.insert(element)
	}

	/// Find the element and return the index with `Ok`, otherwise insert the
	/// element and return the new element index with `Err`.
	#[inline]
	pub fn find_or_insert(&mut self, element: T) -> Result<usize, usize> {
		self.set.find_or_insert(element)
	}

	#[inline]
	pub fn remove_item(&mut self, item: &T) -> Option<T> {
		self.set.remove_item(item)
	}

	/// Panics if index is out of bounds
	#[inline]
	pub fn remove(&mut self, index: usize) -> T {
		self.set.remove(index)
	}

	#[inline]
	pub fn pop(&mut self) -> Option<T> {
		self.set.pop()
	}

	#[inline]
	pub fn clear(&mut self) {
		self.set.clear()
	}

	#[inline]
	pub fn drain<R>(&mut self, range: R) -> sp_std::vec::Drain<T>
	where
		R: core::ops::RangeBounds<usize>,
	{
		self.set.drain(range)
	}

	#[inline]
	pub fn retain<F>(&mut self, f: F)
	where
		F: FnMut(&T) -> bool,
	{
		self.set.retain(f)
	}

	#[inline]
	pub fn into_inner(self) -> Vec<T> {
		self.set.into_inner()
	}

	/// Apply a closure mutating the sorted vector and use `sort_unstable()`
	/// to re-sort the mutated vector and `dedup()` to remove any duplicate
	/// values
	#[inline]
	pub fn mutate<F, O>(&mut self, f: F) -> O
	where
		F: FnOnce(&mut Vec<T>) -> O,
	{
		let res = self.set.mutate(f);
		self.set.dedup();
		res
	}
}

impl<T: Ord> Default for SortedSet<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: Ord> From<Vec<T>> for SortedSet<T> {
	#[inline]
	fn from(unsorted: Vec<T>) -> Self {
		Self::from_unsorted(unsorted)
	}
}

impl<T: Ord> core::ops::Deref for SortedSet<T> {
	type Target = SortedVec<T>;

	#[inline]
	fn deref(&self) -> &SortedVec<T> {
		&self.set
	}
}

impl<T: Ord> Extend<T> for SortedSet<T> {
	#[inline]
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		for t in iter {
			let _ = self.insert(t);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sorted_vec() {
		let mut v = SortedVec::new();
		assert_eq!(v.insert(5), 0);
		assert_eq!(v.insert(3), 0);
		assert_eq!(v.insert(4), 1);
		assert_eq!(v.insert(4), 1);
		assert_eq!(v.find_or_insert(4), Ok(2));
		assert_eq!(v.len(), 4);
		v.dedup();
		assert_eq!(v.len(), 3);
		assert_eq!(v.binary_search(&3), Ok(0));
		assert_eq!(
			*SortedVec::from_unsorted(vec![5, -10, 99, -11, 2, 17, 10]),
			vec![-11, -10, 2, 5, 10, 17, 99]
		);
		assert_eq!(
			SortedVec::from_unsorted(vec![5, -10, 99, -11, 2, 17, 10]),
			vec![5, -10, 99, -11, 2, 17, 10].into()
		);
		let mut v = SortedVec::new();
		v.extend(vec![5, -10, 99, -11, 2, 17, 10].into_iter());
		assert_eq!(*v, vec![-11, -10, 2, 5, 10, 17, 99]);
		let _ = v.mutate(|v| {
			v[0] = 11;
			v[3] = 1;
		});
		assert_eq!(v.drain(..).collect::<Vec<i32>>(), vec![-10, 1, 2, 10, 11, 17, 99]);
	}

	#[test]
	fn test_sorted_set() {
		let mut s = SortedSet::new();
		assert_eq!(s.insert(5), 0);
		assert_eq!(s.insert(3), 0);
		assert_eq!(s.insert(4), 1);
		assert_eq!(s.insert(4), 1);
		assert_eq!(s.find_or_insert(4), Ok(1));
		assert_eq!(s.len(), 3);
		assert_eq!(s.binary_search(&3), Ok(0));
		assert_eq!(
			**SortedSet::from_unsorted(vec![5, -10, 99, -10, -11, 10, 2, 17, 10]),
			vec![-11, -10, 2, 5, 10, 17, 99]
		);
		assert_eq!(
			SortedSet::from_unsorted(vec![5, -10, 99, -10, -11, 10, 2, 17, 10]),
			vec![5, -10, 99, -10, -11, 10, 2, 17, 10].into()
		);
		let mut s = SortedSet::new();
		s.extend(vec![5, -11, -10, 99, -11, 2, 17, 2, 10].into_iter());
		assert_eq!(**s, vec![-11, -10, 2, 5, 10, 17, 99]);
		let _ = s.mutate(|s| {
			s[0] = 5;
			s[3] = 1;
		});
		assert_eq!(s.drain(..).collect::<Vec<i32>>(), vec![-10, 1, 2, 5, 10, 17, 99]);
	}

	#[cfg(feature = "serde-nontransparent")]
	#[test]
	fn test_deserialize() {
		let s = r#"{"vec":[-11,-10,2,5,10,17,99]}"#;
		let _ = serde_json::from_str::<SortedVec<i32>>(s).unwrap();
	}

	#[cfg(all(feature = "serde", not(feature = "serde-nontransparent")))]
	#[test]
	fn test_deserialize() {
		let s = "[-11,-10,2,5,10,17,99]";
		let _ = serde_json::from_str::<SortedVec<i32>>(s).unwrap();
	}

	#[cfg(feature = "serde-nontransparent")]
	#[test]
	#[should_panic]
	fn test_deserialize_unsorted() {
		let s = r#"{"vec":[99,-11,-10,2,5,10,17]}"#;
		let _ = serde_json::from_str::<SortedVec<i32>>(s).unwrap();
	}

	#[cfg(all(feature = "serde", not(feature = "serde-nontransparent")))]
	#[test]
	#[should_panic]
	fn test_deserialize_unsorted() {
		let s = "[99,-11,-10,2,5,10,17]";
		let _ = serde_json::from_str::<SortedVec<i32>>(s).unwrap();
	}

	#[test]
	fn unsorted_fail_to_decode() {
		let v: Vec<u32> = vec![1, 2, 5, 4];
		assert_eq!(
			SortedVec::<u32>::decode(&mut &v.encode()[..]),
			Err("input is not sorted".into()),
		);
	}
}
