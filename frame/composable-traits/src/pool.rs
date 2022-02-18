use codec::Codec;
use frame_support::{
	pallet_prelude::*,
	sp_std::fmt::Debug,
	sp_runtime::Perquintill,
};
use scale_info::TypeInfo;

use fixed::types::U110F18;

pub type FixedBalance = U110F18;
pub type Assets<AssetId> = Vec<AssetId>;

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

{
// Weighting Metric -> Move to Index Pallet
// #[derive(Clone, Encode, Decode, Debug, PartialEq, TypeInfo)]
// pub enum WeightingMetric<CurrencyId> {
//     Equal,
// 	Fixed(Vec<Weight<CurrencyId>>)
// }
// impl<CurrencyId> Default for WeightingMetric<CurrencyId> {
// 	fn default() -> Self {
// 		WeightingMetric::Equal
// 	}
// }
}

#[derive(Clone, Copy, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Bound<T> {
	pub minimum: T,
	pub maximum: T,
}

// Does not derive Copy as asset_ids is a Vector (i.e. the 
//     data resides on the heap) and thus doesn't derive Copy
#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct PoolConfig<AccountId, AssetId>
where
	AccountId: core::cmp::Ord,
{
	pub owner: AccountId,

	pub fee: Perquintill,

	pub assets: Assets<AssetId>,
	pub asset_bounds: Bound<u8>,

	pub weights: WeightsVec<AssetId>,
	pub weight_bounds: Bound<Perquintill>,

	pub deposit_bounds: Bound<Perquintill>,
	pub withdraw_bounds: Bound<Perquintill>,
}

#[derive(Clone, Copy, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct PoolInfo<AccountId, AssetId> {
	// Owner of pool
	pub owner: AccountId,
	// AssetId of LP token,
	pub lp_token: AssetId,
	// Amount of the fee pool charges for the exchange
	pub fee: Perquintill,
	// Min/max bounds on number of assets allowed in the pool
	pub asset_bounds: Bound<u8>,
	// Min/max bounds on weights of assets for the pool
	pub weight_bounds: Bound<Perquintill>,
	// Min/max bounds on amount of assets that can be deposited and withdrawn at once	
	pub deposit_bounds:  Bound<Perquintill>,
	pub withdraw_bounds: Bound<Perquintill>,
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
	type Weight;
	
	type PoolId: Clone + Codec + Debug + PartialEq + Default + Parameter;
	type PoolInfo: Clone + Encode + Decode + Default + Debug + PartialEq + TypeInfo;

	// ---------- Queries ----------

	fn account_id(pool_id: &Self::PoolId) -> Self::AccountId;

	fn pool_info(pool_id: &Self::PoolId) -> Result<Self::PoolInfo, DispatchError>;

	fn lp_token_id(pool_id: &Self::PoolId) -> Result<Self::AssetId, DispatchError>;

	fn lp_circulating_supply(pool_id: &Self::PoolId) -> Result<Self::Balance, DispatchError>;

	fn reserves_of(pool_id: &Self::PoolId) -> Result<Vec<Deposit<Self::AssetId, Self::Balance>>, DispatchError>;

	fn balance_of(pool_id: &Self::PoolId, asset_id: &Self::AssetId) -> Result<Self::Balance, DispatchError>;

	fn weight_of(pool_id: &Self::PoolId, asset_id: &Self::AssetId) -> Result<Self::Weight, DispatchError>;

	// ---------- Commands ----------

	// Used by users to create a new pool with the sepcified configuration. Returns the Pool Index 
	//     of the created Pool
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
