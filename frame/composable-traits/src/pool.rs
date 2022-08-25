use codec::Codec;
use frame_support::{
	pallet_prelude::*,
	sp_std::{fmt::Debug, vec::Vec},
	storage::bounded_vec::BoundedVec,
};
use scale_info::TypeInfo;

use fixed::types::U110F18;

/// Unsigned Integer type used for calculations behind the scenes
pub type FixedBalance = U110F18;

/// Type alias used for working with a list of asset ids
pub type Assets<AssetId, PoolSize> = BoundedVec<AssetId, PoolSize>;

/// Holds the id of an asset and how much weight is given to it
///     proportional to all underlying Pool assets.
#[derive(Clone, Encode, Decode, Default, Debug, MaxEncodedLen, PartialEq, TypeInfo)]
pub struct Weight<CurrencyId, Percent> {
	pub asset_id: CurrencyId,
	pub weight: Percent,
}

/// Type alias used for working with a list of [`Weight`](Weight) objects
pub type WeightsVec<CurrencyId, Percent, PoolSize> =
	BoundedVec<Weight<CurrencyId, Percent>, PoolSize>;

/// Struct to maintain the min/max value bounds for some of the Pool's configurable
///     parameters.
#[derive(
	Clone, Copy, Encode, Decode, Default, Debug, MaxEncodedLen, PartialEq, PartialOrd, TypeInfo,
)]
pub struct Bound<T> {
	pub minimum: Option<T>,
	pub maximum: Option<T>,
}

impl<T> Bound<T> {
	pub fn new(lower: Option<T>, upper: Option<T>) -> Self {
		Bound { minimum: lower, maximum: upper }
	}
}

// Does not derive Copy as assets and weights are Vectors (with their
//     data resides on the heap) and thus doesn't derive Copy
#[derive(Clone, Encode, Decode, Default, Debug, MaxEncodedLen, PartialEq, TypeInfo)]
pub struct PoolConfig<AccountId, AssetId, Percent, PoolSize>
where
	AccountId: core::cmp::Ord,
	PoolSize: core::cmp::Ord + frame_support::traits::Get<u32>,
{
	/// Owner of pool
	pub owner: AccountId,
	/// Amount of the fee pool charges for the exchange
	pub fee: Percent,
	/// Vector of the Pool's underlying assets
	pub assets: Assets<AssetId, PoolSize>,
	/// Vector of the Pool's underlying asset weights
	pub weights: WeightsVec<AssetId, Percent, PoolSize>,
	/// Min/max bounds on weights of assets for the pool
	pub weight_bounds: Bound<Percent>,
	/// Min/max bounds on amount of assets that can be deposited at once
	pub deposit_bounds: Bound<Percent>,
	/// Min/max bounds on amount of assets that can be withdrawn at once
	pub withdraw_bounds: Bound<Percent>,
}

#[derive(Clone, Copy, Encode, Decode, Debug, MaxEncodedLen, PartialEq, TypeInfo, Default)]
pub struct PoolInfo<AccountId, AssetId, Percent> {
	/// Owner of pool
	pub owner: AccountId,
	/// AssetId of LP token,
	pub lp_token: AssetId,
	/// Amount of the fee pool charges for the exchange
	pub fee: Percent,
	/// Min/max bounds on weights of assets for the pool
	pub weight_bounds: Bound<Percent>,
	/// Min/max bounds on amount of assets that can be deposited at once
	pub deposit_bounds: Bound<Percent>,
	/// Min/max bounds on amount of assets that can be withdrawn at once
	pub withdraw_bounds: Bound<Percent>,
}

/// Holds the id of an asset and how a balance associated with the asset. Can be used to
/// represent:
/// - deposits
/// - withdraws
/// - Pool reserves
#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Reserve<CurrencyId, Balance> {
	pub asset_id: CurrencyId,
	pub amount: Balance,
}

/// type synonym to better represent the [`Reserve`](Reserve) type in deposits
pub type Deposit<CurrencyId, Balance> = Reserve<CurrencyId, Balance>;
/// type synonym to better represent the [`Reserve`](Reserve) type in withdrawals
pub type Withdraw<CurrencyId, Balance> = Reserve<CurrencyId, Balance>;

