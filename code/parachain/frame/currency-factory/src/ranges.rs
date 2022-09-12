use codec::{Decode, Encode, MaxEncodedLen};
pub use composable_traits::currency::RangeId;
use frame_support::{traits::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{CheckedAdd, Saturating},
	ArithmeticError, DispatchError,
};
pub struct MaxRanges;

impl Get<u32> for MaxRanges {
	fn get() -> u32 {
		const MAX: u32 = 256;
		// BoundedVec will panic if this variant isn't upheld.
		#[allow(clippy::disallowed_methods, clippy::unwrap_used)]
		{
			debug_assert!(isize::try_from(MAX).unwrap() < isize::MAX);
		}
		MAX
	}
}

/// Collection of `Range`s with functions for maintaining the collection.
#[derive(Encode, Decode, Debug, TypeInfo, MaxEncodedLen)]
pub struct Ranges<AssetId> {
	ranges: BoundedVec<Range<AssetId>, MaxRanges>,
}

impl<AssetId> Ranges<AssetId>
where
	AssetId: From<u128> + Saturating + Ord + Clone + CheckedAdd,
{
	fn bounds() -> u32 {
		MaxRanges::get()
	}

	/// Creates a new set of ranges with preconfigured ranges.
	///
	/// Preconfigured `RangeId`s can be found in `composable_traits::currency::RangeId`.
	///
	/// # Preconfigured Ranges by ID
	/// 0. LP Tokens
	/// 1. Tokens
	/// 2. Foreign Assets
	/// 3. IBC Assets
	/// 4. fNFTs
	/// 5. xTokens
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		let mut ranges = Self { ranges: BoundedVec::default() };

		// If `bounds` is greater than or equal to `n`, add pre-set ranges
		// Where `n` is the number of pre-set ranges
		#[allow(clippy::disallowed_methods)]
		if Self::bounds() >= 6 {
			ranges.add(Range::lp_tokens()).expect("capacity is sufficient, qed");
			ranges.add(Range::tokens()).expect("capacity is sufficient, qed");
			ranges.add(Range::foreign_assets()).expect("capacity is sufficient, qed");
			ranges.add(Range::ibc_assets()).expect("capacity is sufficient, qed");
			ranges.add(Range::fnfts()).expect("capacity is sufficient, qed");
			ranges.add(Range::x_tokens()).expect("capacity is sufficient, qed");
		}

		ranges
	}

	/// Appends a new `Range` of `length` to the of the current ranges.
	///
	/// # Errors
	/// * If appending `Range` would cause Overflow
	/// * If adding a `Range` would exceed the max number of ranges
	pub fn append(&mut self, length: u128) -> Result<(), DispatchError> {
		let start = self.end().checked_add(&AssetId::from(1)).ok_or(ArithmeticError::Overflow)?;
		let end = start.checked_add(&AssetId::from(length)).ok_or(ArithmeticError::Overflow)?;
		let range = Range::new(start, Some(end))?;

		self.add(range)?;

		Ok(())
	}

	/// Adds a new `Range` to current ranges.
	///
	/// # Errors
	/// * If the range overlaps with an existing `Range`
	/// * If adding a `Range` would exceed the max number of ranges
	pub fn add(&mut self, range: Range<AssetId>) -> Result<(), DispatchError> {
		if let Some(last) = self.ranges.last() {
			if last.end >= range.current {
				return Err(DispatchError::from("Range overlaps with existing an range!"))
			}
		}
		self.ranges
			.try_push(range)
			.map_err(|_| DispatchError::from("Exceeds max number of ranges!"))?;

		Ok(())
	}

	/// Gets a `Range` from the current ranges.
	///
	/// Returns `None` if a `Range` with the given `id` cannot be found.
	pub fn get(&self, id: RangeId) -> Option<&Range<AssetId>> {
		self.ranges.get(id.inner() as usize)
	}

	/// Increments the current current `AssetId` of a given range in the current ranges.
	///
	/// # Errors
	/// * If the `Range` is not found
	pub fn increment(&mut self, id: RangeId) -> Result<AssetId, DispatchError> {
		let range = self
			.ranges
			.get_mut(id.inner() as usize)
			.ok_or_else(|| DispatchError::from("Range not found!"))?;
		let next = range.increment()?;
		Ok(next)
	}

	/// Returns the last `Range` in the current ranges.
	///
	/// Returns `None` if no `Range` can be found.
	pub fn last(&self) -> Option<&Range<AssetId>> {
		self.ranges.last()
	}

	/// Returns the last reserved `AssetId` in the current ranges.
	///
	/// Returns `0` if no ranges are present.
	pub fn end(&self) -> AssetId {
		if let Some(last) = self.ranges.last() {
			last.end()
		} else {
			AssetId::from(0)
		}
	}
}

/// Range of `AssetId`s.
#[derive(Encode, Decode, Debug, TypeInfo, MaxEncodedLen, Clone, PartialEq, Eq)]
pub struct Range<AssetId> {
	current: AssetId,
	end: AssetId,
}

