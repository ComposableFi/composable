use frame_support::pallet_prelude::*;

/* NOTE(hussein-aitlahcen):
 I initially added a generic type to index into the generatable sub-range but realised it was
 overkill. Perhaps it will be required later if we want to differentiate multiple sub-ranges
 (possibly making a sub-range constant, e.g. using a constant currency id for a pallet expecting
 currency ids to be generated).
 The implementor should ensure that a new `DynamicCurrency` is created and collisions are
 avoided.
*/
/// A currency we can generate given that we have a previous currency.
pub trait DynamicCurrencyId
where
	Self: Sized,
{
	fn next(self) -> Result<Self, DispatchError>;
}

/// Creates a new asset, compatible with [`MultiCurrency`](https://docs.rs/orml-traits/0.4.0/orml_traits/currency/trait.MultiCurrency.html).
/// The implementor should ensure that a new `CurrencyId` is created and collisions are avoided.
pub trait CurrencyFactory<CurrencyId> {
	fn create() -> Result<CurrencyId, DispatchError>;
}