pub trait ConstantMeanMarket {
	/// Corresponds to the Ids used by the pallet to uniquely identify accounts.
	type AccountId: core::cmp::Ord;
	/// Corresponds to the Ids used by the pallet to uniquely identify assets.
	type AssetId;
	/// The Balance type used by the pallet for bookkeeping.
	type Balance;
	/// The type used by the pallet to deal with asset weights.
	type Weight;

	/// Key type for Pool that uniquely identifieds a Pool.
	type PoolId: Clone + Codec + Debug + PartialEq + Default + Parameter;
	/// Represents the PoolInfo struct that is used to save information about each Pool.
	type PoolInfo: Clone + Encode + Decode + Default + Debug + PartialEq + TypeInfo;
    /// Represents pool size
	type PoolSize: Get<u32> + Debug + Clone + core::cmp::Ord;

	// ---------- Queries ----------

	/// Used by users to query the price of an asset relative to a specific numeraire.
	///
	/// ## Parameters
	/// - `pool_id`: The Pools identifier. This must correspond to an existing Pool.
	/// - `asset`: The identifier of the asset wanting to obtain the price of. This asset must be
	///     tracked by the specified Pool.
	/// - `numeraire`: The identifier of the base asset wanting to obtain the price of `asset` in.
	///         This asset must be tracked by the specified Pool.
	fn spot_price(
		pool_id: &Self::PoolId,
		asset: &Self::AssetId,
		numeraire: &Self::AssetId,
	) -> Result<FixedBalance, DispatchError>;

	// ---------- Commands ----------

	/// Used by users to create a new pool with the sepcified configuration. Returns the Pool Index
	///     of the created Pool
	///
	/// ## Parameters
	/// - `from`: The `account_id` of the issuing user.
	/// - `config`: A [`PoolConfig`](PoolConfig) struct containing the
	///     parameter values to instantiate a new Pool with.
	/// - `creation_fee`: The blance, in the runtimes native asset, that the issuer is supplying
	///     for the creation fee.
	fn create(
		from: Self::AccountId,
		config: PoolConfig<Self::AccountId, Self::AssetId, Self::Weight, Self::PoolSize>,
		creation_fee: Deposit<Self::AssetId, Self::Balance>,
	) -> Result<Self::PoolId, DispatchError>;

	/// Used by users to deposit tokens into the pool. Returns the true amount of
	///     lp token minted to user.
	///
	/// ## Parameters
	/// - `from`: The `account_id` of the issuing user.
	/// - `pool_id`: A unique identifier specifying the Pool to interact with.
	/// - `deposits`: A vector of [`Deposit`](Deposit) structs specifying
	///     the balance of each asset to deposit
	fn all_asset_deposit(
		from: &Self::AccountId,
		pool_id: &Self::PoolId,
		deposits: Vec<Deposit<Self::AssetId, Self::Balance>>,
	) -> Result<Self::Balance, DispatchError>;

	// fn single_asset_deposit(
	// 	from: &Self::AccountId,
	// 	pool_id: &Self::PoolId,
	// 	deposit: Deposit<Self::AssetId, Self::Balance>,
	// ) -> Result<Self::Balance, DispatchError>;

	/// Used by users to deposit lp tokens into the pool and withdraw the equivalent
	///     share of the Pool's assets. Returns a Vector containing the asset ids and
	///     balances of the withdrawn assets.
	///
	/// ## Parameters
	/// - `from`: The `account_id` of the issuing user.
	/// - `pool_id`: A unique identifier specifying the Pool to interact with.
	/// - `deposits`: A vector of [`Deposit`](Deposit) structs specifying
	///     the balance of each asset to deposit
	fn all_asset_withdraw(
		to: &Self::AccountId,
		pool_id: &Self::PoolId,
		lp_amount: Self::Balance,
	) -> Result<Vec<Withdraw<Self::AssetId, Self::Balance>>, DispatchError>;
}
