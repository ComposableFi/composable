use crate::{
	defi::CurrencyPair,
	math::{SafeAdd, SafeSub},
};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::Get, BoundedVec, RuntimeDebug};
use scale_info::TypeInfo;
use sp_arithmetic::traits::Saturating;
use sp_runtime::{
	traits::{CheckedMul, CheckedSub},
	ArithmeticError, DispatchError, Permill,
};
use sp_std::vec::Vec;

/// Trait for automated market maker.
pub trait Amm {
	/// The asset ID type
	type AssetId;
	/// The balance type of an account
	type Balance;
	/// The user account identifier type for the runtime
	type AccountId;
	/// Type that represents pool id
	type PoolId;

	fn pool_exists(pool_id: Self::PoolId) -> bool;

	fn currency_pair(pool_id: Self::PoolId) -> Result<CurrencyPair<Self::AssetId>, DispatchError>;

	/// Get pure exchange value for given units of given asset. (Note this does not include fees.)
	/// `pool_id` the pool containing the `asset_id`.
	/// `asset_id` the asset the user is interested in.
	/// `amount` the amount of `asset_id` the user want to obtain.
	/// Return the amount of quote asset if `asset_id` is base asset, otherwise the amount of base
	/// asset.
	fn get_exchange_value(
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Buy given `amount` of given asset from the pool.
	/// In buy user does not know how much assets he/she has to exchange to get desired amount.
	fn buy(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError>;

	/// Sell given `amount` of given asset to the pool.
	/// In sell user specifies `amount` of asset he/she wants to exchange to get other asset.
	fn sell(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError>;

	/// Deposit coins into the pool
	/// `amounts` - list of amounts of coins to deposit,
	/// `min_mint_amount` - minimum amout of LP tokens to mint from the deposit.
	fn add_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		base_amount: Self::Balance,
		quote_amount: Self::Balance,
		min_mint_amount: Self::Balance,
		keep_alive: bool,
	) -> Result<(), DispatchError>;

	/// Withdraw coins from the pool.
	/// Withdrawal amount are based on current deposit ratios.
	/// `amount` - quantity of LP tokens to burn in the withdrawal,
	/// `min_amounts` - minimum amounts of underlying coins to receive.
	fn remove_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		lp_amount: Self::Balance,
		min_base_amount: Self::Balance,
		min_quote_amount: Self::Balance,
	) -> Result<(), DispatchError>;

	/// Perform an exchange.
	/// This operation is a buy order on the provided `pair`, effectively trading the quote asset
	/// against the base one. The pair can be swapped to execute a sell order.
	/// Implementor must check the pair.
	fn exchange(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		pair: CurrencyPair<Self::AssetId>,
		quote_amount: Self::Balance,
		min_receive: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError>;
}

/// Pool type
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, Eq, RuntimeDebug)]
pub struct StableSwapPoolInfo<AccountId, AssetId> {
	/// Owner of pool
	pub owner: AccountId,
	/// Swappable assets
	pub pair: CurrencyPair<AssetId>,
	/// AssetId of LP token,
	pub lp_token: AssetId,
	/// Initial amplification coefficient
	pub amplification_coefficient: u16,
	/// Amount of the fee pool charges for the exchange, this goes to liquidity provider.
	pub fee: Permill,
	/// Amount of the fee goes to owner of the pool
	pub owner_fee: Permill,
}

/// Describes a simple exchanges which does not allow advanced configurations such as slippage.
pub trait SimpleExchange {
	type AssetId;
	type Balance;
	type AccountId;
	type Error;

	/// Obtains the current price for a given asset, possibly routing through multiple markets.
	fn price(asset_id: Self::AssetId) -> Option<Self::Balance>;

