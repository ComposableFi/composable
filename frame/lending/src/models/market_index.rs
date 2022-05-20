use frame_support::pallet_prelude::*;

pub type MarketId = u32;

#[rustfmt::skip]
#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq, MaxEncodedLen, TypeInfo)]
#[repr(transparent)]
pub struct MarketIndex(
	// to allow pattern matching in tests outside of this crate
	#[cfg(test)] pub MarketId,

	#[cfg(not(test))] pub(crate) MarketId,
);

impl MarketIndex {
	pub fn new(i: u32) -> Self {
		Self(i)
	}
}
