#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicingr,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    // TODO: @mikolaichuk: return dead_code	
    //dead_code,
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
#![allow(missing_docs)]
pub use pallet::*;

#[cfg(test)]
pub mod mocks;
#[cfg(test)]
pub mod tests;

pub mod currency;
pub mod helpers;
pub mod impls;
pub mod types;
pub mod validation;

#[frame_support::pallet]
pub mod pallet {
	use crate::types::{
		LoanConfigOf, LoanId, LoanInputOf, MarketInfoOf, MarketInputOf, PaymentsOutcomes,
		Timestamp,
	};
	use codec::{Codec, FullCodec};
	use composable_traits::{
		currency::CurrencyFactory,
		defi::{DeFiComposableConfig, DeFiEngine},
		liquidation::Liquidation,
		oracle::Oracle,
		undercollateralized_loans::UndercollateralizedLoans,
		vault::StrategicVault,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungible::{Inspect as NativeInspect, Transfer as NativeTransfer},
			fungibles::{InspectHold, Mutate, MutateHold, Transfer},
			UnixTime,
		},
		transactional, PalletId,
	};
	use frame_system::{
		ensure_none, ensure_signed, offchain::SendTransactionTypes, pallet_prelude::OriginFor,
	};
	use scale_info::TypeInfo;
	use sp_runtime::{
		traits::One,
		transaction_validity::{TransactionSource, TransactionValidity},
	};
	use sp_std::{collections::btree_set::BTreeSet, fmt::Debug, ops::AddAssign};

	impl<T: Config> DeFiEngine for Pallet<T> {
		type MayBeAssetId = <T as DeFiComposableConfig>::MayBeAssetId;
		type Balance = <T as DeFiComposableConfig>::Balance;
		type AccountId = <T as frame_system::Config>::AccountId;
	}

	#[pallet::config]
	pub trait Config:
		frame_system::Config + SendTransactionTypes<Call<Self>> + DeFiComposableConfig
	{
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The asset used to pay for rent and other fees.
		type NativeCurrency: NativeTransfer<Self::AccountId, Balance = Self::Balance>
			+ NativeInspect<Self::AccountId, Balance = Self::Balance>;

		/// The `id`s to be used for the [`Vault`][Config::Vault].
		type VaultId: Clone + Codec + MaxEncodedLen + Debug + PartialEq + Default + Parameter;

		/// The Vault used to store the borrow asset.
		type Vault: StrategicVault<
			VaultId = Self::VaultId,
			AssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
		>;

		type Oracle: Oracle<
			AssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			Balance = <Self as DeFiComposableConfig>::Balance,
			Timestamp = <Self as frame_system::Config>::BlockNumber,
		>;

		type MultiCurrency: Transfer<
				Self::AccountId,
				Balance = Self::Balance,
				AssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			> + Mutate<
				Self::AccountId,
				Balance = Self::Balance,
				AssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			> + MutateHold<
				Self::AccountId,
				Balance = Self::Balance,
				AssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			> + InspectHold<
				Self::AccountId,
				Balance = Self::Balance,
				AssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			>;

		type CurrencyFactory: CurrencyFactory<
			<Self as DeFiComposableConfig>::MayBeAssetId,
			Self::Balance,
		>;

		type LiquidationStrategyId: Parameter + Default + PartialEq + Clone + Debug + TypeInfo;

		type PalletId: Get<PalletId>;

		type LoanId: Get<LoanId>;

		type Liquidation: Liquidation<
			MayBeAssetId = Self::MayBeAssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			LiquidationStrategyId = Self::LiquidationStrategyId,
		>;

		// TODO: @mikolaichuk: use u128 instead.
		type Counter: AddAssign
			+ One
			+ FullCodec
			+ Copy
			+ PartialEq
			+ PartialOrd
			+ Debug
			+ Default
			+ TypeInfo;

		type UnixTime: UnixTime;
		type MaxMarketsCounterValue: Get<Self::Counter>;
		type MaxLoansPerMarketCounterValue: Get<Self::Counter>;
		// Each payments schedule can not have more than this amount of payments.
		type MaxPaymentsPerSchedule: Get<u32>;
		type OracleMarketCreationStake: Get<Self::Balance>;
		type CheckPaymentsBatchSize: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type MarketsCounterStorage<T: Config> = StorageValue<_, T::Counter, ValueQuery>;

	#[pallet::storage]
	pub type LoansCounterStorage<T: Config> = StorageValue<_, T::Counter, ValueQuery>;

	// TODO: @mikolaichuk: Checke nonce type in composable-support.
	#[pallet::storage]
	pub type LoansPerMarketCounterStorage<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, T::Counter, ValueQuery>;

	#[pallet::storage]
	pub type CurrentDateStorage<T: Config> = StorageValue<_, Timestamp, ValueQuery>;

	// Markets storage. AccountId is id of market's account.
	#[pallet::storage]
	pub type MarketsStorage<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, MarketInfoOf<T>, OptionQuery>;

	// Loans storage. AccountId is id of loan's account.
	#[pallet::storage]
	pub type LoansStorage<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, LoanConfigOf<T>, OptionQuery>;

	// Use hashmap as a set.
	#[pallet::storage]
	pub type NonActiveLoansStorage<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, (), OptionQuery>;

	// Storage keeps accounts ids of loans which payments were already processed today.
	// Prevents double checking and subsiquent unreasonable liquidation.
	// Use hashmap as a set.
	#[pallet::storage]
	pub type ProcessedLoansStorage<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, (), OptionQuery>;

	// Maps market's account id to market's debt token
	#[pallet::storage]
	pub type DebtTokenForMarketStorage<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		<T as DeFiComposableConfig>::MayBeAssetId,
		OptionQuery,
	>;

	// Payments schedule storage.
	// Maps payment moment and loan account id to interest rate for this payment.
	#[pallet::storage]
	pub type ScheduleStorage<T: Config> =
		StorageMap<_, Twox64Concat, Timestamp, BTreeSet<T::AccountId>, ValueQuery>;

	// TODO: @mikolaichuk: storages for borrowers' strikes (local for particular market and global
	// for all markets).
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MarketCreated { market_info: MarketInfoOf<T> },
		LoanCreated { loan_config: LoanConfigOf<T> },
		LoanContractExecuted { loan_config: LoanConfigOf<T> },
		LoanTerminated { loan_config: LoanConfigOf<T> },
		LoanClosed { loan_config: LoanConfigOf<T> },
		NonActivatedExpiredLoansTerminated { loans_ids: Vec<T::AccountId> },
		LoanSentToLiquidation { loan_config: LoanConfigOf<T> },
		// TODO: @mikolaichuk: add loan information and amount by itself.
		SomeAmountRepaid,
		LoanPaymentWasChecked { loan_config: LoanConfigOf<T> },
	}

	#[allow(missing_docs)]
	#[pallet::error]
	pub enum Error<T> {
		// Amount of markets is bounded.
		MaxMarketsReached,
		// We can not work with zero prices.
		PriceOfInitialBorrowVaultShouldBeGreaterThanZero,
		// If wrong account id of market or loan was provided.
		MarketDoesNotExist,
		LoanDoesNotExistOrWasActivated,
		// Only market manager account allowed to create loans for the market.
		NonAuthorizedToCreateLoan,
		// Nont-authorized user tried to execute loan contract.
		NonAuthorizedToExecuteContract,
		// There is no loan with such account id in the storage.
		LoanNotFound,
		// Out-of-range number of seconds in provided timestamp.
		InvalidTimestamp,
		// When borrower tried to activate a loan after first payment day.
		LoanContractIsExpired,
		// Tis should not happens.
		// Error added for debug.
		CollateralCanNotBeTransferedBackToTheBorrowersAccount,
		// When we try to retrieve interest rate for the date which is not present in the payment
		// schedule for particular loan.
		MomentNotFoundInSchedule,
	}

	#[pallet::genesis_config]
	#[derive(Default)]
	pub struct GenesisConfig {}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			let now = T::UnixTime::now().as_secs() as Timestamp;
			let current_date_timestamp = crate::Pallet::<T>::get_date_aligned_timestamp(now);
			CurrentDateStorage::<T>::put(current_date_timestamp);
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

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		// TODO: @mikolaichuk: add weights calculation
        // TODO: @mikolaichuk:  treat untreated loans on chain.
        //                      then clen yesterday schdule.
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			Self::treat_vaults_balance(block_number);
			let now = Self::now();
			let stored_current_day_timestamp = CurrentDateStorage::<T>::get();
			// Check if date is changed.
			let stored_date = Self::get_date_from_timestamp(stored_current_day_timestamp);
			let date = Self::get_date_from_timestamp(now);
			if stored_date < date {
				let current_date_aligned_timestamp = Self::get_date_aligned_timestamp(now);
				CurrentDateStorage::<T>::put(current_date_aligned_timestamp);
				// Terminate loans which were not activated by borrower before first payment date
				// once a day.
				Self::terminate_non_activated_expired_loans(current_date_aligned_timestamp);
			}
			1000
		}

		fn offchain_worker(_block_number: T::BlockNumber) {
			use sp_runtime::offchain::{storage_lock::StorageLock, Duration};
			let current_date_timestamp = Self::get_current_date_timestamp();
			let next_date_aligned_timestemp =
				Self::get_next_date_aligned_timestamp(current_date_timestamp);
			let mut daily_lock = StorageLock::with_deadline(
				b"undercollateralized_loans::offchain_worker_lock",
				// Type conversion is safe here since we do not use dates before the epoche.
				Duration::from_millis(next_date_aligned_timestemp as u64 * 1000),
			);
			match daily_lock.try_lock() {
				Ok(_) => Self::sync_offchain_worker(current_date_timestamp),
				Err(_) => (),
			};
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;
		fn validate_unsigned(source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			// Check if transaction is local.
			match source {
				TransactionSource::Local | TransactionSource::InBlock => (),
				_ => return InvalidTransaction::Call.into(),
			};
			// Check if call is allowed.
			match call {
				Call::process_checked_payments { .. } => (),
				_ => return InvalidTransaction::Call.into(),
			};
			Ok(ValidTransaction::default())
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1000)]
		#[transactional]
		pub fn create_market(
			origin: OriginFor<T>,
			input: MarketInputOf<T>,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let market_info =
				<Self as UndercollateralizedLoans>::create_market(who.clone(), input, keep_alive)?;
			let event = Event::<T>::MarketCreated { market_info };
			Self::deposit_event(event);
			Ok(())
		}

		#[pallet::weight(1000)]
		#[transactional]
		pub fn create_loan(origin: OriginFor<T>, input: LoanInputOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(
				Self::is_market_manager_account(&who, &input.market_account_id)?,
				Error::<T>::NonAuthorizedToCreateLoan,
			);
			let loan_config = <Self as UndercollateralizedLoans>::create_loan(input)?;
			Self::deposit_event(Event::<T>::LoanCreated { loan_config });
			Ok(())
		}

		#[pallet::weight(1000)]
		#[transactional]
		pub fn borrow(
			origin: OriginFor<T>,
			loan_account: T::AccountId,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let loan_config =
				<Self as UndercollateralizedLoans>::borrow(who, loan_account, keep_alive)?;
			Self::deposit_event(Event::<T>::LoanContractExecuted { loan_config });
			Ok(())
		}

		#[pallet::weight(1000)]
		#[transactional]
		pub fn repay(
			origin: OriginFor<T>,
			loan_account: T::AccountId,
			repay_amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as UndercollateralizedLoans>::repay(who, loan_account, repay_amount, keep_alive)?;
			Self::deposit_event(Event::<T>::SomeAmountRepaid);
			Ok(())
		}
        
        // TODO: @mikolaichuk: check that timestamp is today. 
		#[pallet::weight(1000)]
		pub fn process_checked_payments(
			origin: OriginFor<T>,
			outcomes: PaymentsOutcomes<T>,
			timestamp: Timestamp,
		) -> DispatchResult {
			let who = ensure_none(origin)?;

			Ok(())
		}
	}
}
