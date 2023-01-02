//! https://github.com/ergoplatform/bounded-vec/issues/13
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::{
	convert::{TryFrom, TryInto},
	slice::{Iter, IterMut},
	vec::{self, Vec},
};

/// Non-empty Vec bounded with minimal (L - lower bound) and maximal (U - upper bound) items
/// quantity
#[derive(PartialEq, Eq, Debug, Clone, Hash, PartialOrd, Ord, Encode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize), serde(transparent))]
pub struct BiBoundedVec<T, const L: usize, const U: usize>
// enable when feature(const_evaluatable_checked) is stable
// where
//     Assert<{ L > 0 }>: IsTrue,
{
	inner: Vec<T>,
}

impl<T: Decode, const L: usize, const U: usize> Decode for BiBoundedVec<T, L, U> {
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
		let raw: Vec<_> = Vec::decode(input)?;
		BiBoundedVec::try_from(raw).map_err(|_err| codec::Error::from("Input out of bounds"))
	}
}

impl<T: Decode + MaxEncodedLen, const L: usize, const U: usize> MaxEncodedLen
	for BiBoundedVec<T, L, U>
{
	fn max_encoded_len() -> usize {
		U * T::max_encoded_len()
	}
}

/// BiBoundedVec errors
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BiBoundedVecOutOfBounds {
	/// Items quantity is less than L (lower bound)
	LowerBoundError {
		/// L (lower bound)
		lower_bound: usize,
		/// provided value
		got: usize,
	},
	/// Items quantity is more than U (upper bound)
	UpperBoundError {
		/// U (upper bound)
		upper_bound: usize,
		/// provided value
		got: usize,
	},
}

impl<T, const L: usize, const U: usize> BiBoundedVec<T, L, U> {
	/// Creates new BiBoundedVec or returns error if items count is out of bounds
	///
	/// # Example
	/// ```
	/// use composable_support::collections::vec::bounded::BiBoundedVec;
	/// let data: BiBoundedVec<_, 2, 8> = BiBoundedVec::from_vec(vec![1u8, 2]).unwrap();
	/// ```
	pub fn from_vec(items: Vec<T>) -> Result<Self, BiBoundedVecOutOfBounds> {
		// remove when feature(const_evaluatable_checked) is stable
		// and this requirement is encoded in type sig
		assert!(L > 0);
		let len = items.len();
		if len < L {
			Err(BiBoundedVecOutOfBounds::LowerBoundError { lower_bound: L, got: len })
		} else if len > U {
			Err(BiBoundedVecOutOfBounds::UpperBoundError { upper_bound: U, got: len })
		} else {
			Ok(BiBoundedVec { inner: items })
		}
	}

	/// Returns the inner (unbounded!) [`Vec<T>`].
	pub fn into_inner(self) -> Vec<T> {
		self.inner
	}

	/// Returns a reference to underlying `Vec`
	///
	/// # Example
	/// ```
	/// use composable_support::collections::vec::bounded::BiBoundedVec;
	/// use sp_std::convert::TryInto;
	///
	/// let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
	/// assert_eq!(data.as_vec(), &vec![1u8,2]);
	/// ```
	pub fn as_vec(&self) -> &Vec<T> {
		&self.inner
	}

	/// Returns the number of elements in the vector
	///
	/// # Example
	/// ```
	/// use composable_support::collections::vec::bounded::BiBoundedVec;
	/// use sp_std::convert::TryInto;
	///
	/// let data: BiBoundedVec<u8, 2, 4> = vec![1u8,2].try_into().unwrap();
	/// assert_eq!(data.len(), 2);
	/// ```
	pub fn len(&self) -> usize {
		self.inner.len()
	}

	/// Always returns `false` (cannot be empty)
	///
	/// # Example
	/// ```
	/// use composable_support::collections::vec::bounded::BiBoundedVec;
	/// use sp_std::convert::TryInto;
	///
	/// let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
	/// assert_eq!(data.is_empty(), false);
	/// ```
	pub fn is_empty(&self) -> bool {
		false
	}

	/// Extracts a slice containing the entire vector.
	///
	/// # Example
	/// ```
	/// use composable_support::collections::vec::bounded::BiBoundedVec;
	/// use sp_std::convert::TryInto;
	///
	/// let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
	/// assert_eq!(data.as_slice(), &[1u8,2]);
	/// ```
	pub fn as_slice(&self) -> &[T] {
		self.inner.as_slice()
	}