impl<AssetId> Range<AssetId>
where
	AssetId: From<u128> + Saturating + Ord + Clone,
{
	/// Returns the end `AssetId` of this `Range`.
	pub fn end(&self) -> AssetId {
		self.end.clone()
	}

	/// Returns the current `AssetId` of this `Range`.
	pub fn current(&self) -> AssetId {
		self.current.clone()
	}

	// Preset ranges
	// Ranges should be made in chunks of the size `u32::MAX`.
	// Meaning, range `n` will be from `u32::MAX * n + 1` to `u32::MAX * (n + 1)`.
	// The range `0` to `u32::MAX` is used for assets defined by the runtime.

	// REVIEW(connor): Range definitions could be simplified as a function of `n`. Should we
	// simplify the code, or avoid abstracting this away.
	// NOTE(connor): I plan on moving preconfigured ranges to the runtime config. More details to
	// come in RFC.

	/// Range for LP Tokens.
	fn lp_tokens() -> Self {
		Range {
			current: AssetId::from(
				(u32::MAX as u128)
					.checked_add(1)
					.expect("Range must be within u128 bounds; QED"),
			),
			end: AssetId::from(
				(u32::MAX as u128)
					.checked_mul(2)
					.expect("Range must be within u128 bounds; QED"),
			),
		}
	}

	/// Range for Tokens.
	fn tokens() -> Self {
		Range {
			current: AssetId::from(
				(u32::MAX as u128)
					.checked_mul(2)
					.and_then(|value| value.checked_add(1))
					.expect("Range must be within u128 bounds; QED"),
			),
			end: AssetId::from(
				(u32::MAX as u128)
					.checked_mul(3)
					.expect("Range must be within u128 bounds; QED"),
			),
		}
	}

	/// Range for foreign assets.
	fn foreign_assets() -> Self {
		Range {
			current: AssetId::from(
				(u32::MAX as u128)
					.checked_mul(3)
					.and_then(|value| value.checked_add(1))
					.expect("Range must be within u128 bounds; QED"),
			),
			end: AssetId::from(
				(u32::MAX as u128)
					.checked_mul(4)
					.expect("Range must be within u128 bounds; QED"),
			),
		}
	}

	/// Range for IBC assets.
	fn ibc_assets() -> Self {
		Range {
			current: AssetId::from(
				(u32::MAX as u128)
					.checked_mul(4)
					.and_then(|value| value.checked_add(1))
					.expect("Range must be within u128 bounds; QED"),
			),
			end: AssetId::from(
				(u32::MAX as u128)
					.checked_mul(5)
					.expect("Range must be within u128 bounds; QED"),
			),
		}
	}

	/// Range for fNFTs.
	fn fnfts() -> Self {
		Self {
			current: AssetId::from(
				(u32::MAX as u128)
					.checked_mul(5)
					.and_then(|value| value.checked_add(1))
					.expect("Range must be within u128 bounds; QED"),
			),
			end: AssetId::from(
				(u32::MAX as u128)
					.checked_mul(6)
					.expect("Range must be within u128 bounds; QED"),
			),
		}
	}

	/// Range for xTokens.
	/// xTokens are provided to stakers in exchange for staked token by the staking rewards pallet
	/// and may be used for governance.
	fn x_tokens() -> Self {
		Self {
			current: AssetId::from(
				(u32::MAX as u128)
					.checked_mul(6)
					.and_then(|value| value.checked_add(1))
					.expect("Range must be within u128 bounds; QED"),
			),
			end: AssetId::from(
				(u32::MAX as u128)
					.checked_mul(7)
					.expect("Range must be within u128 bounds; QED"),
			),
		}
	}

	/// Creates a new `Range`.
	fn new(at: AssetId, end: Option<AssetId>) -> Result<Self, DispatchError> {
		// TODO(connor): These AssetId restrictions and defaults don't make a lot of sense.
		let end = if let Some(end) = end {
			if at.clone().saturating_add(end.clone()) < AssetId::from(100_000_000_u128) {
				return Err(DispatchError::from("range does not have the minimum length"))
			}
			end
		} else {
			AssetId::from(100_000_000_000_u128).saturating_add(at.clone())
		};
		Ok(Range { current: at, end })
	}

	/// Increments the `current` `AssetId` of this `Range`.
	fn increment(&mut self) -> Result<AssetId, DispatchError> {
		if self.current == self.end {
			return Err(DispatchError::Other("range exhausted"))
		}

		let current = self.current.clone();
		self.current = current.clone().saturating_add(AssetId::from(1));
		Ok(current)
	}
}

#[cfg(test)]
mod tests {
	// TODO(connor): Split up these test
	use super::*;

	#[test]
	fn ranges() {
		let mut range = Ranges::<u128>::new();
		assert!(
			range.increment(RangeId::TOKENS).unwrap() ==
				range.increment(RangeId::TOKENS).unwrap() - 1
		);
		assert!(
			range.increment(RangeId::LP_TOKENS).unwrap() ==
				range.increment(RangeId::LP_TOKENS).unwrap() - 1
		);
		assert!(
			range.increment(RangeId::FOREIGN_ASSETS).unwrap() ==
				range.increment(RangeId::FOREIGN_ASSETS).unwrap() - 1
		);

		range
			.add(Range::new(0, None).unwrap())
			.expect_err("overlapping ranges/smaller ranges not allowed");

		let end = range.end();
		range.add(Range::new(end + 1, None).unwrap()).unwrap();

		range.append(u128::MAX).expect_err("should overflow");
		range.append(u128::MAX / 2).expect("should not overflow");
	}
}
