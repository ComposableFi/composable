use codec::FullCodec;
use frame_support::{
	pallet_prelude::*,
	sp_runtime::traits::AtLeast32BitUnsigned,
	sp_std::fmt::Debug,
	traits::{
		fungibles::{Balanced, CreditOf, DebtOf, HandleImbalanceDrop, Mutate, Transfer},
		tokens::fungibles::{Imbalance, Inspect},
		ExistenceRequirement, SameOrOther,
	},
};

use scale_info::TypeInfo;
pub type Exponent = u32;

pub trait PriceableAsset
where
	Self: Copy,
{
	fn unit<T: From<u64>>(&self) -> T {
		T::from(10u64.pow(self.smallest_unit_exponent()))
	}
	fn smallest_unit_exponent(self) -> Exponent;
}

impl PriceableAsset for u128 {
    fn smallest_unit_exponent(self) -> Exponent {
        0
    }
}

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

pub trait AssetId: FullCodec + Copy + Eq + PartialEq + Debug {}
impl<T: FullCodec + Copy + Eq + PartialEq + Debug> AssetId for T {}
pub trait Balance:
	AtLeast32BitUnsigned
	+ FullCodec
	+ Copy
	+ Default
	+ Debug
	+ MaybeSerializeDeserialize
	+ MaxEncodedLen
{
}
impl<
		T: AtLeast32BitUnsigned
			+ FullCodec
			+ Copy
			+ Default
			+ Debug
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen,
	> Balance for T
{
}

pub trait MultiImbalance<A: AssetId, B: Balance>: Sized {
	type Rev: MultiImbalance<A, B>;

	fn zero(asset: A) -> Self;
	fn drop_zero(self) -> Result<(), Self>;
	fn split(self, amount: B) -> (Self, Self);
	fn merge(self, other: Self) -> Result<Self, (Self, Self)>;
	fn subsume(&mut self, other: Self) -> Result<(), Self>;
	fn offset(self, other: Self::Rev) -> Result<SameOrOther<Self, Self::Rev>, (Self, Self::Rev)>;
	fn peek(&self) -> B;
	fn asset(&self) -> A;
}

impl<
		A: AssetId,
		B: Balance + TypeInfo,
		OppositeOnDrop: HandleImbalanceDrop<A, B>,
		OnDrop: HandleImbalanceDrop<A, B>,
	> MultiImbalance<A, B> for Imbalance<A, B, OnDrop, OppositeOnDrop>
{
	type Rev = Imbalance<A, B, OppositeOnDrop, OnDrop>;

	fn zero(asset: A) -> Self {
		Imbalance::zero(asset)
	}

	fn drop_zero(self) -> Result<(), Self> {
		Imbalance::drop_zero(self)
	}

	fn split(self, amount: B) -> (Self, Self) {
		Imbalance::split(self, amount)
	}

	fn merge(self, other: Self) -> Result<Self, (Self, Self)> {
		Imbalance::merge(self, other)
	}

	fn subsume(&mut self, other: Self) -> Result<(), Self> {
		Imbalance::subsume(self, other)
	}

	fn offset(self, other: Self::Rev) -> Result<SameOrOther<Self, Self::Rev>, (Self, Self::Rev)> {
		Imbalance::offset(self, other)
	}

	fn peek(&self) -> B {
		Imbalance::peek(self)
	}

	fn asset(&self) -> A {
		Imbalance::asset(self)
	}
}

pub trait MultiCurrency<AccountId> {
	type Balance: Balance + MaybeSerializeDeserialize + Debug + MaxEncodedLen;
	type PositiveImbalance: MultiImbalance<Self::AssetId, Self::Balance>;
	type NegativeImbalance: MultiImbalance<Self::AssetId, Self::Balance>;
	type AssetId: AssetId;