	/// Returns the first element of non-empty Vec
	///
	/// # Example
	/// ```
	/// use composable_support::collections::vec::bounded::BiBoundedVec;
	/// use sp_std::convert::TryInto;
	///
	/// let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
	/// assert_eq!(data.first(), Some(&1));
	/// ```
	pub fn first(&self) -> Option<&T> {
		// can make conditional depending on `const_evaluatable_checked` in nightly so that  in case
		// of at least 1 element, never option
		self.inner.first()
	}

	/// Returns the last element of non-empty Vec
	///
	/// # Example
	/// ```
	/// use composable_support::collections::vec::bounded::BiBoundedVec;
	/// use sp_std::convert::TryInto;
	///
	/// let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
	/// assert_eq!(data.last(), Some(&2));
	/// ```
	pub fn last(&self) -> Option<&T> {
		// can make conditional depending on `const_evaluatable_checked` in nightly so that  in case
		// of at least 1 element, never option
		self.inner.last()
	}

	/// Create a new `BiBoundedVec` by consuming `self` and mapping each element.
	///
	/// This is useful as it keeps the knowledge that the length is >= U, <= L,
	/// even through the old `BiBoundedVec` is consumed and turned into an iterator.
	///
	/// # Example
	///
	/// ```
	/// use composable_support::collections::vec::bounded::BiBoundedVec;
	/// let data: BiBoundedVec<u8, 2, 8> = [1u8,2].into();
	/// let data = data.mapped(|x|x*2);
	/// assert_eq!(data, [2u8,4].into());
	/// ```
	pub fn mapped<F, N>(self, map_fn: F) -> BiBoundedVec<N, L, U>
	where
		F: FnMut(T) -> N,
	{
		BiBoundedVec { inner: self.inner.into_iter().map(map_fn).collect::<Vec<_>>() }
	}

	/// Create a new `BiBoundedVec` by mapping references to the elements of self
	///
	/// This is useful as it keeps the knowledge that the length is >= U, <= L,
	/// will still hold for new `BiBoundedVec`
	///
	/// # Example
	///
	/// ```
	/// use composable_support::collections::vec::bounded::BiBoundedVec;
	/// let data: BiBoundedVec<u8, 2, 8> = [1u8,2].into();
	/// let data = data.mapped_ref(|x|x*2);
	/// assert_eq!(data, [2u8,4].into());
	/// ```
	pub fn mapped_ref<F, N>(&self, map_fn: F) -> BiBoundedVec<N, L, U>
	where
		F: FnMut(&T) -> N,
	{
		BiBoundedVec { inner: self.inner.iter().map(map_fn).collect::<Vec<_>>() }
	}

	/// Create a new `BiBoundedVec` by consuming `self` and mapping each element
	/// to a `Result`.
	///
	/// This is useful as it keeps the knowledge that the length is preserved
	/// even through the old `BiBoundedVec` is consumed and turned into an iterator.
	///
	/// As this method consumes self, returning an error means that this
	/// vec is dropped. I.e. this method behaves roughly like using a
	/// chain of `into_iter()`, `map`, `collect::<Result<Vec<N>,E>>` and
	/// then converting the `Vec` back to a `Vec1`.
	///
	///
	/// # Errors
	///
	/// Once any call to `map_fn` returns a error that error is directly
	/// returned by this method.
	///
	/// # Example
	///
	/// ```
	/// use composable_support::collections::vec::bounded::BiBoundedVec;
	/// let data: BiBoundedVec<u8, 2, 8> = [1u8,2].into();
	/// let data: Result<BiBoundedVec<u8, 2, 8>, _> = data.try_mapped(|x| Err("failed"));
	/// assert_eq!(data, Err("failed"));
	/// ```
	pub fn try_mapped<F, N, E>(self, map_fn: F) -> Result<BiBoundedVec<N, L, U>, E>
	where
		F: FnMut(T) -> Result<N, E>,
	{
		let mut map_fn = map_fn;
		let mut out = Vec::with_capacity(self.len());
		for element in self.inner.into_iter() {
			out.push(map_fn(element)?);
		}

		Ok(BiBoundedVec::from_vec(out)
			.expect("prove: was created with exact known amount of elements"))
	}

