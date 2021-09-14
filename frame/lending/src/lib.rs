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
mod mocks;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, FullCodec};
	use composable_traits::{
		currency::CurrencyFactory,
		lending::{
			BorrowAmountOf, CollateralLpAmountOf, Lending, MarketConfig, MarketConfigInput,
			NormalizedCollateralFactor,
		},
		oracle::Oracle,
		rate_model::*,
		vault::{Deposit, FundsAvailability, StrategicVault, Vault, VaultConfig},
	};
	use frame_support::{
		pallet_prelude::*,
		storage::{with_transaction, TransactionOutcome},
		traits::{
			fungibles::{Inspect, InspectHold, Mutate, MutateHold, Transfer},
			tokens::DepositConsequence,
			GenesisBuild, Len, ReservableCurrency, UnixTime,
		},
		PalletId,
	};
	use num_traits::{CheckedDiv, SaturatingSub};
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
			Saturating, Zero,
		},
		ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128, Perquintill,
	};
	use sp_std::fmt::Debug;

	use composable_traits::rate_model::{LiftedFixedBalance, SafeArithmetic};

	#[derive(Default, Debug, Copy, Clone, Encode, Decode)]
	#[repr(transparent)]
	pub struct MarketIndex(u32);

	impl MarketIndex {
		pub fn new(i: u32) -> Self {
			Self(i)
		}
	}

	pub const PALLET_ID: PalletId = PalletId(*b"Lending!");

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
			+ FixedPointOperand
			+ Into<LiftedFixedBalance> // integer part not more than bits in this
			+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit

		// vault owned - can transfer, cannot mint
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;

		// market owned - debt token can be minted
		type MarketCurrency: Transfer<Self::AccountId, Balance = u128, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = u128, AssetId = Self::AssetId>
			+ MutateHold<Self::AccountId, Balance = u128, AssetId = Self::AssetId>
			+ InspectHold<Self::AccountId, Balance = u128, AssetId = Self::AssetId>;

		type UnixTime: UnixTime;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			with_transaction(|| {
				let now = T::UnixTime::now().as_secs();
				let results = Markets::<T>::iter()
					.map(|(index, _)| <Pallet<T>>::accrue_interest(&index, now))
					.collect::<Vec<_>>();
				let (_oks, errors): (Vec<_>, Vec<_>) = results.iter().partition(|r| r.is_ok());
				if errors.is_empty() {
					LastBlockTimestamp::<T>::put(now);
					TransactionOutcome::Commit(1000)
				} else {
					errors.iter().for_each(|e| {
						if let Err(e) = e {
							log::error!(
								"This should never happen, could not initialize block!!! {:#?} {:#?}",
								block_number,
								e
							)
						}
					});
					TransactionOutcome::Rollback(0)
				}
			});
			0
		}
	}

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
		NotEnoughCollateral,
		TransferFailed,
		CannotWithdrawFromProvidedBorrowAccount,
		CannotRepayMoreThanBorrowAmount,
		BorrowRateDoesNotExist,
		BorrowIndexDoesNotExist,
	}

	pub struct BorrowerData {
		pub collateral_balance: LiftedFixedBalance,
		pub collateral_price: LiftedFixedBalance,
		pub borrower_balance_with_interest: LiftedFixedBalance,
		pub borrow_price: LiftedFixedBalance,
		pub collateral_factor: NormalizedCollateralFactor,
	}

	impl BorrowerData {
		#[inline(always)]
		pub fn new<T: Into<LiftedFixedBalance>>(
			collateral_balance: T,
			collateral_price: T,
			borrower_balance_with_interest: T,
			borrow_price: T,
			collateral_factor: NormalizedCollateralFactor,
		) -> Self {
			Self {
				collateral_balance: collateral_balance.into(),
				collateral_price: collateral_price.into(),
				borrower_balance_with_interest: borrower_balance_with_interest.into(),
				borrow_price: borrow_price.into(),
				collateral_factor: collateral_factor.into(),
			}
		}

		#[inline(always)]
		pub fn collateral_over_borrow(&self) -> Result<LiftedFixedBalance, ArithmeticError> {
			let collateral = self.collateral_balance.safe_mul(&self.collateral_price)?;
			let borrowed = self
				.borrower_balance_with_interest
				.safe_mul(&self.borrow_price)?
				.safe_mul(&self.collateral_factor)?;
			collateral.safe_sub(&borrowed)
		}

		#[inline(always)]
		pub fn borrow_for_collateral(&self) -> Result<LiftedFixedBalance, ArithmeticError> {
			let max_borrow = self
				.collateral_balance
				.safe_mul(&self.collateral_price)?
				.safe_div(&self.collateral_factor)?;
			let borrowed = self.borrower_balance_with_interest.safe_mul(&self.borrow_price)?;
			Ok(max_borrow.saturating_sub(borrowed))
		}
	}

	/// Lending instances counter
	#[pallet::storage]
	#[pallet::getter(fn lending_count)]
	pub type LendingCount<T: Config> = StorageValue<_, MarketIndex, ValueQuery>;

	/// Indexed lending instances
	#[pallet::storage]
	#[pallet::getter(fn markets)]
	pub type Markets<T: Config> = StorageMap<
		_,
		Twox64Concat,
		MarketIndex,
		MarketConfig<T::VaultId, T::AssetId, T::AccountId>,
		ValueQuery,
	>;

	/// Original debt values are on balances.
	/// Debt token allows to simplify some debt management and implementation of features
	#[pallet::storage]
	#[pallet::getter(fn debt_currencies)]

	pub type DebtMarkets<T: Config> =
		StorageMap<_, Twox64Concat, MarketIndex, T::AssetId, ValueQuery>;

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
	#[pallet::storage]
	#[pallet::getter(fn last_block_timestamp)]
	pub type LastBlockTimestamp<T: Config> = StorageValue<_, Timestamp, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig {}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self {}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			let now = T::UnixTime::now().as_secs();
			LastBlockTimestamp::<T>::put(now);
		}
	}

	#[cfg(feature = "std")]
	impl GenesisConfig {
		/// Direct implementation of `GenesisBuild::build_storage`.
		///
		/// Kept in order not to break dependency.
		pub fn build_storage<T: Config>(&self) -> Result<sp_runtime::Storage, String> {
			<Self as frame_support::traits::GenesisBuild<T>>::build_storage(self)
		}

		/// Direct implementation of `GenesisBuild::assimilate_storage`.
		///
		/// Kept in order not to break dependency.
		pub fn assimilate_storage<T: Config>(
			&self,
			storage: &mut sp_runtime::Storage,
		) -> Result<(), String> {
			<Self as frame_support::traits::GenesisBuild<T>>::assimilate_storage(self, storage)
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn total_interest_accurate(
			market_id: &<Self as Lending>::MarketId,
		) -> Result<u128, DispatchError> {
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let total_interest =
				T::MarketCurrency::balance(debt_asset_id, &Self::account_id(market_id));
			Ok(total_interest)
		}

		pub fn account_id(market_id: &<Self as Lending>::MarketId) -> <Self as Lending>::AccountId {
			<Self as Lending>::account_id(market_id)
		}
		pub fn calc_utilization_ratio(
			cash: &<Self as Lending>::Balance,
			borrows: &<Self as Lending>::Balance,
		) -> Result<Ratio, DispatchError> {
			<Self as Lending>::calc_utilization_ratio(cash, borrows)
		}
		pub fn create(
			borrow_asset: <Self as Lending>::AssetId,
			collateral_asset: <Self as Lending>::AssetId,
			config_input: MarketConfigInput<<Self as Lending>::AccountId>,
		) -> Result<(<Self as Lending>::MarketId, <Self as Lending>::VaultId), DispatchError> {
			<Self as Lending>::create(borrow_asset, collateral_asset, config_input)
		}
		pub fn deposit_collateral(
			market_id: &<Self as Lending>::MarketId,
			account_id: &<Self as Lending>::AccountId,
			amount: CollateralLpAmountOf<Self>,
		) -> Result<(), DispatchError> {
			<Self as Lending>::deposit_collateral(market_id, account_id, amount)
		}
		pub fn collateral_of_account(
			market_id: &<Self as Lending>::MarketId,
			account: &<Self as Lending>::AccountId,
		) -> Result<<Self as Lending>::Balance, DispatchError> {
			<Self as Lending>::collateral_of_account(market_id, account)
		}
		pub fn withdraw_collateral(
			market_id: &<Self as Lending>::MarketId,
			account: &<Self as Lending>::AccountId,
			amount: CollateralLpAmountOf<Self>,
		) -> Result<(), DispatchError> {
			<Self as Lending>::withdraw_collateral(market_id, account, amount)
		}

		pub fn get_borrow_limit(
			market_id: &<Self as Lending>::MarketId,
			account: &<Self as Lending>::AccountId,
		) -> Result<<Self as Lending>::Balance, DispatchError> {
			<Self as Lending>::get_borrow_limit(market_id, account)
		}

		pub fn borrow(
			market_id: &<Self as Lending>::MarketId,
			debt_owner: &<Self as Lending>::AccountId,
			amount_to_borrow: <Self as Lending>::Balance,
		) -> Result<(), DispatchError> {
			<Self as Lending>::borrow(market_id, debt_owner, amount_to_borrow)
		}

		pub fn borrow_balance_current(
			market_id: &<Self as Lending>::MarketId,
			account: &<Self as Lending>::AccountId,
		) -> Result<Option<BorrowAmountOf<Self>>, DispatchError> {
			<Self as Lending>::borrow_balance_current(market_id, account)
		}

		pub fn total_borrows(
			market_id: &<Self as Lending>::MarketId,
		) -> Result<<Self as Lending>::Balance, DispatchError> {
			<Self as Lending>::total_borrows(market_id)
		}

		pub fn total_cash(
			market_id: &<Self as Lending>::MarketId,
		) -> Result<<Self as Lending>::Balance, DispatchError> {
			<Self as Lending>::total_cash(market_id)
		}

		pub fn total_interest(
			market_id: &<Self as Lending>::MarketId,
		) -> Result<<Self as Lending>::Balance, DispatchError> {
			<Self as Lending>::total_interest(market_id)
		}
	}

	impl<T: Config> Lending for Pallet<T> {
		/// we are operating only on vault types, so restricted by these
		type AssetId = T::AssetId;
		type VaultId = <T::Vault as Vault>::VaultId;
		type AccountId = <T::Vault as Vault>::AccountId;
		type Balance = T::Balance;

		type MarketId = MarketIndex;

		type BlockNumber = T::BlockNumber;

		fn create(
			borrow_asset: Self::AssetId,
			collateral_asset: Self::AssetId,
			config_input: MarketConfigInput<Self::AccountId>,
		) -> Result<(Self::MarketId, Self::VaultId), DispatchError> {
			ensure!(
				config_input.collateral_factor > 1.into(),
				Error::<T>::CollateralFactorIsLessOrEqualOne
			);

			<T::Oracle as Oracle>::get_price(&collateral_asset)
				.map_err(|_| Error::<T>::AssetWithoutPrice)?;
			<T::Oracle as Oracle>::get_price(&borrow_asset)
				.map_err(|_| Error::<T>::AssetWithoutPrice)?;

			LendingCount::<T>::try_mutate(|MarketIndex(previous_market_index)| {
				let market_index = {
					*previous_market_index += 1;
					MarketIndex(*previous_market_index)
				};

				/* Generate the underlying vault that will hold borrowable asset


				#  -----------
				#  |  vault  |  I
				#  -----------
				#       |
				# -------------
				# |  strategy | P
				# -------------
				#       |                            M
				#       |                   -------------------
				#       |                   |    ---------    |
				#       -----------------------> |       |    |
				#                           |    | vault |    |
				#       -----------------------> |       |    |
				#       |                   |    ---------    |
				#       |                   -------------------
				#       |
				# -------------
				# |  strategy | Q
				# -------------
				#       |
				#  ----------
				#  |  vault | J
				#  ----------


				   The idea here is that the lending pallet owns the vault.

				   Let's assume a group of users X want to use a strategy P
				   and a group of users Y want to use a strategy Q:

				   Assuming both groups are interested in lending an asset A, they can create two vaults I and J.
				   They would deposit in I and J, then set P and respectively Q as their strategy.
				   Now imagine that our lending market M has a good APY, both strategy P and Q
				   could decide to allocate a share for it, transferring from I and J to the borrow asset vault of M.
				   Their allocated share could differ because of the strategies being different,
				   but the lending Market would have all the lendable funds in a single vault.
				*/
				let borrow_asset_vault = T::Vault::create(
					Deposit::Existential,
					VaultConfig {
						asset_id: borrow_asset,
						reserved: config_input.reserved,
						manager: config_input.manager.clone(),
						strategies: [(
							Self::account_id(&market_index),
							// Borrowable = 100% - reserved
							Perquintill::one().saturating_sub(config_input.reserved),
						)]
						.iter()
						.cloned()
						.collect(),
					},
				)?;

				let config = MarketConfig {
					manager: config_input.manager,
					borrow: borrow_asset_vault.clone(),
					collateral: collateral_asset,
					collateral_factor: config_input.collateral_factor,
					interest_rate_model: InterestRateModel::default(),
				};

				let debt_asset_id = T::CurrencyFactory::create()?;
				DebtMarkets::<T>::insert(market_index, debt_asset_id);
				Markets::<T>::insert(market_index, config);
				BorrowIndex::<T>::insert(market_index, Ratio::one());

				Ok((market_index, borrow_asset_vault))
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

		fn get_all_markets(
		) -> Vec<(Self::MarketId, MarketConfig<T::VaultId, T::AssetId, T::AccountId>)> {
			Markets::<T>::iter().map(|(index, config)| (index, config)).collect()
		}

		fn borrow(
			market_id: &Self::MarketId,
			debt_owner: &Self::AccountId,
			amount_to_borrow: Self::Balance,
		) -> Result<(), DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;

			let asset_id = T::Vault::asset_id(&market.borrow)?;
			let possible_borrow = Self::get_borrow_limit(&market_id, debt_owner)?;
			if possible_borrow < amount_to_borrow {
				return Err(Error::<T>::NotEnoughCollateralToBorrowAmount.into());
			}
			let account_id = Self::account_id(market_id);
			let can_withdraw = T::Currency::reducible_balance(asset_id.clone(), &account_id, true);
			let fund = T::Vault::available_funds(&market.borrow, &Self::account_id(&market_id))?;
			match fund {
				FundsAvailability::Withdrawable(balance) => {
					<T::Vault as StrategicVault>::withdraw(
						&market.borrow,
						&account_id,
						amount_to_borrow,
					)?;
					ensure!(
						can_withdraw + balance >= amount_to_borrow,
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

			T::Currency::transfer(
				asset_id.clone(),
				&Self::account_id(market_id),
				debt_owner,
				amount_to_borrow,
				true,
			)?;

			let market_index =
				BorrowIndex::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;

			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let amount_to_borrow: u128 = amount_to_borrow.into();
			let amount_to_borrow = amount_to_borrow
				.checked_mul(LiftedFixedBalance::accuracy())
				.expect("more detailed currency");
			T::MarketCurrency::mint_into(debt_asset_id, debt_owner, amount_to_borrow)?;
			T::MarketCurrency::hold(debt_asset_id, debt_owner, amount_to_borrow)?;

			// TODO: decide what todo do with reborrow  https://mlabs-corp.slack.com/archives/C02CRQ9KW04/p1631005365082200
			DebtIndex::<T>::insert(market_id, debt_owner, market_index);

			Ok(())
		}

		fn repay_borrow(
			market_id: &Self::MarketId,
			from: &Self::AccountId,
			beneficiary: &Self::AccountId,
			repay_amount: Option<BorrowAmountOf<Self>>,
		) -> Result<(), DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			if let Some(owed) = Self::borrow_balance_current(market_id, beneficiary)? {
				let repay_amount = repay_amount.unwrap_or(owed);
				let borrow_id = T::Vault::asset_id(&market.borrow)?;
				ensure!(repay_amount <= owed, Error::<T>::CannotRepayMoreThanBorrowAmount);
				ensure!(
					T::Currency::can_withdraw(borrow_id, &from, repay_amount).into_result().is_ok(),
					Error::<T>::CannotWithdrawFromProvidedBorrowAccount
				);
				let market_account = Self::account_id(market_id);
				ensure!(
					T::Currency::can_deposit(borrow_id, &market_account, repay_amount)
						.into_result()
						.is_ok(),
					Error::<T>::TransferFailed
				);
				let debt_asset_id = DebtMarkets::<T>::get(market_id);

				let burn_amount: u128 = T::Currency::balance(debt_asset_id, beneficiary).into();
				let burn_amount = burn_amount
					.checked_mul(LiftedFixedBalance::accuracy())
					.expect("should work for 64 currency");

				// TODO: fuzzing is must to uncover cases when sum != total
				let market_debt_reduction =
					T::MarketCurrency::balance(debt_asset_id, &market_account)
						.checked_sub(burn_amount)
						.expect("debt balance of market must be of parts of debts of borrowers");
				T::MarketCurrency::burn_from(debt_asset_id, &market_account, market_debt_reduction).expect(
					"debt balance of market must be of parts of debts of borrowers and can reduce it",
				);
				T::MarketCurrency::burn_from(debt_asset_id, beneficiary, burn_amount)
					.expect("can always burn current balance");
				T::Currency::transfer(borrow_id, from, &market_account, repay_amount, false)
					.expect("must be able to transfer because of above checks");

				// TODO: not sure why Warp V2 (Blacksmith) does that, but seems will need to revise
				// it later with some strategy
				let interest_index = BorrowIndex::<T>::get(market_id);
				DebtIndex::<T>::insert(market_id, beneficiary, interest_index);

				// we do not optimize vault here, will do it on finalize after all repays
			}

			Ok(())
		}

		fn total_borrows(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let accrued_debt =
				T::MarketCurrency::balance(debt_asset_id, &Self::account_id(market_id));
			let total_issued = T::MarketCurrency::total_issuance(debt_asset_id);
			let total_borrows = (total_issued - accrued_debt) / LiftedFixedBalance::accuracy();
			Ok((total_borrows as u64).into())
		}

		fn total_interest(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let total_interest =
				T::MarketCurrency::balance(debt_asset_id, &Self::account_id(market_id));
			Ok(((total_interest / LiftedFixedBalance::accuracy()) as u64).into())
		}

		fn total_cash(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let borrow_id = T::Vault::asset_id(&market.borrow)?;
			Ok(T::Currency::balance(borrow_id, &T::Vault::account_id(&market.borrow)))
		}

		fn update_borrows(
			market_id: &Self::MarketId,
			delta_interest_rate: Rate,
		) -> Result<(), DispatchError> {
			// we maintain original borrow principals intact on hold,
			// but accrue total borrow balance by adding to market debt balance
			// when user pays loan back, we reduce marked accrued debt
			// so no need to loop over each account -> scales to millions of users
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let total_debt = Self::total_borrows(market_id)?;

			let accrued_debt =
				T::MarketCurrency::balance(debt_asset_id, &Self::account_id(market_id));
			let total_issued = T::MarketCurrency::total_issuance(debt_asset_id);
			let total_borrows = total_issued - accrued_debt;

			let accrued = LiftedFixedBalance::from_inner(total_borrows)
				.safe_mul(&delta_interest_rate)?
				.into_inner();

			T::MarketCurrency::mint_into(debt_asset_id, &Self::account_id(market_id), accrued)?;
			Ok(())
		}

		fn calc_utilization_ratio(
			cash: &Self::Balance,
			borrows: &Self::Balance,
		) -> Result<Ratio, DispatchError> {
			Ok(composable_traits::rate_model::calc_utilization_ratio(
				(*cash).into(),
				(*borrows).into(),
			)?)
		}

		fn accrue_interest(
			market_id: &Self::MarketId,
			now: Timestamp,
		) -> Result<(), DispatchError> {
			let total_borrows = Self::total_borrows(market_id)?;
			let total_cash = Self::total_cash(market_id)?;
			let utilization_ratio = Self::calc_utilization_ratio(&total_cash, &total_borrows)?;
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let delta_time =
				now.checked_sub(Self::last_block_timestamp()).ok_or(Error::<T>::Underflow)?;
			let borrow_index =
				BorrowIndex::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;

			// pure math
			let borrow_rate = market
				.interest_rate_model
				.get_borrow_rate(utilization_ratio)
				.ok_or(Error::<T>::BorrowRateDoesNotExist)?;
			let borrow_index_new = increment_index(borrow_rate, borrow_index, delta_time)
				.and_then(|r| r.checked_add(&borrow_index))
				.ok_or(Error::<T>::Overflow)?;
			let delta_interest_rate =
				increment_borrow_rate(borrow_rate, delta_time).ok_or(Error::<T>::Overflow)?;

			BorrowIndex::<T>::insert(market_id, borrow_index_new);

			Self::update_borrows(market_id, delta_interest_rate)?;

			Ok(())
		}

		fn borrow_balance_current(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Option<BorrowAmountOf<Self>>, DispatchError> {
			let debt_asset_id =
				DebtMarkets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;

			let account_debt = DebtIndex::<T>::try_get(market_id, account);

			match account_debt {
				Ok(account_interest_index) => {
					let principal = T::MarketCurrency::balance_on_hold(debt_asset_id, account);
					let market_interest_index = BorrowIndex::<T>::try_get(market_id)
						.map_err(|_| Error::<T>::BorrowIndexDoesNotExist)?;

					let balance = borrow_from_principal::<T>(
						((principal / LiftedFixedBalance::accuracy()) as u64).into(),
						market_interest_index,
						account_interest_index,
					)?;

					Ok(balance.map(Into::into))
				}
				// no active borrow on  market for given account
				Err(()) => Ok(Some(BorrowAmountOf::<Self>::zero())),
			}
		}

		fn collateral_of_account(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			AccountCollateral::<T>::try_get(market_id, account)
				.map_err(|_| Error::<T>::MarketCollateralWasNotDepositedByAccount.into())
		}

		fn collateral_required(
			_market_id: &Self::MarketId,
			_borrow_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn get_borrow_limit(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let collateral_balance: LiftedFixedBalance =
				AccountCollateral::<T>::try_get(market_id, account).unwrap_or_default().into();

			if collateral_balance > LiftedFixedBalance::zero() {
				let collateral_price: LiftedFixedBalance =
					T::Oracle::get_price(&market.collateral)?.0.into();
				let borrow_asset = T::Vault::asset_id(&market.borrow)?;
				let borrow_price = T::Oracle::get_price(&borrow_asset)?.0;
				let borrower_balance_with_interest =
					Self::borrow_balance_current(market_id, account)?.unwrap_or_default();

				let borrower = BorrowerData {
					collateral_balance: collateral_balance.into(),
					collateral_price: collateral_price.into(),
					borrower_balance_with_interest: borrower_balance_with_interest.into(),
					borrow_price: borrow_price.into(),
					collateral_factor: market.collateral_factor,
				};

				return Ok(borrower
					.borrow_for_collateral()
					.map_err(|_| Error::<T>::NotEnoughCollateralToBorrowAmount)?
					.checked_mul_int(1u64)
					.ok_or(ArithmeticError::Overflow)?
					.into());
			}

			Ok(Self::Balance::zero())
		}

		fn deposit_collateral(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
			amount: CollateralLpAmountOf<Self>,
		) -> Result<(), DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let market_account = Self::account_id(&market_id);
			ensure!(
				T::Currency::can_withdraw(market.collateral, &account, amount)
					.into_result()
					.is_ok(),
				Error::<T>::TransferFailed
			);
			ensure!(
				T::Currency::can_deposit(market.collateral, &market_account, amount)
					== DepositConsequence::Success,
				Error::<T>::TransferFailed
			);

			AccountCollateral::<T>::try_mutate(market_id, account, |collateral_balance| {
				let new_collateral_balance = (*collateral_balance)
					.checked_add(&amount)
					.ok_or(Error::<T>::Overflow.into())?;
				*collateral_balance = new_collateral_balance;
				Result::<(), Error<T>>::Ok(())
			})?;
			T::Currency::transfer(market.collateral, account, &market_account, amount, true)
				.expect("impossible; qed;");
			Ok(())
		}

		fn withdraw_collateral(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
			amount: CollateralLpAmountOf<Self>,
		) -> Result<(), DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;

			let (collateral_price, _) = T::Oracle::get_price(&market.collateral)?;

			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let collateral_balance: LiftedFixedBalance =
				AccountCollateral::<T>::try_get(market_id, account).unwrap_or_default().into();

			let borrow_asset = T::Vault::asset_id(&market.borrow)?;
			let borrow_price = T::Oracle::get_price(&borrow_asset)?.0;
			let borrower_balance_with_interest =
				Self::borrow_balance_current(market_id, account)?.unwrap_or_default();

			let borrower = BorrowerData {
				collateral_balance: collateral_balance.into(),
				collateral_price: collateral_price.into(),
				borrower_balance_with_interest: borrower_balance_with_interest.into(),
				borrow_price: borrow_price.into(),
				collateral_factor: market.collateral_factor,
			};

			let withdrawable_collateral_value = borrower
				.collateral_over_borrow()?
				.checked_mul_int(1u64)
				.ok_or(Error::<T>::Overflow)?
				.into();

			let collateral_value =
				amount.checked_mul(&collateral_price).ok_or(Error::<T>::Overflow)?;

			ensure!(
				collateral_value <= withdrawable_collateral_value,
				Error::<T>::NotEnoughCollateral
			);

			let market_account = Self::account_id(&market_id);
			ensure!(
				T::Currency::can_deposit(market.collateral, &account, amount)
					== DepositConsequence::Success,
				Error::<T>::TransferFailed
			);
			ensure!(
				T::Currency::can_withdraw(market.collateral, &market_account, amount)
					.into_result()
					.is_ok(),
				Error::<T>::TransferFailed
			);

			AccountCollateral::<T>::try_mutate(market_id, account, |collateral_balance| {
				let new_collateral_balance =
					(*collateral_balance).checked_sub(&amount).ok_or(Error::<T>::Underflow)?;
				*collateral_balance = new_collateral_balance;
				Result::<(), Error<T>>::Ok(())
			})?;
			T::Currency::transfer(market.collateral, &market_account, account, amount, true)
				.expect("impossible; qed;");
			Ok(())
		}
	}

	// If borrowBalance = 0 then borrow index is likely also 0.
	// Rather than failing the calculation with a division by 0, we immediately return 0 in
	// this case.
	fn borrow_from_principal<T: Config>(
		principal: <T as Config>::Balance,
		market_interest_index: Ratio,
		account_interest_index: Ratio,
	) -> Result<Option<u64>, DispatchError> {
		if principal.is_zero() {
			return Ok(None);
		}
		let principal: LiftedFixedBalance = principal.into();
		let balance = principal
			.checked_mul(&market_interest_index)
			.and_then(|from_start_total| from_start_total.checked_div(&account_interest_index))
			.and_then(|x| x.checked_mul_int(1u64))
			.ok_or(ArithmeticError::Overflow)?;
		Ok(Some(balance))
	}
}
