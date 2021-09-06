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

#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, EncodeLike, FullCodec};
	use composable_traits::{
		lending::{
			Lending, MarketConfig, MarketConfigInput, NormalizedCollateralFactor, Timestamp,
		},
		oracle::Oracle,
		rate_model::*,
		vault::{Deposit, FundsAvailability, StrategicVault, Vault, VaultConfig},
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::{fungibles::MutateHold, DepositConsequence},
			Backing, UnixTime,
		},
		PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor, Config as SystemConfig};
	use num_traits::{Bounded, CheckedDiv, SaturatingSub};
	use sp_runtime::{
		helpers_128bit::multiply_by_rational,
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedConversion, CheckedMul,
			CheckedSub, Convert, Hash, One, Zero,
		},
		FixedPointNumber, FixedPointOperand, FixedU128, Permill, Perquintill,
	};
	use sp_std::{convert::TryInto, fmt::Debug};

	#[derive(Default, Copy, Clone, Encode, Decode)]
	#[repr(transparent)]
	pub struct MarketIndex(u32);

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
		type Vault: StrategicVault<
			VaultId = Self::VaultId,
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
		>;

		type CurrencyFactory: CurrencyFactory<Self::AssetId>;
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
		NotEnoughCollateralToBorrowAmount,
		CannotBorrowInCurrentLendingState,
		NotEnoughBorrowAsset,
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

	/// Original debt values are on balances.
	/// Debt token allows to simplify some debt management and implementation of features
	#[pallet::storage]
	#[pallet::getter(fn debt_currencies)]

	pub type DebtMarkets<T: Config> = StorageMap<
		_,
		Twox64Concat,
		MarketIndex,
		T::AssetId,
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

	impl<T: Config> Lending for Pallet<T> {
		/// we are operating only on vault types, so restricted by these
		type VaultId = <T::Vault as Vault>::VaultId;
		type AccountId = <T::Vault as Vault>::AccountId;
		type Balance = T::Balance;

		type MarketId = MarketIndex;

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
			market_id: &Self::MarketId,
			debt_owner: &Self::AccountId,
			amount_to_borrow: Self::Balance,
		) -> Result<(), DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;

			// how much account owes
			let borrower_balance_with_interest =
				Self::borrow_balance_current(market_id, debt_owner)?;
			let asset_id = <T::Vault as Vault>::asset_id(&market.borrow)?;
			let borrow_asset_price = <T::Oracle as Oracle>::get_price(&asset_id)?.0;
			let borrowed_normalized = borrower_balance_with_interest
				.checked_mul(&borrow_asset_price)
				.ok_or(Error::<T>::Overflow)?;

			// how much he can borrow total
			let borrow_limit = Self::get_borrow_limit(&market_id, debt_owner)?;

			let possible_borrow = borrow_limit
				.checked_sub(&borrowed_normalized)
				.ok_or(Error::<T>::NotEnoughCollateralToBorrowAmount)?;

			let account_id = Self::account_id(market_id);
			let can_withdraw = <T::Currency as Inspect<T::AccountId>>::reducible_balance(
				asset_id.clone(),
				&account_id,
				true,
			);
			match <T::Vault as StrategicVault>::available_funds(
				&market.borrow,
				&Self::account_id(&market_id),
			)? {
				FundsAvailability::Withdrawable(balance) => {
					<T::Vault as StrategicVault>::withdraw(
						&market.borrow,
						&account_id,
						possible_borrow,
					)?;
					ensure!(
						can_withdraw + balance >= possible_borrow,
						Error::<T>::NotEnoughBorrowAsset
					)
				}
				FundsAvailability::Depositable(_) => (),
				// TODO: decide when react and how to return fees back
				// https://mlabs-corp.slack.com/archives/C02CRQ9KW04/p1630662664380600?thread_ts=1630658877.363600&cid=C02CRQ9KW04
				FundsAvailability::MustLiquidate => {
					return Err(Error::<T>::CannotBorrowInCurrentLendingState.into())
				}
			}

			<T::Currency as Transfer<T::AccountId>>::transfer(
				asset_id.clone(),
				&Self::account_id(market_id),
				debt_owner,
				possible_borrow,
				true,
			)?;

			let market_index =
				BorrowIndex::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			//ASK: storage or currency?
			//<T::DebtCurrency as Mutate<T::AccountId>>::mint_into(market_id.clone(), debt_owner, possible_borrow)?;
			DebtPrincipals::<T>::insert(market_id, debt_owner, possible_borrow);
			DebtIndex::<T>::insert(market_id, debt_owner, market_index);

			Ok(())
		}

		fn repay_borrow(
			market_id: &Self::MarketId,
			from: &Self::AccountId,
			beneficiary: &Self::AccountId,
			repay_amount: Self::Balance,
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

		fn total_cash(pair: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn total_reserves(pair: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn update_borrows(
			market_id: &Self::MarketId,
			interest_rate: Rate,
		) -> Result<(), DispatchError> {
			let mut active_debts = DebtPrincipals::<T>::iter_prefix_values(market_id);
			for mut debt in active_debts {
				// new_debt = (debt * interest_rate) + debt
				let debt_fixed: LiftedFixedBalance = debt.into();
				let new_debt = debt_fixed
					.checked_mul(&interest_rate)
					.and_then(|d| d.checked_add(&debt_fixed))
					.and_then(|x| x.checked_mul_int(1u64))
					.ok_or(Error::<T>::Overflow)?;
				debt = new_debt.into();
			}
			Ok(())
		}

		fn update_reserves(
			market_id: &Self::MarketId,
			reserves: Self::Balance,
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
			let interest_time = Rate::saturating_from_rational(delta_time, SECONDS_PER_YEAR);
			let borrow_rate =
				borrow_rate.checked_mul(&interest_time).ok_or(Error::<T>::Overflow)?;
			Self::update_borrows(market_id, borrow_rate)?;

			//TODO: update_reserves
			Ok(())
		}

		fn borrow_balance_current(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			<Self as Lending>::accrue_interest(market_id)?;
			let principal = DebtPrincipals::<T>::try_get(market_id, account)
				.map_err(|_| Error::<T>::MarketAndAccountPairNotFound)?;
			// If borrowBalance = 0 then borrow index is likely also 0.
			// Rather than failing the calculation with a division by 0, we immediately return 0 in
			// this case.
			if principal.is_zero() {
				return Ok(T::Balance::zero())
			}

			let account_interest_index = DebtIndex::<T>::try_get(market_id, account)
				.map_err(|_| Error::<T>::MarketAndAccountPairNotFound)?;
			let market_interest_index =
				BorrowIndex::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;

			let principal: LiftedFixedBalance = principal.into();

			let balance = principal
				.checked_mul(&market_interest_index)
				.and_then(|from_start_total| from_start_total.checked_div(&account_interest_index))
				.and_then(|x| x.checked_mul_int(1u64))
				.ok_or(Error::<T>::Overflow)?;
			Ok(balance.into())
		}

		fn collateral_of_account(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn collateral_required(
			market_id: &Self::MarketId,
			borrow_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn get_borrow_limit(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
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
