use crate::collections::vec::sorted::SortedVec;
use codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use core::{
	ops::{Deref, Index},
	slice::SliceIndex,
};
use frame_support::{traits::Get, BoundedVec};
use sp_std::{convert::TryFrom, marker::PhantomData, prelude::*};

/// A bounded, sorted vector.
///
/// It has implementations for efficient append and length decoding, as with a normal `Vec<_>`, once
/// put into storage as a raw value, map or double-map.
///
/// As the name suggests, the length of the queue is always bounded and sorted. All internal
/// operations ensure this bound is respected and order is maintained.
#[derive(Encode)]
pub struct BoundedSortedVec<T: Ord, S>(SortedVec<T>, PhantomData<S>);

impl<T: Decode + Ord, S: Get<u32>> Decode for BoundedSortedVec<T, S> {
	#[inline]
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
		use is_sorted::IsSorted;

		let inner = Vec::<T>::decode(input)?;
		if inner.len() > S::get() as usize {
			return Err("BoundedSortedVec exceeds its limit".into())
		}

		if !IsSorted::is_sorted(&mut inner.iter()) {
			return Err("input is not sorted".into())
		}
		// checked above
		Ok(Self(SortedVec::unchecked_from(inner), PhantomData))
	}

	#[inline]
	fn skip<I: codec::Input>(input: &mut I) -> Result<(), codec::Error> {
		Vec::<T>::skip(input)
	}
}

// `BoundedSortedVec`s encode to something which will always decode as a `Vec`.
impl<T: Encode + Decode + Ord, S: Get<u32>> EncodeLike<Vec<T>> for BoundedSortedVec<T, S> {}

impl<T: Ord, S> BoundedSortedVec<T, S> {
	/// Create `Self` from `t` without any checks.
	fn unchecked_from(t: Vec<T>) -> Self {
		Self(SortedVec::unchecked_from(t), Default::default())
	}

	/// Consume self, and return the inner `Vec`. Henceforth, the `Vec<_>` can be altered in an
	/// arbitrary way. At some point, if the reverse conversion is required, `TryFrom<Vec<_>>` can
	/// be used.
	///
	/// This is useful for cases if you need access to an internal API of the inner `Vec<_>` which
	/// is not provided by the wrapper `BoundedVec`.
	#[inline]
	pub fn into_inner(self) -> SortedVec<T> {
		self.0
	}

	/// Exactly the same semantics as [`Vec::remove`].
	///
	/// # Panics
	///
	/// Panics if `index` is out of bounds.
	#[inline]
	pub fn remove(&mut self, index: usize) -> T {
		self.0.remove(index)
	}

	/// Exactly the same semantics as [`Vec::retain`].
	#[inline]
	pub fn retain<F: FnMut(&T) -> bool>(&mut self, f: F) {
		self.0.retain(f)
	}
}

impl<T: Ord, S: Get<u32>> From<BoundedSortedVec<T, S>> for Vec<T> {
	#[inline]
	fn from(x: BoundedSortedVec<T, S>) -> Vec<T> {
		x.0.into_inner()
	}
}

impl<T: Ord, S: Get<u32>> TryFrom<BoundedSortedVec<T, S>> for BoundedVec<T, S> {
	type Error = ();

	#[inline]
	fn try_from(x: BoundedSortedVec<T, S>) -> Result<Self, Self::Error> {
		BoundedVec::try_from(x.0.into_inner())
	}
}

impl<T: Ord, S: Get<u32>> BoundedSortedVec<T, S> {
	/// Get the bound of the type in `usize`.
	#[inline]
	pub fn bound() -> usize {
		S::get() as usize
	}

	/// Exactly the same semantics as [`Vec::insert`], but returns an `Err` (and is a noop) if the
	/// new length of the vector exceeds `S`.
	#[inline]
	pub fn try_insert(&mut self, element: T) -> Result<(), T> {
		if self.len() < Self::bound() {
			self.0.insert(element);
			Ok(())
		} else {
			Err(element)
		}
	}

