use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_runtime::{traits::Saturating, DispatchError};

pub struct RangeId(usize);

impl RangeId {
	pub const LP_TOKENS: RangeId = RangeId(0);
	pub const TOKENS: RangeId = RangeId(1);
	pub const FOREIGN_ASSETS: RangeId = RangeId(2);
}

pub struct MaxRanges;

impl Get<u32> for MaxRanges {
	fn get() -> u32 {
		const MAX: u32 = 256;
		// BoundedVec will panic if this variant isn't upheld.
		debug_assert!(isize::try_from(MAX).unwrap() < isize::MAX);
		MAX
	}
}

#[derive(Encode, Decode, Debug, TypeInfo, MaxEncodedLen)]
pub struct Ranges<AssetId> {
	ranges: BoundedVec<Range<AssetId>, MaxRanges>,
}

impl<AssetId> Ranges<AssetId>
where
	AssetId: From<u128> + Saturating + Ord + Clone,
{
    #[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		let mut ranges = Self { ranges: BoundedVec::default() };
		ranges.add(Range::lp_tokens()).unwrap();
		ranges.add(Range::tokens()).unwrap();
		ranges.add(Range::foreign_assets()).unwrap();
		ranges
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
		self.ranges.get(id.0)
	}

	pub fn lp_tokens(&self) -> &Range<AssetId> {
		self.get(RangeId::LP_TOKENS).unwrap()
	}

	pub fn tokens(&self) -> &Range<AssetId> {
		self.get(RangeId::TOKENS).unwrap()
	}

	pub fn foreign_assets(&self) -> &Range<AssetId> {
		self.get(RangeId::FOREIGN_ASSETS).unwrap()
	}

	pub fn increment(&mut self, id: RangeId) -> Result<AssetId, DispatchError> {
		let range = self.ranges.get_mut(id.0).ok_or(DispatchError::Other("range not found"))?;
		let next = range.increment()?;
		Ok(next)
	}

	pub fn increment_lp_tokens(&mut self) -> Result<AssetId, DispatchError> {
		self.increment(RangeId::LP_TOKENS)
	}

	pub fn increment_tokens(&mut self) -> Result<AssetId, DispatchError> {
		self.increment(RangeId::TOKENS)
	}

	pub fn increment_foreign_assets(&mut self) -> Result<AssetId, DispatchError> {
		self.increment(RangeId::FOREIGN_ASSETS)
	}

	pub fn end(&self) -> AssetId {
		if let Some(last) = self.ranges.last() {
			last.end()
		} else {
			AssetId::from(0)
		}
	}
}

#[derive(Encode, Decode, Debug, TypeInfo, MaxEncodedLen)]
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

	fn new(at: AssetId, end: Option<AssetId>) -> Result<Self, ()> {
		let end = if let Some(end) = end {
			if at.clone().saturating_add(end.clone()) < AssetId::from(100_000_000_u128) {
				return Err(())
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
		assert!(range.increment_lp_tokens().unwrap() == range.increment_lp_tokens().unwrap() - 1);
		assert!(range.increment_tokens().unwrap() == range.increment_tokens().unwrap() - 1);
		assert!(
			range.increment_foreign_assets().unwrap() ==
				range.increment_foreign_assets().unwrap() - 1
		);

		range
			.add(Range::new(0, None).unwrap())
			.expect_err("overlapping ranges/smaller ranges not allowed");

		let end = range.end();
		range.add(Range::new(end + 1, None).unwrap()).unwrap()
	}
}
