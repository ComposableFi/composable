use frame_support::pallet_prelude::*;
use codec::FullCodec;
use core::fmt::Debug;

/// Creates a new asset, compatible with [`MultiCurrency`](https://docs.rs/orml-traits/0.4.0/orml_traits/currency/trait.MultiCurrency.html).
/// The implementor should ensure that a new `CurrencyId` is created and collisions are avoided.
pub trait CurrencyFactory<CurrencyId>
where CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug
{
    fn create() -> Result<CurrencyId, DispatchError>;
}