	/// Exactly the same semantics as [`Vec::push`], but returns an `Err` (and is a noop) if the
	/// new length of the vector exceeds `S`.
	///
	/// # Panics
	///
	/// Panics if the new capacity exceeds isize::MAX bytes.
	#[inline]
	pub fn try_push(&mut self, element: T) -> Result<(), T> {
		if self.len() < Self::bound() {
			self.0.insert(element);
			Ok(())
		} else {
			Err(element)
		}
	}
}

impl<T: Ord, S> Default for BoundedSortedVec<T, S> {
	fn default() -> Self {
		// the bound cannot be below 0, which is satisfied by an empty vector
		Self::unchecked_from(Vec::default())
	}
}

#[cfg(feature = "std")]
impl<T: Ord, S> sp_std::fmt::Debug for BoundedSortedVec<T, S>
where
	T: sp_std::fmt::Debug,
	S: Get<u32>,
{
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		f.debug_tuple("BoundedVec").field(&self.0).field(&Self::bound()).finish()
	}
}

impl<T: Ord, S> Clone for BoundedSortedVec<T, S>
where
	T: Clone,
{
	#[inline]
	fn clone(&self) -> Self {
		// bound is retained
		Self::unchecked_from(self.0.as_inner().to_vec())
	}
}

impl<T: Ord, S: Get<u32>> TryFrom<Vec<T>> for BoundedSortedVec<T, S> {
	type Error = ();

	#[inline]
	fn try_from(t: Vec<T>) -> Result<Self, Self::Error> {
		if t.len() <= Self::bound() {
			// explicit check just above
			Ok(Self::unchecked_from(t))
		} else {
			Err(())
		}
	}
}

// It is okay to give a non-mutable reference of the inner vec to anyone.
impl<T: Ord, S> AsRef<Vec<T>> for BoundedSortedVec<T, S> {
	#[inline]
	fn as_ref(&self) -> &Vec<T> {
		&self.0
	}
}

impl<T: Ord, S> AsRef<[T]> for BoundedSortedVec<T, S> {
	#[inline]
	fn as_ref(&self) -> &[T] {
		&self.0
	}
}

// will allow for immutable all operations of `Vec<T>` on `BoundedVec<T>`.
impl<T: Ord, S> Deref for BoundedSortedVec<T, S> {
	type Target = Vec<T>;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

// Allows for indexing similar to a normal `Vec`. Can panic if out of bound.
impl<T: Ord, S, I> Index<I> for BoundedSortedVec<T, S>
where
	I: SliceIndex<[T]>,
{
	type Output = I::Output;

	#[inline]
	fn index(&self, index: I) -> &Self::Output {
		self.0.index(index)
	}
}

impl<T: Ord, S> sp_std::iter::IntoIterator for BoundedSortedVec<T, S> {
	type Item = T;
	type IntoIter = sp_std::vec::IntoIter<T>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_inner().into_iter()
	}
}

impl<T: Ord, S> codec::DecodeLength for BoundedSortedVec<T, S> {
	#[inline]
	fn len(self_encoded: &[u8]) -> Result<usize, codec::Error> {
		// `BoundedSortedVec<T, _>` stored just a `Vec<T>`, thus the length is at the beginning in
		// `Compact` form, and same implementation as `Vec<T>` can be used.
		<Vec<T> as codec::DecodeLength>::len(self_encoded)
	}
}

/// Allows for comparing vectors with different bounds.
impl<T, S1: Get<u32>, S2: Get<u32>> PartialEq<BoundedSortedVec<T, S2>> for BoundedSortedVec<T, S1>
where
	T: PartialEq + Ord,
{
	#[inline]
	fn eq(&self, rhs: &BoundedSortedVec<T, S2>) -> bool {
		self.0 == rhs.0
	}
}

impl<T: PartialEq + Ord, S: Get<u32>> PartialEq<Vec<T>> for BoundedSortedVec<T, S> {
	#[inline]
	fn eq(&self, other: &Vec<T>) -> bool {
		self.0.as_inner() == other
	}
}

impl<T: PartialEq + Ord, S: Get<u32>> Eq for BoundedSortedVec<T, S> where T: Eq {}

