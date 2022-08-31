//! Lending pallet
#![cfg_attr(
	not(any(test, feature = "runtime-benchmarks")),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic,
		clippy::identity_op,
	)
)] // allow in tests
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::unseparated_literal_suffix)]
#![deny(
	unused_imports,
	clippy::useless_conversion,
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

pub mod validation;
pub mod weights;
pub use crate::weights::WeightInfo;

pub mod crypto;
mod helpers;
mod impls;
mod models;
mod types;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

#[cfg(any(feature = "runtime-benchmarks", test))]
pub mod currency;

/// Various helpers used in the implementation of [`Lending::repay_borrow`].
///
/// [`Lending::repay_borrow`]: composable_traits::lending::Lending::repay_borrow
mod repay_borrow;

#[frame_support::pallet]
pub mod pallet {
    // ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies
	// ----------------------------------------------------------------------------------------------------о
    
    use crate::weights::WeightInfo;
	pub(crate) use crate::types::MarketAssets;
	pub use crate::{
		helpers::swap,
		types::{MarketIdInner, MarketId},
	};
	use composable_traits::{
		currency::CurrencyFactory,
		defi::{DeFiComposableConfig, *},
		lending::{Lending, MarketConfig, RepayStrategy, UpdateInput, CreateInput},
		liquidation::Liquidation,
		oracle::Oracle,
		time::Timestamp,
		vault::StrategicVault,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungible::{Inspect as NativeInspect, Transfer as NativeTransfer},
			fungibles::{InspectHold, Mutate, MutateHold, Transfer},
			UnixTime,
		},
		transactional,
		weights::{WeightToFee, WeightToFeePolynomial},
		PalletId,
	};
	use frame_system::{
		offchain::{AppCrypto, CreateSignedTransaction},
		pallet_prelude::*,
	};
	use sp_core::crypto::KeyTypeId;
	use sp_runtime::KeyTypeId as CryptoKeyTypeId;
	use sp_std::{fmt::Debug, vec::Vec};
	use codec::Codec;
	pub use crate::crypto;
	
    // ----------------------------------------------------------------------------------------------------
	//                                    Declaration Of The Pallet Type
	// ----------------------------------------------------------------------------------------------------

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

    // ----------------------------------------------------------------------------------------------------
	//                                             Config Trait
	// ----------------------------------------------------------------------------------------------------
    
    // Configure the pallet by specifying the parameters and types on which it depends.   
    #[pallet::config]
	pub trait Config:
		CreateSignedTransaction<Call<Self>> + frame_system::Config + DeFiComposableConfig
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Oracle: Oracle<
			AssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			Balance = <Self as DeFiComposableConfig>::Balance,
			Timestamp = <Self as frame_system::Config>::BlockNumber,
		>;

		/// The `id`s to be used for the [`Vault`][Config::Vault].
		type VaultId: Clone + Codec + MaxEncodedLen + Debug + PartialEq + Default + Parameter;

		/// The Vault used to store the borrow asset.
		type Vault: StrategicVault<
			VaultId = Self::VaultId,
			AssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
		>;

		type CurrencyFactory: CurrencyFactory<
			<Self as DeFiComposableConfig>::MayBeAssetId,
			Self::Balance,
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

		type Liquidation: Liquidation<
			MayBeAssetId = Self::MayBeAssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			LiquidationStrategyId = Self::LiquidationStrategyId,
		>;

		type UnixTime: UnixTime;

		/// The maximum amount of markets that can be open at once.
		type MaxMarketCount: Get<u32>;

		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		type WeightInfo: WeightInfo;

		/// Id of proxy to liquidate
		type LiquidationStrategyId: Parameter + Default + PartialEq + Clone + Debug + TypeInfo;

		/// Minimal price of borrow asset in Oracle price required to create.
		/// Examples, 100 USDC.
		/// Creators puts that amount and it is staked under Vault account.
		/// So he does not owns it anymore.
		/// So borrow is both stake and tool to create market.
		///
		/// # Why not pure borrow amount minimum?
		///
		/// Borrow may have very small price. Will imbalance some markets on creation.
		///
		/// # Why not native parachain token?
		///
		/// Possible option. But I doubt closing market as easy as transferring back rent.  So it is
		/// not exactly platform rent only.
		///
		/// # Why borrow amount priced by Oracle?
		///
		/// We depend on Oracle to price in Lending. So we know price anyway.
		/// We normalized price over all markets and protect from spam all possible pairs equally.
		/// Locking borrow amount ensures manager can create market with borrow assets, and we force
		/// him to really create it.
		///
		/// This solution forces to have amount before creating market.
		/// Vault can take that amount if reconfigured so, but that may be changed during runtime
		/// upgrades.
		#[pallet::constant]
		type OracleMarketCreationStake: Get<Self::Balance>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		type NativeCurrency: NativeTransfer<Self::AccountId, Balance = Self::Balance>
			+ NativeInspect<Self::AccountId, Balance = Self::Balance>;

		/// The maximum size of batch for liquidation.
		type MaxLiquidationBatchSize: Get<u32>;

		/// Convert a weight value into a deductible fee based on the currency type.
		type WeightToFee: WeightToFeePolynomial<Balance = Self::Balance>
			+ WeightToFee<Balance = Self::Balance>;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                             Pallet Types Aliases
	// ----------------------------------------------------------------------------------------------------
    
    /// Simple type alias around [`MarketConfig`] for this pallet.
	pub(crate) type MarketConfigOf<T> = MarketConfig<
		<T as Config>::VaultId,
		<T as DeFiComposableConfig>::MayBeAssetId,
		<T as frame_system::Config>::AccountId,
		<T as Config>::LiquidationStrategyId,
		<T as frame_system::Config>::BlockNumber,
	>;
    /// A convenience wrapper around [`CreateInput`].
    pub type CreateInputOf<T> = CreateInput<
	    <T as Config>::LiquidationStrategyId,
	    <T as DeFiComposableConfig>::MayBeAssetId,
	    <T as frame_system::Config>::BlockNumber,
    >;
	
    // ----------------------------------------------------------------------------------------------------
	//                                             Pallet Constants 
	// ----------------------------------------------------------------------------------------------------
    
    pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"lend");
	pub const CRYPTO_KEY_TYPE: CryptoKeyTypeId = CryptoKeyTypeId(*b"lend");

    // ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Storage
	// ----------------------------------------------------------------------------------------------------
        
	/// Lending instances counter
	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // MarketId implements Default, so ValueQuery is ok here. REVIEW: Should it?
	pub type LendingCount<T: Config> = StorageValue<_, MarketId, ValueQuery>;

	/// Indexed lending instances. Maps markets to their respective [`MarketConfig`].
	///
	/// ```text
	/// MarketId -> MarketConfig
	/// ```
	#[pallet::storage]
	pub type Markets<T: Config> =
		StorageMap<_, Twox64Concat, MarketId, MarketConfigOf<T>, OptionQuery>;

	/// Maps markets to their corresponding debt token.
	///
	/// ```text
	/// MarketId -> debt asset
	/// ```
	///
	/// See [this clickup task](task) for a more in-depth explanation.
	///
	/// [task]: <https://sharing.clickup.com/20465559/t/h/27y9y84/15U30TKC3THPZYT>
	#[pallet::storage]
	pub type DebtTokenForMarket<T: Config> = StorageMap<
		_,
		Twox64Concat,
		MarketId,
		<T as DeFiComposableConfig>::MayBeAssetId,
		OptionQuery,
	>;

	/// at which lending index account did borrowed.
	/// if first borrow: market index when the borrowed occured
	/// if additional borrow: market index adjusted wrt the previous index
	#[pallet::storage]
	pub type DebtIndex<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		MarketId,
		Twox64Concat,
		T::AccountId,
		ZeroToOneFixedU128,
		OptionQuery,
	>;

	/// Latest timestamp at which account borrowed from market.
	///
	/// (Market, Account) -> Timestamp
	#[pallet::storage]
	pub type BorrowTimestamp<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		MarketId,
		Twox64Concat,
		T::AccountId,
		Timestamp,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type BorrowRent<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		MarketId,
		Twox64Concat,
		T::AccountId,
		T::Balance,
		OptionQuery,
	>;

	/// market borrow index
	// REVIEW: ZeroToOneFixedU128?
	#[pallet::storage]
	pub type BorrowIndex<T: Config> =
		StorageMap<_, Twox64Concat, MarketId, ZeroToOneFixedU128, OptionQuery>;

	/// (Market, Account) -> Collateral
	#[pallet::storage]
	pub type AccountCollateral<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		MarketId,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
		OptionQuery,
	>;

	/// The timestamp of the previous block or defaults to timestamp at genesis.
	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // LastBlockTimestamp is set on genesis (see below) so it will always be set.
	pub type LastBlockTimestamp<T: Config> = StorageValue<_, Timestamp, ValueQuery>;

    // ----------------------------------------------------------------------------------------------------
	//                                            Runtime Events
	// ----------------------------------------------------------------------------------------------------
	
    #[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when new lending market is created.
		MarketCreated {
			market_id: MarketId,
			vault_id: T::VaultId,
			manager: T::AccountId,
			currency_pair: CurrencyPair<T::MayBeAssetId>,
		},
		MarketUpdated {
			market_id: MarketId,
			input: UpdateInput<T::LiquidationStrategyId, <T as frame_system::Config>::BlockNumber>,
		},
		/// Event emitted when collateral is deposited.
		CollateralDeposited { sender: T::AccountId, market_id: MarketId, amount: T::Balance },
		/// Event emitted when collateral is withdrawed.
		CollateralWithdrawn { sender: T::AccountId, market_id: MarketId, amount: T::Balance },
		/// Event emitted when user borrows from given market.
		Borrowed { sender: T::AccountId, market_id: MarketId, amount: T::Balance },
		/// Event emitted when user repays borrow of beneficiary in given market.
		BorrowRepaid {
			sender: T::AccountId,
			market_id: MarketId,
			beneficiary: T::AccountId,
			amount: T::Balance,
		},
		/// Event emitted when a liquidation is initiated for a loan.
		LiquidationInitiated { market_id: MarketId, borrowers: Vec<T::AccountId> },
		/// Event emitted to warn that loan may go under collaterlized soon.
		MayGoUnderCollateralizedSoon { market_id: MarketId, account: T::AccountId },
	}

    // ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Errors
	// ----------------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		/// The market could not be found
		MarketDoesNotExist,

		MarketCollateralWasNotDepositedByAccount,

		/// The collateral factor for a market must be more than one.
		CollateralFactorMustBeMoreThanOne,

		MarketIsClosing,
		InvalidTimestampOnBorrowRequest,
		NotEnoughBorrowAsset,

		/// Attempted to withdraw more collateral than the account has in the market.
		NotEnoughCollateralToWithdraw,
		/// The market would go under collateralized if the requested amount of collateral was
		/// withdrawn.
		WouldGoUnderCollateralized,
		NotEnoughCollateralToBorrow,

		// TODO: This can probably be removed, it was only used in
		// ensure!(can_{withdraw/transfer/etc}) checks
		TransferFailed,

		BorrowRateDoesNotExist,

		/// Borrow and repay in the same block (flashloans) are not allowed.
		BorrowAndRepayInSameBlockIsNotSupported,

		BorrowDoesNotExist,

		ExceedLendingCount,

		BorrowerDataCalculationFailed,
		/// Attempted to update a market owned by someone else.
		Unauthorized,
		NotEnoughRent,
		/// borrow assets should have enough value as per oracle
		PriceOfInitialBorrowVaultShouldBeGreaterThanZero,

		/// A market with a borrow balance of `0` was attempted to be repaid.
		CannotRepayZeroBalance,
		/// Cannot repay the total amount of debt when partially repaying.
		CannotRepayMoreThanTotalDebt,

		BorrowRentDoesNotExist,

		PriceTooOld,

		// Open market collateral factor increase would allow market manager to liquidate all
		// positions in any time.
		CannotIncreaseCollateralFactorOfOpenMarket,

		// If Vault is unbalanced we can not borrow from it, since
		// we do not know how many asset it needs to balance.
		CannotBorrowFromMarketWithUnbalancedVault,
	}

    // ----------------------------------------------------------------------------------------------------
	//                                                Hooks
	// ----------------------------------------------------------------------------------------------------
	
    #[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			let mut weight: Weight = 0;
			let call_counters = Self::initialize_block(block_number);
			let one_read = T::DbWeight::get().reads(1);
			weight += u64::from(call_counters.now) * <T as Config>::WeightInfo::now();
			weight += u64::from(call_counters.read_markets) * one_read;
			weight += u64::from(call_counters.accrue_interest) *
				<T as Config>::WeightInfo::accrue_interest(1);
			weight += u64::from(call_counters.account_id) * <T as Config>::WeightInfo::account_id();
			weight += u64::from(call_counters.available_funds) *
				<T as Config>::WeightInfo::available_funds();
			weight += u64::from(call_counters.handle_withdrawable) *
				<T as Config>::WeightInfo::handle_withdrawable();
			weight += u64::from(call_counters.handle_depositable) *
				<T as Config>::WeightInfo::handle_depositable();
			weight += u64::from(call_counters.handle_must_liquidate) *
				<T as Config>::WeightInfo::handle_must_liquidate();
			weight
		}

		fn offchain_worker(_block_number: T::BlockNumber) {
			log::info!("Off-chain worker running");
			Self::do_offchain_worker(_block_number)
		}
	}
    
    // ----------------------------------------------------------------------------------------------------
	//                                         Genesis Configuration
	// ----------------------------------------------------------------------------------------------------

	#[pallet::genesis_config]
	#[derive(Default)]
	pub struct GenesisConfig {}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			let now = T::UnixTime::now().as_secs();
			// INVARIANT: Don't remove this, required to use `ValueQuery` in LastBlockTimestamp.
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

    // ----------------------------------------------------------------------------------------------------
	//                                     Lending Trait Implementation 
	// ----------------------------------------------------------------------------------------------------

    // ----------------------------------------------------------------------------------------------------
	//                                     Other Traits Implementations  
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> DeFiEngine for Pallet<T> {
		type MayBeAssetId = <T as DeFiComposableConfig>::MayBeAssetId;
		type Balance = <T as DeFiComposableConfig>::Balance;
		type AccountId = <T as frame_system::Config>::AccountId;
	}

    // ----------------------------------------------------------------------------------------------------
	//                                           Callable Functions  
	// ----------------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new lending market.
		/// - `origin` : Sender of this extrinsic. Manager for new market to be created. Can pause
		///   borrow operations.
		/// - `input`   : Borrow & deposits of assets, persentages.
		///
		/// `origin` irreversibly pays `T::OracleMarketCreationStake`.
		#[pallet::weight(<T as Config>::WeightInfo::create_market())]
		#[transactional]
		pub fn create_market(
			origin: OriginFor<T>,
			input: CreateInputOf<T>,
			keep_alive: bool,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let pair = input.currency_pair;
			let (market_id, vault_id) =
				<Self as Lending>::create_market(who.clone(), input, keep_alive)?;
			Self::deposit_event(Event::<T>::MarketCreated {
				market_id,
				vault_id,
				manager: who,
				currency_pair: pair,
			});
			Ok(().into())
		}

		/// owner must be very careful calling this
		// REVIEW: Why?
		#[pallet::weight(<T as Config>::WeightInfo::create_market())]
		#[transactional]
		pub fn update_market(
			origin: OriginFor<T>,
			market_id: MarketId,
			input: UpdateInput<T::LiquidationStrategyId, <T as frame_system::Config>::BlockNumber>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			<Self as Lending>::update_market(who, market_id, input)
		}

		/// Deposit collateral to market.
		/// - `origin` : Sender of this extrinsic.
		/// - `market` : Market index to which collateral will be deposited.
		/// - `amount` : Amount of collateral to be deposited.
		#[pallet::weight(<T as Config>::WeightInfo::deposit_collateral())]
		#[transactional]
		pub fn deposit_collateral(
			origin: OriginFor<T>,
			market_id: MarketId,
			amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			<Self as Lending>::deposit_collateral(&market_id, &sender, amount, keep_alive)?;
			Self::deposit_event(Event::<T>::CollateralDeposited { sender, market_id, amount });
			Ok(().into())
		}

		/// Withdraw collateral from market.
		/// - `origin` : Sender of this extrinsic.
		/// - `market_id` : Market index from which collateral will be withdraw.
		/// - `amount` : Amount of collateral to be withdrawn.
		#[pallet::weight(<T as Config>::WeightInfo::withdraw_collateral())]
		#[transactional]
		pub fn withdraw_collateral(
			origin: OriginFor<T>,
			market_id: MarketId,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			<Self as Lending>::withdraw_collateral(&market_id, &sender, amount)?;
			Self::deposit_event(Event::<T>::CollateralWithdrawn { sender, market_id, amount });
			Ok(().into())
		}

		/// Borrow asset against deposited collateral.
		/// - `origin` : Sender of this extrinsic. (Also the user who wants to borrow from market.)
		/// - `market_id` : Market index from which user wants to borrow.
		/// - `amount_to_borrow` : Amount which user wants to borrow.
		#[pallet::weight(<T as Config>::WeightInfo::borrow())]
		#[transactional]
		pub fn borrow(
			origin: OriginFor<T>,
			market_id: MarketId,
			amount_to_borrow: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			<Self as Lending>::borrow(&market_id, &sender, amount_to_borrow)?;
			Self::deposit_event(Event::<T>::Borrowed {
				sender,
				market_id,
				amount: amount_to_borrow,
			});
			Ok(().into())
		}

		/// Repay part or all of the borrow in the given market.
		///
		/// # Parameters
		///
		/// - `origin` : Sender of this extrinsic. (Also the user who repays beneficiary's borrow.)
		/// - `market_id` : [`MarketId`] of the market being repaid.
		/// - `beneficiary` : [`AccountId`] of the account who is in debt to (has borrowed assets
		///   from) the market. This can be same or different from the `origin`, allowing one
		///   account to pay off another's debts.
		/// - `amount`: The amount to repay. See [`RepayStrategy`] for more information.
		#[pallet::weight(<T as Config>::WeightInfo::repay_borrow())]
		#[transactional]
		pub fn repay_borrow(
			origin: OriginFor<T>,
			market_id: MarketId,
			beneficiary: T::AccountId,
			amount: RepayStrategy<T::Balance>,
			keep_alive: bool,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let amount_repaid = <Self as Lending>::repay_borrow(
				&market_id,
				&sender,
				&beneficiary,
				amount,
				keep_alive,
			)?;
			Self::deposit_event(Event::<T>::BorrowRepaid {
				sender,
				market_id,
				beneficiary,
				amount: amount_repaid,
			});
			Ok(().into())
		}

		/// Check if borrows for the `borrowers` accounts are required to be liquidated, initiate
		/// liquidation.
		/// - `origin` : Sender of this extrinsic.
		/// - `market_id` : Market index from which `borrower` has taken borrow.
		/// - `borrowers` : Vector of borrowers accounts' ids.
		#[pallet::weight(<T as Config>::WeightInfo::liquidate(borrowers.len() as u32))]
		#[transactional]
		pub fn liquidate(
			origin: OriginFor<T>,
			market_id: MarketId,
			borrowers: BoundedVec<T::AccountId, T::MaxLiquidationBatchSize>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin.clone())?;
			let subjected_borrowers = Self::liquidate_internal(&sender, &market_id, borrowers)?;
			// if at least one borrower was affected then liquidation been initiated
			if !subjected_borrowers.is_empty() {
				Self::deposit_event(Event::LiquidationInitiated {
					market_id,
					borrowers: subjected_borrowers,
				});
			}
			Ok(().into())
		}
	}

}
