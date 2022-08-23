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
		Counter, LoanInfoOf, LoanConfigOf, LoanId, LoanInputOf, MarketInfoOf, MarketInputOf, PaymentsOutcomes,
		Timestamp,
	};
	use codec::Codec;
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
	use sp_runtime::
		transaction_validity::{
			TransactionPriority, TransactionSource, TransactionValidity, ValidTransaction,
	};
	use sp_std::{
        collections::{
            btree_map::BTreeMap, btree_set::BTreeSet,
        },
        fmt::Debug
    };

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

		type UnixTime: UnixTime;
		type MaxMarketsCounterValue: Get<Counter>;
		type MaxLoansPerMarketCounterValue: Get<Counter>;
		type OracleMarketCreationStake: Get<Self::Balance>;
		// Amount of loans which can be processed within one tansaction submitted by off-chain
		// worker.
		type CheckPaymentsBatchSize: Get<u32>;
		// Amount of loans which can be processed within one tansaction submitted by off-chain
		// worker.
		type CheckNonActivatedLoansBatchSize: Get<u32>;
		// Bounds are used during validation.
		type WhiteListBound: Get<u32>;
		type ScheduleBound: Get<u32>;
        // Borrower may fail repayment not more then this amount of times. 
        // After this he will placed in the blacklist.
        type MaxRepyamentFails: Get<u128>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Counting created market. Counter's value is used to generate market's id value.
	#[pallet::storage]
	pub type MarketsCounterStorage<T: Config> = StorageValue<_, Counter, ValueQuery>;

	// Counting created loans. Counter's value is used to generate loan's id value.
	#[pallet::storage]
	pub type LoansCounterStorage<T: Config> = StorageValue<_, Counter, ValueQuery>;

	// Counting amount of loans created within each market.
	#[pallet::storage]
	pub type LoansPerMarketCounterStorage<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, Counter, ValueQuery>;

	#[pallet::storage]
	pub type CurrentDateStorage<T: Config> = StorageValue<_, Timestamp, ValueQuery>;

	// Storage keeps accounts ids of loans which payments were already processed today.
	// Prevents double checking and subsiquent unreasonable liquidation.
	#[pallet::storage]
	pub type ProcessedLoansStorage<T: Config> = StorageValue<_, BTreeSet<T::AccountId>, ValueQuery>;

	// Markets storage. AccountId is id of market's account.
	#[pallet::storage]
	pub type MarketsStorage<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, MarketInfoOf<T>, OptionQuery>;

   // Loans storage. AccountId is id of loan's account.
	#[pallet::storage]
	pub type LoansStorage<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, LoanInfoOf<T>, OptionQuery>;
       
    
	// Use hashmap as a set.
	#[pallet::storage]
	pub type NonActiveLoansStorage<T: Config> =
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
	// Maps payment moment  => [loan account id => payment amount] for this payment.
	#[pallet::storage]
	pub type ScheduleStorage<T: Config> =
		StorageMap<_, Twox64Concat, Timestamp, BTreeMap<T::AccountId, T::Balance>, ValueQuery>;

    // Storage for blacklisted borrowers accounts ids which have failed significant amount of
    // payments. If borrower is within the list it is not possible to create new loan with it's
    // account's id.	
    #[pallet::storage]
	pub type BlackListPerMakretStorage<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BTreeSet<T::AccountId>, ValueQuery>;
    
    // Storage holds failed payments counters for pairs (Market, Borrower). When threshold value is achived, 
    // borrowers id is added to BlackListPerMarketStorage and BlackListStorage.
    #[pallet::storage]
    pub type FailsCounterStorage<T: Config> = 
        StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, T::AccountId, Counter, ValueQuery>;

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
		// Amount of loans per market is bounded.
		MaxLoansPerMarketReached,
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
        // Borrower in question is blacklisted due to payments failing.
        BlacklistedBorrowerAccount
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
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			Self::treat_vaults_balance(block_number);
			let now = Self::now();
			let stored_date_timestamp = CurrentDateStorage::<T>::get();
			// Check if date is changed.
			let stored_date = Self::get_date_from_timestamp(stored_date_timestamp);
			let date = Self::get_date_from_timestamp(now);
			if stored_date < date {
				// Check if we have loans which were not processed via off-chain worker,
				// and process them.
				Self::last_chance_processing(stored_date_timestamp);
				// Remove yesterday schedule.
				crate::ScheduleStorage::<T>::remove(stored_date_timestamp);
				// Set up current date.
				let current_date_aligned_timestamp = Self::get_date_aligned_timestamp(now);
				CurrentDateStorage::<T>::put(current_date_aligned_timestamp);
			}
			1000
		}

		fn offchain_worker(_block_number: T::BlockNumber) {
			use sp_runtime::offchain::{storage_lock::StorageLock, Duration};
			let current_date_timestamp = Self::get_current_date_timestamp();
			let next_date_aligned_timestemp =
				Self::get_next_date_aligned_timestamp(current_date_timestamp);
		    
            // Create daily lock for payments and expired loans checking.	
            let mut daily_lock = StorageLock::with_deadline(
				b"UndercollateralizedLoansOffchainWorkerLock",
				// Type conversion is safe here since we do not use dates before the epoche.
				Duration::from_millis(next_date_aligned_timestemp as u64 * 1000),
			);
			
            // Run procedures on daily basis.
			match daily_lock.try_lock() {
				Ok(_) => Self::sync_offchain_worker(current_date_timestamp),
                Err(_) => (),
			};
		}
	}

	// Unsigned transactions are disabled by default.
	// This implimentation allow us to use unsigned transactions.
	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;
		// This validate function gurantee that only locall calls (i.e. transcations submitted via
		// off-chain worker) are allowed.
		fn validate_unsigned(source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			// Check if transaction is local.
			match source {
				TransactionSource::Local | TransactionSource::InBlock => (),
				_ => return InvalidTransaction::Call.into(),
			};
			// Only methods mentioned here will be accessible for unsigned transactions.
			match call {
				Call::process_checked_payments { .. } => (),
				Call::remove_loans { .. } => (),
				_ => return InvalidTransaction::Call.into(),
			};
			ValidTransaction::with_tag_prefix("UndercollateralizedLoansOffchainWorker")
			    // Setup maximum priority since we want to run such transaction ASAP.	
                .priority(TransactionPriority::MAX)
			    // We want to propagate such transaction to other nodes.	
                .propagate(true)
				.build()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Market creation. Note that manager has to have initial amount of borrow asset which will
		// be deposited to market's account. Makret manager provides whitelist of borrowers who
		// are allowed to borrow money from the market.
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

		// Loan can be created only by market's manager. Contract terms should be previously
		// discussed with borrower. After loan is been created, it's config placed in the loan's
		// storage. The loan is marked as non-activated until borrower make borrow.To borrow money,
		// borrower should be provided with loan's account id.
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

		// To borrow money, user should provide loan's account id.
		// User will be allowed to borrow if his account is mentioned as borrower's
		// account in the loan's configuration. Borrower has to have sufficient amount of
		// collateral on his account. This collateral will be transferred to the loan's account.
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

		// Borrower have to repay his loan as per payment schedule which is mentioned in the loan's
		// configuration. Assets should be transferred on the loan's account
		// before payment date.
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

		// This method supposed to be called from off-chain worker.
		#[pallet::weight(0)]
		pub fn process_checked_payments(
			origin: OriginFor<T>,
			outcomes: PaymentsOutcomes<T>,
		) -> DispatchResult {
			ensure_none(origin)?;
			Self::do_process_checked_payments(outcomes);
			Ok(())
		}

		// This method supposed to be called from off-chain worker.
		#[pallet::weight(0)]
		pub fn remove_loans(
			origin: OriginFor<T>,
			loans_accounts_ids: Vec<T::AccountId>,
		) -> DispatchResult {
			ensure_none(origin)?;
			Self::do_remove_non_activated_loans(loans_accounts_ids);
			Ok(())
		}
	}
}
