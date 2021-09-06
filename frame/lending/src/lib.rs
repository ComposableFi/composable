//!

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, FullCodec};
	use composable_traits::{
		lending::{Lending, MarketConfig, MarketConfigInput, Timestamp},
		oracle::Oracle,
		rate_model::*,
		vault::Vault,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungibles::{Mutate, Transfer},
			UnixTime,
		},
		PalletId,
	};
	use num_traits::{CheckedDiv, SaturatingSub};
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
			Zero,
		},
		FixedPointNumber, FixedU128,
	};
	use sp_std::{convert::TryInto, fmt::Debug};

	#[derive(Default, Copy, Clone, Encode, Decode)]
	#[repr(transparent)]
	pub struct MarketIndex(u32);

	impl MarketIndex {
		pub fn new(i: u32) -> Self {
			Self(i)
		}
	}

	pub const PALLET_ID: PalletId = PalletId(*b"Lending!");

	/// number like of hi bits, so that amount and balance calculations are don it it with high
	/// precision via fixed point
	/// while this is 128 bit, cannot support u128
	/// can support it if to use FixedU256
	type LiftedFixedBalance = FixedU128;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Oracle: Oracle<AssetId = Self::AssetId, Balance = Self::Balance>;
		type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;
		type Vault: Vault<VaultId = Self::VaultId, AssetId = Self::AssetId, Balance = Self::Balance>;
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default;
		type Balance: Default
			+ Parameter
			+ Codec
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ SaturatingSub
			+ AtLeast32BitUnsigned
			+ From<u64> // at least 64 bit
			+ Zero
			+ Into<LiftedFixedBalance> // integer part not more than bits in this
			+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128 bit

		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;

		type UnixTime: UnixTime;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		Overflow,
		Underflow,
		/// vault provided does not exist
		VaultNotFound,
		/// Only assets for which we can track price are supported
		AssetWithoutPrice,
		/// The market could not be found
		MarketDoesNotExist,
		CollateralDepositFailed,
		MarketCollateralWasNotDepositedByAccount,
		CollateralFactorIsLessOrEqualOne,
		MarketAndAccountPairNotFound,
	}

	/// Lending instances counter
	#[pallet::storage]
	#[pallet::getter(fn lending_count)]
	pub type LendingCount<T: Config> = StorageValue<_, MarketIndex, ValueQuery>;

	/// Indexed lending instances
	#[pallet::storage]
	#[pallet::getter(fn markets)]
	pub type Markets<T: Config> =
		StorageMap<_, Twox64Concat, MarketIndex, MarketConfig<T::VaultId>, ValueQuery>;

	/// original debt values
	#[pallet::storage]
	#[pallet::getter(fn debt_principals)]
	pub type DebtPrincipals<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		MarketIndex,
		Twox64Concat,
		T::AccountId,
		T::Balance,
		ValueQuery,
	>;

	/// at which lending index account did borrowed.
	#[pallet::storage]
	#[pallet::getter(fn debt_index)]
	pub type DebtIndex<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		MarketIndex,
		Twox64Concat,
		T::AccountId,
		Ratio,
		ValueQuery,
	>;

	/// market borrow index
	#[pallet::storage]
	#[pallet::getter(fn borrow_index)]
	pub type BorrowIndex<T: Config> = StorageMap<_, Twox64Concat, MarketIndex, Ratio, ValueQuery>;

	/// (Market, Account) -> Collateral
	#[pallet::storage]
	#[pallet::getter(fn account_collateral)]
	pub type AccountCollateral<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		MarketIndex,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
		ValueQuery,
	>;

	/// The timestamp of the previous block or defaults to timestamp at genesis.
	/// TODO: should be updated in on_finalize() hook.
	#[pallet::storage]
	#[pallet::getter(fn last_block_timestamp)]
	pub type LastBlockTimestamp<T: Config> = StorageValue<_, Timestamp, ValueQuery>;

	impl<T: Config> Pallet<T> {
		pub fn account_id(market_id: &<Self as Lending>::MarketId) -> <Self as Lending>::AccountId {
			<Self as Lending>::account_id(market_id)
		}
	}

	impl<T: Config> Lending for Pallet<T> {
		type VaultId = <T::Vault as Vault>::VaultId;

		type AccountId = T::AccountId;

		type MarketId = MarketIndex;

		type Balance = T::Balance;

		type BlockNumber = T::BlockNumber;

		fn create_or_update(
			borrow_asset_vault: <T::Vault as Vault>::VaultId,
			collateral_asset_vault: <T::Vault as Vault>::VaultId,
			config_input: MarketConfigInput<Self::AccountId>,
		) -> Result<(), DispatchError> {
			ensure!(
				config_input.collateral_factor > 1.into(),
				Error::<T>::CollateralFactorIsLessOrEqualOne
			);
			let collateral_asset = T::Vault::asset_id(&collateral_asset_vault)?;
			let borrow_asset = T::Vault::asset_id(&borrow_asset_vault)?;

			<T::Oracle as Oracle>::get_price(&collateral_asset)
				.map_err(|_| Error::<T>::AssetWithoutPrice)?;
			<T::Oracle as Oracle>::get_price(&borrow_asset)
				.map_err(|_| Error::<T>::AssetWithoutPrice)?;

			LendingCount::<T>::try_mutate(|MarketIndex(previous_lending_index)| {
				let lending_index = {
					*previous_lending_index += 1;
					MarketIndex(*previous_lending_index)
				};

				let config = MarketConfig {
					borrow: borrow_asset_vault,
					collateral: collateral_asset_vault,
					reserve_factor: config_input.reserve_factor,
					collateral_factor: config_input.collateral_factor,
					interest_rate: InterestRateModel::default(),
				};

				Markets::<T>::insert(lending_index, config);

				Ok(())
			})
		}

		fn account_id(market_id: &Self::MarketId) -> Self::AccountId {
			PALLET_ID.into_sub_account(market_id)
		}

		fn get_markets_for_borrow(borrow: Self::VaultId) -> Vec<Self::MarketId> {
			// allow to be slow until it becomes write transaction (not the case now and then)
			let mut markets = sp_std::vec![];
			for (index, market) in Markets::<T>::iter() {
				if market.borrow == borrow {
					markets.push(index);
				}
			}

			markets
		}

		fn get_all_markets() -> Vec<(Self::MarketId, MarketConfig<<T::Vault as Vault>::VaultId>)> {
			Markets::<T>::iter().map(|(index, config)| (index, config)).collect()
		}

		fn borrow(
			_market_id: &Self::MarketId,
			_debt_owner: &Self::AccountId,
			_amount_to_borrow: Self::Balance,
		) -> Result<(), DispatchError> {
			todo!()
		}

		fn repay_borrow(
			_market_id: &Self::MarketId,
			_from: &Self::AccountId,
			_beneficiary: &Self::AccountId,
			_repay_amount: Self::Balance,
		) -> Result<(), DispatchError> {
			todo!()
		}

		fn total_borrows(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			// Iterate over all account with debt in given market and accumulate that value
			let active_debts = DebtPrincipals::<T>::iter_prefix_values(market_id);
			active_debts.fold(Ok(Self::Balance::zero()), |acc, x| {
				acc.and_then(|a| a.checked_add(&x).ok_or(Error::<T>::Overflow.into()))
			})
		}

		fn total_cash(_pair: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn total_reserves(_pair: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn update_borrows(market_id: &Self::MarketId) -> Result<(), DispatchError> {
			let mut active_debts = DebtPrincipals::<T>::iter_prefix(market_id);
			for (debt_owner, mut debt) in active_debts {
				// new_debt = (debt * market_borrow_index) / user_borrow_index
				let debt_fixed: LiftedFixedBalance = debt.into();
				let market_borrow_index = BorrowIndex::<T>::try_get(market_id)
					.map_err(|_| Error::<T>::MarketDoesNotExist)?;
				let user_borrow_index = DebtIndex::<T>::try_get(market_id, debt_owner)
					.map_err(|_| Error::<T>::MarketDoesNotExist)?;
				let new_debt = debt_fixed
					.checked_mul(&market_borrow_index)
					.and_then(|x| x.checked_div(&user_borrow_index))
					.and_then(|x| x.checked_mul_int(1u64))
					.ok_or(Error::<T>::Overflow)?;
				debt = new_debt.into();
			}
			Ok(())
		}

		fn update_reserves(
			_market_id: &Self::MarketId,
			_reserves: Self::Balance,
		) -> Result<(), DispatchError> {
			todo!()
		}

		fn calc_utilization_ratio(
			cash: &Self::Balance,
			borrows: &Self::Balance,
			reserves: &Self::Balance,
		) -> Result<Ratio, DispatchError> {
			// utilization ratio is 0 when there are no borrows
			if borrows.is_zero() {
				return Ok(Ratio::zero())
			}
			// utilizationRatio = totalBorrows / (totalCash + totalBorrows − totalReserves)
			let total: u128 = cash
				.checked_add(borrows)
				.and_then(|r| r.checked_sub(reserves))
				.ok_or(Error::<T>::Overflow)?
				.into();
			let borrows: u128 = (*borrows).into();
			let mut util_ratio = Ratio::saturating_from_rational(borrows, total);
			if util_ratio > Ratio::one() {
				util_ratio = Ratio::one();
			}
			Ok(util_ratio)
		}

		fn accrue_interest(market_id: &Self::MarketId) -> Result<(), DispatchError> {
			let total_borrows = Self::total_borrows(market_id)?;
			let total_cash = Self::total_cash(market_id)?;
			let total_reserves = Self::total_reserves(market_id)?;
			let util = Self::calc_utilization_ratio(&total_cash, &total_borrows, &total_reserves)?;
			let delta_time = T::UnixTime::now()
				.as_secs()
				.checked_sub(Self::last_block_timestamp())
				.ok_or(Error::<T>::Underflow)?;
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let borrow_rate = market.interest_rate.get_borrow_rate(util).unwrap();

			//update borrow_index
			let borrow_index =
				BorrowIndex::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let borrow_index_new = increment_index(borrow_rate, borrow_index, delta_time)
				.and_then(|r| r.checked_add(&borrow_index))
				.ok_or(Error::<T>::Overflow)?;
			// overwrite value
			BorrowIndex::<T>::insert(market_id, borrow_index_new);

			// update borrows
			Self::update_borrows(market_id)?;

			//TODO: update_reserves
			Ok(())
		}

		fn borrow_balance_current(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			<Self as Lending>::accrue_interest(market_id)?;
			DebtPrincipals::<T>::try_get(market_id, account)
				.map_err(|_| Error::<T>::MarketAndAccountPairNotFound.into())
		}

		fn collateral_of_account(
			_market_id: &Self::MarketId,
			_account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn collateral_required(
			_market_id: &Self::MarketId,
			_borrow_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn get_borrow_limit(
			market_id: &Self::MarketId,
			account: Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			let config =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;

			let collateral_balance = AccountCollateral::<T>::try_get(market_id, account)
				.map_err(|_| Error::<T>::MarketCollateralWasNotDepositedByAccount)?;
			let underlying_collateral_balance: LiftedFixedBalance =
				<T::Vault as Vault>::to_underlying_value(&config.collateral, collateral_balance)?
					.into();
			let collateral_asset_id = <T::Vault as Vault>::asset_id(&config.collateral)?;
			let collateral_price: LiftedFixedBalance =
				<T::Oracle as Oracle>::get_price(&collateral_asset_id)?.0.into();

			let normalized_limit = underlying_collateral_balance
				.checked_mul(&collateral_price)
				.and_then(|collateral_normalized| {
					collateral_normalized.checked_div(&config.collateral_factor)
				})
				.and_then(|borrow_normalized| borrow_normalized.checked_mul_int(1u64))
				.ok_or(Error::<T>::Overflow)?;

			Ok(normalized_limit.into())
		}

		fn deposit_collateral(
			market_id: &Self::MarketId,
			from: &Self::AccountId,
			amount: Self::Balance,
		) -> Result<(), DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let collateral_lp_id = T::Vault::lp_asset_id(&market.collateral)?;
			T::Currency::transfer(
				collateral_lp_id,
				from,
				&Self::account_id(market_id),
				amount,
				true,
			)
			.map_err(|_| Error::<T>::CollateralDepositFailed)?;
			AccountCollateral::<T>::try_mutate(market_id, from, |collateral_balance| {
				let new_collateral_balance = *collateral_balance + amount;
				*collateral_balance = new_collateral_balance;
				Ok(())
			})
		}
	}
}
