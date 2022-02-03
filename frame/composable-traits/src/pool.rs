use codec::Codec;
use frame_support::{
	pallet_prelude::*,
	sp_std::fmt::Debug,
	sp_runtime::Perquintill,
};
use scale_info::TypeInfo;

// Holds the id of an asset and how much weight is given to it
//     proportional to all underlying Pool assets.
#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Weight<CurrencyId> {
	pub asset_id: CurrencyId,
	pub weight: Perquintill,
}

// Holds the mapping of an id of an asset and how much weight is given to it
//     proportional to all underlying Pool assets.
pub type WeightsVec<CurrencyId> = Vec::<Weight<CurrencyId>>;

#[derive(Clone, Encode, Decode, Debug, PartialEq, TypeInfo)]
pub enum WeightingMetric<CurrencyId> {
    Equal,
	Fixed(Vec<Weight<CurrencyId>>)
}
impl<CurrencyId> Default for WeightingMetric<CurrencyId> {
	fn default() -> Self {
		WeightingMetric::Equal
	}
}

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Bound<T> {
	pub minimum: T,
	pub maximum: T,
}

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct PoolConfig<AccountId, CurrencyId>
where
	AccountId: core::cmp::Ord,
{
	pub manager: AccountId,

	pub assets: 	  Vec<CurrencyId>,
	pub asset_bounds: Bound<u8>,

	pub weights:	   WeightsVec<CurrencyId>,
	pub weight_bounds: Bound<Perquintill>,

	pub deposit_bounds:  Bound<Perquintill>,
	pub withdraw_bounds: Bound<Perquintill>,

	pub transaction_fee: Perquintill,
}

// Holds the id of an asset and how much of it is being deposited
//     Pool functions accept a Vec<Deposit> as an argument.
#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Deposit<CurrencyId, Balance> {
	pub asset_id: CurrencyId,
	pub amount: Balance,
}

pub trait ConstantMeanMarket {
	type AccountId: core::cmp::Ord;
	type AssetId;
	type Balance;
	
	type PoolId: Clone + Codec + Debug + PartialEq + Default + Parameter;
	type PoolInfo: Clone + Encode + Decode + Default + Debug + PartialEq + TypeInfo;

	// ---------- Queries ----------

	fn account_id(pool_id: &Self::PoolId) -> Self::AccountId;

	fn pool_info(pool_id: &Self::PoolId) -> Result<Self::PoolInfo, DispatchError>;

	fn lp_token_id(pool_id: &Self::PoolId) -> Result<Self::AssetId, DispatchError>;

	fn lp_circulating_supply(pool_id: &Self::PoolId) -> Result<Self::Balance, DispatchError>;

	fn balance_of(pool_id: &Self::PoolId, asset_id: &Self::AssetId) -> Result<Self::Balance, DispatchError>;

	fn reserves_of(pool_id: &Self::PoolId) -> Result<Vec<Deposit<Self::AssetId, Self::Balance>>, DispatchError>;

	// ---------- Commands ----------

	// Used by users to create a new pool with the sepcified configuration.
	fn create(
		from: Self::AccountId,
		config: PoolConfig<Self::AccountId, Self::AssetId>,
		creation_fee: Deposit<Self::AssetId, Self::Balance>,
	) -> Result<Self::PoolId, DispatchError>;

	// Used by users to deposit tokens into the pool. Returns the true amount of 
	//     lp token minted to user.
	fn deposit(
		from: &Self::AccountId,
		pool_id: &Self::PoolId,
		deposits: Vec<Deposit<Self::AssetId, Self::Balance>>,
	) -> Result<Self::Balance, DispatchError>;

	// fn single_asset_deposit(
	// 	from: &Self::AccountId,
	// 	pool_id: &Self::PoolId,
	// 	deposit: Deposit<Self::AssetId, Self::Balance>,
	// ) -> Result<Self::Balance, DispatchError>;

	// // Used by users to deposit lp tokens into the pool and withdraw the equivalent 
	// //     share of the pools assets. Returns a Vector containing the asset id and 
	// //     balance of assets withdrawn.
	// fn withdraw(
	// 	to: &Self::AccountId,
	// 	pool_id: &Self::PoolId,
	// 	lp_amount: Self::Balance,
	// ) -> Result<Vec<Deposit<Self::AssetId, Self::Balance>>, DispatchError>;
}
