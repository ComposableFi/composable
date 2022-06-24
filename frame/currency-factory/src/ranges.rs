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

	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		let mut ranges = Self { ranges: BoundedVec::default() };

		#[allow(clippy::disallowed_methods)]
		if Self::bounds() >= 4 {
			ranges.add(Range::lp_tokens()).expect("capacitiy is sufficient, qed");
			ranges.add(Range::tokens()).expect("capacitiy is sufficient, qed");
			ranges.add(Range::foreign_assets()).expect("capacitiy is sufficient, qed");
			ranges.add(Range::ibc_assets()).expect("capacitiy is sufficient, qed");
		}

		ranges
	}

	pub fn append(&mut self, length: u128) -> Result<(), DispatchError> {
		let start = self
			.end()
			.checked_add(&AssetId::from(1))
			.ok_or(DispatchError::Arithmetic(ArithmeticError::Overflow))?;
		let end = start
			.checked_add(&AssetId::from(length))
			.ok_or(DispatchError::Arithmetic(ArithmeticError::Overflow))?;
		let range = Range::new(start, Some(end))?;
		self.add(range)
			.map_err(|_| DispatchError::Arithmetic(ArithmeticError::Overflow))
	}

	pub fn add(&mut self, range: Range<AssetId>) -> Result<(), ()> {
		if let Some(last) = self.ranges.last() {
			if last.end >= range.current {
				return Err(())
			}
		}
		self.ranges.try_push(range)?;
		Ok(())
	}

	pub fn get(&self, id: RangeId) -> Option<&Range<AssetId>> {
		self.ranges.get(id.inner() as usize)
	}

	pub fn increment(&mut self, id: RangeId) -> Result<AssetId, DispatchError> {
		let range = self
			.ranges
			.get_mut(id.inner() as usize)
			.ok_or(DispatchError::Other("range not found"))?;
		let next = range.increment()?;
		Ok(next)
	}

	pub fn last(&self) -> Option<&Range<AssetId>> {
		self.ranges.last()
	}

	pub fn end(&self) -> AssetId {
		if let Some(last) = self.ranges.last() {
			last.end()
		} else {
			AssetId::from(0)
		}
	}
}

#[derive(Encode, Decode, Debug, TypeInfo, MaxEncodedLen, Clone, PartialEq)]
pub struct Range<AssetId> {
	current: AssetId,
	end: AssetId,
}

impl<AssetId> Range<AssetId>
where
	AssetId: From<u128> + Saturating + Ord + Clone,
{
	pub fn end(&self) -> AssetId {
		self.end.clone()
	}

	pub fn current(&self) -> AssetId {
		self.current.clone()
	}

	fn lp_tokens() -> Self {
		Range {
			current: AssetId::from(100_000_000_001_u128),
			end: AssetId::from(200_000_000_000_u128),
		}
	}

	fn tokens() -> Self {
		Range {
			current: AssetId::from(200_000_000_001_u128),
			end: AssetId::from(300_000_000_000_u128),
		}
	}

	fn foreign_assets() -> Self {
		Range {
			current: AssetId::from(300_000_000_001_u128),
			end: AssetId::from(400_000_000_000_u128),
		}
	}

	fn ibc_assets() -> Self {
		Range {
			current: AssetId::from(400_000_000_001_u128),
			end: AssetId::from(500_000_000_000_u128),
		}
	}

	fn new(at: AssetId, end: Option<AssetId>) -> Result<Self, DispatchError> {
		let end = if let Some(end) = end {
			if at.clone().saturating_add(end.clone()) < AssetId::from(100_000_000_u128) {
				return Err(DispatchError::Other("range does not have the minimum length"))
			}
			end
		} else {
			AssetId::from(100_000_000_000_u128).saturating_add(at.clone())
		};
		Ok(Range { current: at, end })
	}

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

		range.append(u128::MAX).expect_err("should overlfow");
		range.append(u128::MAX / 2).expect("should not overlfow");
	}
}