	/// Create a new `BiBoundedVec` by mapping references of `self` elements
	/// to a `Result`.
	///
	/// This is useful as it keeps the knowledge that the length is preserved
	/// even through the old `BiBoundedVec` is consumed and turned into an iterator.
	///
	/// # Errors
	///
	/// Once any call to `map_fn` returns a error that error is directly
	/// returned by this method.
	///
	/// # Example
	///
	/// ```
	/// use composable_support::collections::vec::bounded::BiBoundedVec;
	/// let data: BiBoundedVec<u8, 2, 8> = [1u8,2].into();
	/// let data: Result<BiBoundedVec<u8, 2, 8>, _> = data.try_mapped_ref(|x| Err("failed"));
	/// assert_eq!(data, Err("failed"));
	/// ```
	pub fn try_mapped_ref<F, N, E>(&self, map_fn: F) -> Result<BiBoundedVec<N, L, U>, E>
	where
		F: FnMut(&T) -> Result<N, E>,
	{
		let mut map_fn = map_fn;
		let mut out = Vec::with_capacity(self.len());
		for element in self.inner.iter() {
			out.push(map_fn(element)?);
		}

		Ok(BiBoundedVec::from_vec(out)
			.expect("prove: was created with exact known amount of elements"))
	}

	/// Returns a reference for an element at index or `None` if out of bounds
	///
	/// # Example
	///
	/// ```
	/// use composable_support::collections::vec::bounded::BiBoundedVec;
	/// let data: BiBoundedVec<u8, 2, 8> = [1u8,2].into();
	/// let elem = *data.get(1).unwrap();
	/// assert_eq!(elem, 2);
	/// ```
	pub fn get(&self, index: usize) -> Option<&T> {
		self.inner.get(index)
	}

	/// Returns an iterator
	pub fn iter(&self) -> Iter<T> {
		self.inner.iter()
	}

	/// Returns an iterator that allows to modify each value
	pub fn iter_mut(&mut self) -> IterMut<T> {
		self.inner.iter_mut()
	}

	/// Returns the last and all the rest of the elements
	pub fn split_last(&self) -> Option<(&T, &[T])> {
		self.inner.split_last()
	}

	/// Return a new BiBoundedVec with indices included
	pub fn enumerated(self) -> BiBoundedVec<(usize, T), L, U> {
		self.inner
			.into_iter()
			.enumerate()
			.collect::<Vec<(usize, T)>>()
			.try_into()
			.expect("prove: was created with exact known amount of elements")
	}
}

/// A non-empty Vec with no effective upper-bound on its length
pub type NonEmptyVec<T> = BiBoundedVec<T, 1, { usize::MAX }>;

impl<T, const L: usize, const U: usize> TryFrom<Vec<T>> for BiBoundedVec<T, L, U> {
	type Error = BiBoundedVecOutOfBounds;

	fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
		BiBoundedVec::from_vec(value)
	}
}

// when feature(const_evaluatable_checked) is stable cover all array sizes (L..=U)
impl<T, const L: usize, const U: usize> From<[T; L]> for BiBoundedVec<T, L, U> {
	fn from(arr: [T; L]) -> Self {
		BiBoundedVec { inner: arr.into() }
	}
}

impl<T, const L: usize, const U: usize> From<BiBoundedVec<T, L, U>> for Vec<T> {
	fn from(v: BiBoundedVec<T, L, U>) -> Self {
		v.inner
	}
}

impl<T, const L: usize, const U: usize> IntoIterator for BiBoundedVec<T, L, U> {
	type Item = T;
	type IntoIter = vec::IntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		self.inner.into_iter()
	}
}

impl<'a, T, const L: usize, const U: usize> IntoIterator for &'a BiBoundedVec<T, L, U> {
	type Item = &'a T;
	type IntoIter = core::slice::Iter<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.inner.iter()
	}
}

impl<'a, T, const L: usize, const U: usize> IntoIterator for &'a mut BiBoundedVec<T, L, U> {
	type Item = &'a mut T;
	type IntoIter = core::slice::IterMut<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.inner.iter_mut()
	}
}

impl<T, const L: usize, const U: usize> AsRef<Vec<T>> for BiBoundedVec<T, L, U> {
	fn as_ref(&self) -> &Vec<T> {
		&self.inner
	}
}

impl<T, const L: usize, const U: usize> AsRef<[T]> for BiBoundedVec<T, L, U> {
	fn as_ref(&self) -> &[T] {
		self.inner.as_ref()
	}
}

impl<T, const L: usize, const U: usize> AsMut<Vec<T>> for BiBoundedVec<T, L, U> {
	fn as_mut(&mut self) -> &mut Vec<T> {
		self.inner.as_mut()
	}
}

impl<T, const L: usize, const U: usize> AsMut<[T]> for BiBoundedVec<T, L, U> {
	fn as_mut(&mut self) -> &mut [T] {
		self.inner.as_mut()
	}
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
	use sp_std::convert::TryInto;