	fn total_balance(asset: Self::AssetId, who: &AccountId) -> Self::Balance;
	fn can_slash(asset: Self::AssetId, who: &AccountId, value: Self::Balance) -> bool;
	fn total_issuance(asset: Self::AssetId) -> Self::Balance;
	fn minimum_balance(asset: Self::AssetId) -> Self::Balance;
	fn burn(asset: Self::AssetId, amount: Self::Balance) -> Self::PositiveImbalance;
	fn issue(asset: Self::AssetId, amount: Self::Balance) -> Self::NegativeImbalance;
	fn free_balance(asset: Self::AssetId, who: &AccountId) -> Self::Balance;
	fn ensure_can_withdraw(
		asset: Self::AssetId,
		who: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult;
	fn transfer(
		asset: Self::AssetId,
		source: &AccountId,
		dest: &AccountId,
		value: Self::Balance,
		existence_requirement: ExistenceRequirement,
	) -> DispatchResult;
	fn slash(
		asset: Self::AssetId,
		who: &AccountId,
		value: Self::Balance,
	) -> (Self::NegativeImbalance, Self::Balance);
	fn deposit(
		asset: Self::AssetId,
		who: &AccountId,
		value: Self::Balance,
	) -> Result<Self::PositiveImbalance, DispatchError>;
	fn withdraw(
		asset: Self::AssetId,
		who: &AccountId,
		value: Self::Balance,
	) -> Result<Self::NegativeImbalance, DispatchError>;
}

impl<AccountId, T> MultiCurrency<AccountId> for T
where
	T: Inspect<AccountId> + Balanced<AccountId> + Mutate<AccountId> + Transfer<AccountId>,
	T::Balance: Balance,
{
	type Balance = T::Balance;
	type PositiveImbalance = DebtOf<AccountId, Self>;
	type NegativeImbalance = CreditOf<AccountId, Self>;
	type AssetId = T::AssetId;

	fn total_balance(asset: Self::AssetId, who: &AccountId) -> Self::Balance {
		T::balance(asset, who)
	}

	fn can_slash(asset: Self::AssetId, who: &AccountId, value: Self::Balance) -> bool {
		T::reducible_balance(asset, who, false) >= value
	}

	fn total_issuance(asset: Self::AssetId) -> Self::Balance {
		<T as Inspect<AccountId>>::total_issuance(asset)
	}

	fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
		<T as Inspect<AccountId>>::minimum_balance(asset)
	}

	fn burn(asset: Self::AssetId, amount: Self::Balance) -> Self::PositiveImbalance {
		<T as Balanced<AccountId>>::rescind(asset, amount)
	}

	fn issue(asset: Self::AssetId, amount: Self::Balance) -> Self::NegativeImbalance {
		<T as Balanced<AccountId>>::issue(asset, amount)
	}

	fn free_balance(asset: Self::AssetId, who: &AccountId) -> Self::Balance {
		<T as Inspect<AccountId>>::reducible_balance(asset, who, false)
	}

	fn ensure_can_withdraw(
		asset: Self::AssetId,
		who: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		<T as Inspect<AccountId>>::can_withdraw(asset, who, amount)
			.into_result()
			.map(|_| ())
	}

	fn transfer(
		asset: Self::AssetId,
		source: &AccountId,
		dest: &AccountId,
		value: Self::Balance,
		existence_requirement: ExistenceRequirement,
	) -> DispatchResult {
		<T as Transfer<AccountId>>::transfer(
			asset,
			source,
			dest,
			value,
			matches!(existence_requirement, ExistenceRequirement::KeepAlive),
		)
		.map(|_| ())
	}

	fn slash(
		asset: Self::AssetId,
		who: &AccountId,
		value: Self::Balance,
	) -> (Self::NegativeImbalance, Self::Balance) {
		<T as Balanced<AccountId>>::slash(asset, who, value)
	}

	fn deposit(
		asset: Self::AssetId,
		who: &AccountId,
		value: Self::Balance,
	) -> Result<Self::PositiveImbalance, DispatchError> {
		<T as Balanced<AccountId>>::deposit(asset, who, value)
	}

	fn withdraw(
		asset: Self::AssetId,
		who: &AccountId,
		value: Self::Balance,
	) -> Result<Self::NegativeImbalance, DispatchError> {
		<T as Balanced<AccountId>>::withdraw(asset, who, value)
	}
}
