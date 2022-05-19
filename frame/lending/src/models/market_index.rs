use frame_support::pallet_prelude::*;

pub type MarketId = u32;

#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq, MaxEncodedLen, TypeInfo)]
#[repr(transparent)]
pub struct MarketIndex(
    #[cfg(test)] // to allow pattern matching in tests outside of this crate
    pub MarketId,

    #[cfg(not(test))]
    pub(crate) MarketId,
);

impl MarketIndex {
    pub fn new(i: u32) -> Self {
        Self(i)
    }
}