	use super::*;

	#[test]
	fn from_vec() {
		assert!(BiBoundedVec::<u8, 2, 8>::from_vec(vec![1, 2]).is_ok());
		assert!(BiBoundedVec::<u8, 2, 8>::from_vec(vec![]).is_err());
		assert!(BiBoundedVec::<u8, 3, 8>::from_vec(vec![1, 2]).is_err());
		assert!(BiBoundedVec::<u8, 1, 2>::from_vec(vec![1, 2, 3]).is_err());
	}

	#[test]
	fn is_empty() {
		let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
		assert!(!data.is_empty());
	}

	#[test]
	fn as_vec() {
		let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
		assert_eq!(data.as_vec(), &vec![1u8, 2]);
	}

	#[test]
	fn as_slice() {
		let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
		assert_eq!(data.as_slice(), &[1u8, 2]);
	}

	#[test]
	fn len() {
		let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
		assert_eq!(data.len(), 2);
	}

	#[test]
	fn first() {
		let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
		assert_eq!(data.first(), Some(&1u8));
	}

	#[test]
	fn last() {
		let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
		assert_eq!(data.last(), Some(&2u8));
	}

	#[test]
	fn mapped() {
		let data: BiBoundedVec<u8, 2, 8> = [1u8, 2].into();
		let data = data.mapped(|x| x * 2);
		assert_eq!(data, [2u8, 4].into());
	}

	#[test]
	fn mapped_ref() {
		let data: BiBoundedVec<u8, 2, 8> = [1u8, 2].into();
		let data = data.mapped_ref(|x| x * 2);
		assert_eq!(data, [2u8, 4].into());
	}

	#[test]
	fn get() {
		let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
		assert_eq!(data.get(1).unwrap(), &2u8);
		assert!(data.get(3).is_none());
	}

	#[test]
	fn try_mapped() {
		let data: BiBoundedVec<u8, 2, 8> = [1u8, 2].into();
		let data = data.try_mapped(|x| 100u8.checked_div(x).ok_or("error"));
		assert_eq!(data, Ok([100u8, 50].into()));
	}

	#[test]
	fn try_mapped_error() {
		let data: BiBoundedVec<u8, 2, 8> = [0u8, 2].into();
		let data = data.try_mapped(|x| 100u8.checked_div(x).ok_or("error"));
		assert_eq!(data, Err("error"));
	}

	#[test]
	fn try_mapped_ref() {
		let data: BiBoundedVec<u8, 2, 8> = [1u8, 2].into();
		let data = data.try_mapped_ref(|x| 100u8.checked_div(*x).ok_or("error"));
		assert_eq!(data, Ok([100u8, 50].into()));
	}

	#[test]
	fn try_mapped_ref_error() {
		let data: BiBoundedVec<u8, 2, 8> = [0u8, 2].into();
		let data = data.try_mapped_ref(|x| 100u8.checked_div(*x).ok_or("error"));
		assert_eq!(data, Err("error"));
	}

	#[test]
	fn split_last() {
		let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
		assert_eq!(data.split_last().unwrap(), (&2u8, [1u8].as_ref()));
		let data1: BiBoundedVec<_, 1, 8> = vec![1u8].try_into().unwrap();
		assert_eq!(data1.split_last().unwrap(), (&1u8, Vec::new().as_ref()));
	}

	#[test]
	fn enumerated() {
		let data: BiBoundedVec<_, 2, 8> = vec![1u8, 2].try_into().unwrap();
		let expected: BiBoundedVec<_, 2, 8> = vec![(0, 1u8), (1, 2)].try_into().unwrap();
		assert_eq!(data.enumerated(), expected);
	}

	#[test]
	fn into_iter() {
		let mut vec = vec![1u8, 2];
		let mut data: BiBoundedVec<_, 2, 8> = vec.clone().try_into().unwrap();
		assert_eq!(data.clone().into_iter().collect::<Vec<u8>>(), vec);
		assert_eq!(data.iter().collect::<Vec<&u8>>(), vec.iter().collect::<Vec<&u8>>());
		assert_eq!(
			data.iter_mut().collect::<Vec<&mut u8>>(),
			vec.iter_mut().collect::<Vec<&mut u8>>()
		);
	}

	#[test]
	fn scale() {
		let input: BiBoundedVec<u8, 2, 8> = vec![1u8, 2].try_into().unwrap();
		let output = BiBoundedVec::<u8, 2, 8>::decode(&mut &input.encode()[..]).unwrap();
		assert_eq!(output, input);
	}
}