	/// Exchange `amount` of `from` asset for `to` asset. The maximum price paid for the `to` asset
	/// is `SimpleExchange::price * (1 + slippage)`
	fn exchange(
		from: Self::AssetId,
		from_account: Self::AccountId,
		to: Self::AssetId,
		to_account: Self::AccountId,
		to_amount: Self::Balance,
		slippage: sp_runtime::Perbill,
	) -> Result<Self::Balance, DispatchError>;
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, Eq, RuntimeDebug)]
pub struct ConstantProductPoolInfo<AccountId, AssetId> {
	/// Owner of pool
	pub owner: AccountId,
	/// Swappable assets
	pub pair: CurrencyPair<AssetId>,
	/// AssetId of LP token
	pub lp_token: AssetId,
	/// Amount of the fee pool charges for the exchange
	pub fee: Permill,
	/// Amount of the fee goes to owner of the pool
	pub owner_fee: Permill,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SaleState {
	NotStarted,
	Ongoing,
	Ended,
}

#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Copy, Clone, PartialEq, Eq, TypeInfo)]
pub struct Sale<BlockNumber> {
	/// Block at which the sale start.
	pub start: BlockNumber,
	/// Block at which the sale stop.
	pub end: BlockNumber,
	/// Initial weight of the base asset of the current pair.
	pub initial_weight: Permill,
	/// Final weight of the base asset of the current pair.
	pub final_weight: Permill,
}

impl<BlockNumber: TryInto<u64> + Ord + Copy + Saturating + SafeAdd + SafeSub> Sale<BlockNumber> {
	// TODO unit test
	pub fn current_weights(
		&self,
		current_block: BlockNumber,
	) -> Result<(Permill, Permill), DispatchError> {
		/* NOTE(hussein-aitlahcen): currently only linear

		Linearly decrease the base asset initial_weight to final_weight.
		Quote asset weight is simple 1-base_asset_weight

			  Assuming final_weight < initial_weight
			  current_weight = initial_weight - (current - start) / (end - start) * (initial_weight - final_weight)
							 = initial_weight - normalized_current / sale_duration * weight_range
							 = initial_weight - point_in_sale * weight_range
		   */
		let normalized_current_block = current_block.safe_sub(&self.start)?;
		let point_in_sale = Permill::from_rational(
			normalized_current_block.try_into().map_err(|_| ArithmeticError::Overflow)?,
			self.duration().try_into().map_err(|_| ArithmeticError::Overflow)?,
		);
		let weight_range = self
			.initial_weight
			.checked_sub(&self.final_weight)
			.ok_or(ArithmeticError::Underflow)?;
		let current_base_weight = self
			.initial_weight
			.checked_sub(
				&point_in_sale.checked_mul(&weight_range).ok_or(ArithmeticError::Overflow)?,
			)
			.ok_or(ArithmeticError::Underflow)?;
		let current_quote_weight = Permill::one()
			.checked_sub(&current_base_weight)
			.ok_or(ArithmeticError::Underflow)?;
		Ok((current_base_weight, current_quote_weight))
	}
}

impl<BlockNumber: Copy + Saturating> Sale<BlockNumber> {
	pub fn duration(&self) -> BlockNumber {
		// NOTE(hussein-aitlahcen): end > start as previously checked by PoolIsValid.
		self.end.saturating_sub(self.start)
	}
}

impl<BlockNumber: Ord> Sale<BlockNumber> {
	pub fn state(&self, current_block: BlockNumber) -> SaleState {
		if current_block < self.start {
			SaleState::NotStarted
		} else if current_block >= self.end {
			SaleState::Ended
		} else {
			SaleState::Ongoing
		}
	}
}

#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Copy, Clone, PartialEq, Eq, TypeInfo)]
pub struct LiquidityBootstrappingPoolInfo<AccountId, AssetId, BlockNumber> {
	/// Owner of the pool
	pub owner: AccountId,
	/// Asset pair of the pool along their weight.
	/// Base asset is the project token.
	/// Quote asset is the collateral token.
	pub pair: CurrencyPair<AssetId>,
	/// Sale period of the LBP.
	pub sale: Sale<BlockNumber>,
	/// Trading fees.
	pub fee: Permill,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum DexRouteNode<PoolId> {
	Curve(PoolId),
	Uniswap(PoolId),
}

/// Describes route for DEX.
/// `Direct` gives vector of pool_id to use as router.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum DexRoute<PoolId, MaxHops: Get<u32>> {
	Direct(BoundedVec<DexRouteNode<PoolId>, MaxHops>),
}

pub trait DexRouter<AccountId, AssetId, PoolId, Balance, MaxHops> {
	/// If route is `None` then delete existing entry for `asset_pair`
	/// If route is `Some` and no entry exist for `asset_pair` then add new entry
	/// else update existing entry.
	fn update_route(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		route: Option<BoundedVec<DexRouteNode<PoolId>, MaxHops>>,
	) -> Result<(), DispatchError>;
	/// If route exist return `Some(Vec<PoolId>)`, else `None`.
	fn get_route(asset_pair: CurrencyPair<AssetId>) -> Option<Vec<DexRouteNode<PoolId>>>;
	/// Exchange `dx` of `base` asset of `asset_pair` with associated route.
	fn exchange(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		dx: Balance,
	) -> Result<Balance, DispatchError>;
	/// Sell `amount` of `base` asset of asset_pair with associated route.
	fn sell(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		amount: Balance,
	) -> Result<Balance, DispatchError>;
	/// Buy `amount` of `quote` asset of asset_pair with associated route.
	fn buy(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		amount: Balance,
	) -> Result<Balance, DispatchError>;
}