impl<T: Ord, S> MaxEncodedLen for BoundedSortedVec<T, S>
where
	T: MaxEncodedLen,
	S: Get<u32>,
	BoundedSortedVec<T, S>: Encode,
{
	#[inline]
	fn max_encoded_len() -> usize {
		// BoundedSortedVec<T, S> encodes like Vec<T> which encodes like [T], which is a compact u32
		// plus each item in the slice:
		// https://substrate.dev/rustdocs/v3.0.0/src/parity_scale_codec/codec.rs.html#798-808
		codec::Compact(S::get())
			.encoded_size()
			.saturating_add(Self::bound().saturating_mul(T::max_encoded_len()))
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	use frame_support::sp_io::TestExternalities;
	use sp_std::convert::TryInto;

	frame_support::parameter_types! {
		pub const Seven: u32 = 7;
		pub const Four: u32 = 4;
	}

	#[frame_support::storage_alias]
	type Foo = StorageValue<Prefix, BoundedSortedVec<u32, Seven>>;

	#[test]
	fn store_works() {
		TestExternalities::default().execute_with(|| {
			let bounded: BoundedSortedVec<u32, Seven> = vec![1, 2, 3].try_into().unwrap();
			Foo::put(bounded.clone());
			assert_eq!(Foo::get().unwrap(), bounded);
		});
	}

	#[test]
	fn try_append_is_correct() {
		assert_eq!(BoundedSortedVec::<u32, Seven>::bound(), 7);
	}

	#[test]
	fn try_insert_works() {
		let mut bounded: BoundedSortedVec<u32, Four> = vec![1, 2, 3].try_into().unwrap();
		bounded.try_insert(1).unwrap();
		assert_eq!(*bounded, vec![1, 1, 2, 3]);

		assert!(bounded.try_insert(0).is_err());
		assert_eq!(*bounded, vec![1, 1, 2, 3]);
	}

	#[test]
	#[should_panic]
	fn try_insert_panics_if_oob() {
		let mut bounded: BoundedSortedVec<u32, Four> = vec![1, 2, 3, 4].try_into().unwrap();
		bounded.try_insert(9).unwrap();
	}

	#[test]
	fn try_push_works() {
		let mut bounded: BoundedSortedVec<u32, Four> = vec![1, 2, 3].try_into().unwrap();
		bounded.try_push(0).unwrap();
		assert_eq!(*bounded, vec![0, 1, 2, 3]);

		assert!(bounded.try_push(9).is_err());
	}

	#[test]
	fn deref_coercion_works() {
		let bounded: BoundedSortedVec<u32, Seven> = vec![1, 2, 3].try_into().unwrap();
		// these methods come from deref-ed vec.
		assert_eq!(bounded.len(), 3);
		assert!(bounded.iter().next().is_some());
		assert!(!bounded.is_empty());
	}

	#[test]
	fn slice_indexing_works() {
		let bounded: BoundedSortedVec<u32, Seven> = vec![1, 2, 3, 4, 5, 6].try_into().unwrap();
		assert_eq!(&bounded[0..=2], &[1, 2, 3]);
	}

	#[test]
	fn vec_eq_works() {
		let bounded: BoundedSortedVec<u32, Seven> = vec![1, 2, 3, 4, 5, 6].try_into().unwrap();
		assert_eq!(bounded, vec![1, 2, 3, 4, 5, 6]);
	}

	#[test]
	fn too_big_vec_fail_to_decode() {
		let v: Vec<u32> = vec![1, 2, 3, 4, 5];
		assert_eq!(
			BoundedSortedVec::<u32, Four>::decode(&mut &v.encode()[..]),
			Err("BoundedSortedVec exceeds its limit".into()),
		);
	}

	#[test]
	fn unsorted_fail_to_decode() {
		let v: Vec<u32> = vec![1, 2, 5, 4];
		assert_eq!(
			BoundedSortedVec::<u32, Four>::decode(&mut &v.encode()[..]),
			Err("input is not sorted".into()),
		);
	}
}
