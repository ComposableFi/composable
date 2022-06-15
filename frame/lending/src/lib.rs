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

pub mod weights;
pub use crate::weights::WeightInfo;

mod models;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod mocks_offchain;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_offchain;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
#[cfg(any(feature = "runtime-benchmarks", test))]
mod setup;

#[cfg(any(feature = "runtime-benchmarks", test))]
pub mod currency;

/// Various helpers used in the implementation of [`Lending::repay_borrow`].
///
/// [`Lending::repay_borrow`]: composable_traits::lending::Lending::repay_borrow
mod repay_borrow;

#[frame_support::pallet]
pub mod pallet {
	use crate::{models::borrower_data::BorrowerData, weights::WeightInfo};

	use codec::Codec;
	use composable_support::{
		math::safe::{SafeAdd, SafeDiv, SafeMul, SafeSub},
		validation::{TryIntoValidated, Validated},
	};
	use composable_traits::{
		currency::CurrencyFactory,
		defi::*,
		lending::{
			math::{self, *},
			BorrowAmountOf, CollateralLpAmountOf, CreateInput, CurrencyPairIsNotSame, Lending,
			MarketConfig, MarketModelValid, RepayStrategy, TotalDebtWithInterest, UpdateInput,
			UpdateInputValid,
		},
		liquidation::Liquidation,
		oracle::Oracle,
		time::{DurationSeconds, Timestamp, SECONDS_PER_YEAR_NAIVE},
		vault::{Deposit, FundsAvailability, StrategicVault, Vault, VaultConfig},
	};
	use frame_support::{
		pallet_prelude::*,
		storage::{with_transaction, TransactionOutcome},
		traits::{
			fungible::{Inspect as NativeInspect, Transfer as NativeTransfer},
			fungibles::{Inspect, InspectHold, Mutate, MutateHold, Transfer},
			tokens::DepositConsequence,
			UnixTime,
		},
		transactional,
		weights::WeightToFeePolynomial,
		PalletId,
	};
	use frame_system::{
		offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer},
		pallet_prelude::*,
	};
	use sp_core::crypto::KeyTypeId;
	use sp_runtime::{
		traits::{AccountIdConversion, One, Saturating, Zero},
		ArithmeticError, DispatchError, FixedPointNumber, FixedU128, KeyTypeId as CryptoKeyTypeId,
		Percent, Perquintill,
	};
	use sp_std::{fmt::Debug, vec, vec::Vec};

	/// Simple type alias around [`MarketConfig`] for this pallet.
	type MarketConfigOf<T> = MarketConfig<
		<T as Config>::VaultId,
		<T as DeFiComposableConfig>::MayBeAssetId,
		<T as frame_system::Config>::AccountId,
		<T as Config>::LiquidationStrategyId,
		<T as frame_system::Config>::BlockNumber,
	>;

	pub type MarketId = u32;

	// REVIEW: Maybe move this to `models::market_index`?
	// TODO: Rename to `MarketId`.
	#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq, MaxEncodedLen, TypeInfo)]
	#[repr(transparent)]
	pub struct MarketIndex(
		#[cfg(test)] // to allow pattern matching in tests outside of this crate
		pub  MarketId,
		#[cfg(not(test))] pub(crate) MarketId,
	);

	impl MarketIndex {
		pub fn new(i: u32) -> Self {
			Self(i)
		}
	}

	pub(crate) struct MarketAssets<T: DeFiComposableConfig> {
		/// The borrow asset for the market.
		pub(crate) borrow_asset: <T as DeFiComposableConfig>::MayBeAssetId,
		/// The debt token/ debt marker for the market.
		pub(crate) debt_asset: <T as DeFiComposableConfig>::MayBeAssetId,
	}

	/// Used to count the calls in [`Pallet::initialize_block`]. Each field corresponds to a
	/// function call to count.
	#[derive(Debug, Default, Clone, Copy)]
	pub(crate) struct InitializeBlockCallCounters {
		now: u32,
		read_markets: u32,
		accrue_interest: u32,
		account_id: u32,
		available_funds: u32,
		handle_withdrawable: u32,
		handle_depositable: u32,
		handle_must_liquidate: u32,
	}

	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"lend");
	pub const CRYPTO_KEY_TYPE: CryptoKeyTypeId = CryptoKeyTypeId(*b"lend");

	pub mod crypto {
		use super::KEY_TYPE;
		use frame_system::offchain;
		use sp_core::sr25519::{self, Signature as Sr25519Signature};
		use sp_runtime::{app_crypto::app_crypto, traits::Verify, MultiSignature, MultiSigner};

		app_crypto!(sr25519, KEY_TYPE);

		pub struct TestAuthId;

		// implementation for runtime
		impl offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sr25519::Signature;
			type GenericPublic = sr25519::Public;
		}

		// implementation for mock runtime in test
		impl offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for TestAuthId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sr25519::Signature;
			type GenericPublic = sr25519::Public;
		}
	}

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
		type WeightToFee: WeightToFeePolynomial<Balance = Self::Balance>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

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
			let signer = Signer::<T, <T as Config>::AuthorityId>::all_accounts();
			if !signer.can_sign() {
				log::warn!("No signer");
				return
			}
			for (market_id, account, _) in DebtIndex::<T>::iter() {
				//Check that it should liquidate before liquidations
				let should_be_liquidated =
					match Self::should_liquidate(&market_id, &account) {
						Ok(status) => status,
						Err(error) => {
							log::error!("Liquidation necessity check failed, market_id: {:?}, account: {:?},
                                        error: {:?}", market_id, account, error);
							false
						},
					};
				if !should_be_liquidated {
					continue
				}
				let results = signer.send_signed_transaction(|_account| Call::liquidate {
					market_id,
					borrowers: vec![account.clone()],
				});

				for (_acc, res) in &results {
					match res {
						Ok(()) => log::info!(
							"Liquidation succeed, market_id: {:?}, account: {:?}",
							market_id,
							account
						),
						Err(e) => log::error!(
							"Liquidation failed, market_id: {:?}, account: {:?}, error: {:?}",
							market_id,
							account,
							e
						),
					}
				}
			}
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		Overflow,
		Underflow,
		/// vault provided does not exist
		VaultNotFound,

		/// Only assets that have a known price are supported.
		BorrowAssetNotSupportedByOracle,
		/// Only assets that have a known price are supported.
		CollateralAssetNotSupportedByOracle,

		AssetPriceNotFound,
		/// The market could not be found
		MarketDoesNotExist,

		CollateralDepositFailed,
		MarketCollateralWasNotDepositedByAccount,

		/// The collateral factor for a market must be more than one.
		CollateralFactorMustBeMoreThanOne,
		/// Can't allow amount 0 as collateral.
		CannotDepositZeroCollateral,

		// REVIEW: Currently unused
		MarketAndAccountPairNotFound,

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

		// REVIEW: Currently unused
		CannotWithdrawFromProvidedBorrowAccount,

		BorrowRateDoesNotExist,

		// REVIEW: Currently unused
		BorrowIndexDoesNotExist,

		/// Borrow and repay in the same block (flashloans) are not allowed.
		BorrowAndRepayInSameBlockIsNotSupported,
		/// Repaying more than once in the same block is not allowed.
		CannotRepayMoreThanOnceInSameBlock,

		BorrowDoesNotExist,

		RepayAmountMustBeGreaterThanZero,
		CannotRepayMoreThanBorrowAmount,

		ExceedLendingCount,
		LiquidationFailed,

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

		MaxLiquidationBatchSizeExceeded,

		PriceTooOld,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when new lending market is created.
		MarketCreated {
			market_id: MarketIndex,
			vault_id: T::VaultId,
			manager: T::AccountId,
			currency_pair: CurrencyPair<T::MayBeAssetId>,
		},
		MarketUpdated {
			market_id: MarketIndex,
			input: UpdateInput<T::LiquidationStrategyId, <T as frame_system::Config>::BlockNumber>,
		},
		/// Event emitted when collateral is deposited.
		CollateralDeposited { sender: T::AccountId, market_id: MarketIndex, amount: T::Balance },
		/// Event emitted when collateral is withdrawed.
		CollateralWithdrawn { sender: T::AccountId, market_id: MarketIndex, amount: T::Balance },
		/// Event emitted when user borrows from given market.
		Borrowed { sender: T::AccountId, market_id: MarketIndex, amount: T::Balance },
		/// Event emitted when user repays borrow of beneficiary in given market.
		BorrowRepaid {
			sender: T::AccountId,
			market_id: MarketIndex,
			beneficiary: T::AccountId,
			amount: T::Balance,
		},
		/// Event emitted when a liquidation is initiated for a loan.
		LiquidationInitiated { market_id: MarketIndex, borrowers: Vec<T::AccountId> },
		/// Event emitted to warn that loan may go under collaterlized soon.
		MayGoUnderCollateralizedSoon { market_id: MarketIndex, account: T::AccountId },
	}

	/// Lending instances counter
	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // MarketIndex implements Default, so ValueQuery is ok here. REVIEW: Should it?
	pub type LendingCount<T: Config> = StorageValue<_, MarketIndex, ValueQuery>;

	/// Indexed lending instances. Maps markets to their respective [`MarketConfig`].
	///
	/// ```text
	/// MarketIndex -> MarketConfig
	/// ```
	#[pallet::storage]
	pub type Markets<T: Config> =
		StorageMap<_, Twox64Concat, MarketIndex, MarketConfigOf<T>, OptionQuery>;

	/// Maps markets to their corresponding debt token.
	///
	/// ```text
	/// MarketIndex -> debt asset
	/// ```
	///
	/// See [this clickup task](task) for a more in-depth explanation.
	///
	/// [task]: <https://sharing.clickup.com/20465559/t/h/27y9y84/15U30TKC3THPZYT>
	#[pallet::storage]
	pub type DebtTokenForMarket<T: Config> = StorageMap<
		_,
		Twox64Concat,
		MarketIndex,
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
		MarketIndex,
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
		MarketIndex,
		Twox64Concat,
		T::AccountId,
		Timestamp,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type BorrowRent<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		MarketIndex,
		Twox64Concat,
		T::AccountId,
		T::Balance,
		OptionQuery,
	>;

	/// market borrow index
	// REVIEW: ZeroToOneFixedU128?
	#[pallet::storage]
	pub type BorrowIndex<T: Config> =
		StorageMap<_, Twox64Concat, MarketIndex, ZeroToOneFixedU128, OptionQuery>;

	/// (Market, Account) -> Collateral
	#[pallet::storage]
	pub type AccountCollateral<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		MarketIndex,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
		OptionQuery,
	>;

	/// The timestamp of the previous block or defaults to timestamp at genesis.
	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // LastBlockTimestamp is set on genesis (see below) so it will always be set.
	pub type LastBlockTimestamp<T: Config> = StorageValue<_, Timestamp, ValueQuery>;

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

	/// A convenience wrapper around [`CreateInput`] for `T: Config`.
	pub type CreateInputOf<T> = CreateInput<
		<T as Config>::LiquidationStrategyId,
		<T as DeFiComposableConfig>::MayBeAssetId,
		<T as frame_system::Config>::BlockNumber,
	>;

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
			input: Validated<CreateInputOf<T>, (MarketModelValid, CurrencyPairIsNotSame)>,
			keep_alive: bool,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let input = input.value();
			let pair = input.currency_pair;
			let (market_id, vault_id) = Self::create(who.clone(), input, keep_alive)?;
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
			market_id: MarketIndex,
			input: Validated<
				UpdateInput<T::LiquidationStrategyId, <T as frame_system::Config>::BlockNumber>,
				UpdateInputValid,
			>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let input = input.value();
			Markets::<T>::mutate(&market_id, |market| {
				if let Some(market) = market {
					ensure!(who == market.manager, Error::<T>::Unauthorized);

					market.collateral_factor = input.collateral_factor;
					market.interest_rate_model = input.interest_rate_model;
					market.under_collateralized_warn_percent =
						input.under_collateralized_warn_percent;
					market.liquidators = input.liquidators.clone();
					Ok(())
				} else {
					Err(Error::<T>::MarketDoesNotExist)
				}
			})?;
			Self::deposit_event(Event::<T>::MarketUpdated { market_id, input });
			Ok(().into())
		}

		/// Deposit collateral to market.
		/// - `origin` : Sender of this extrinsic.
		/// - `market` : Market index to which collateral will be deposited.
		/// - `amount` : Amount of collateral to be deposited.
		#[pallet::weight(<T as Config>::WeightInfo::deposit_collateral())]
		#[transactional]
		pub fn deposit_collateral(
			origin: OriginFor<T>,
			market_id: MarketIndex,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			<Self as Lending>::deposit_collateral(&market_id, &sender, amount)?;
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
			market_id: MarketIndex,
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
			market_id: MarketIndex,
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
		/// - `market_id` : [`MarketIndex`] of the market being repaid.
		/// - `beneficiary` : [`AccountId`] of the account who is in debt to (has borrowed assets
		///   from) the market. This can be same or different from the `origin`, allowing one
		///   account to pay off another's debts.
		/// - `amount`: The amount to repay. See [`RepayStrategy`] for more information.
		#[pallet::weight(<T as Config>::WeightInfo::repay_borrow())]
		#[transactional]
		pub fn repay_borrow(
			origin: OriginFor<T>,
			market_id: MarketIndex,
			beneficiary: T::AccountId,
			amount: RepayStrategy<T::Balance>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let amount_repaid =
				<Self as Lending>::repay_borrow(&market_id, &sender, &beneficiary, amount)?;
			Self::deposit_event(Event::<T>::BorrowRepaid {
				sender,
				market_id,
				beneficiary,
				amount: amount_repaid,
			});
			Ok(().into())
		}

		/// Check if borrow for `borrower` account is required to be liquidated, initiate
		/// liquidation.
		/// - `origin` : Sender of this extrinsic.
		/// - `market_id` : Market index from which `borrower` has taken borrow.
		#[pallet::weight(<T as Config>::WeightInfo::liquidate(borrowers.len() as u32))]
		#[transactional]
		pub fn liquidate(
			origin: OriginFor<T>,
			market_id: MarketIndex,
			borrowers: Vec<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin.clone())?;
			ensure!(
				borrowers.len() <= T::MaxLiquidationBatchSize::get() as usize,
				Error::<T>::MaxLiquidationBatchSizeExceeded
			);
			Self::liquidate_internal(&sender, &market_id, borrowers.clone())?;
			Self::deposit_event(Event::LiquidationInitiated { market_id, borrowers });
			Ok(().into())
		}
	}

	// public helper functions
	impl<T: Config> Pallet<T> {
		/// Returns the initial pool size for a a market with `borrow_asset`. Calculated with
		/// [`Config::OracleMarketCreationStake`].
		pub fn calculate_initial_pool_size(
			borrow_asset: <T::Oracle as composable_traits::oracle::Oracle>::AssetId,
		) -> Result<<T as composable_traits::defi::DeFiComposableConfig>::Balance, DispatchError> {
			T::Oracle::get_price_inverse(borrow_asset, T::OracleMarketCreationStake::get())
		}

		/// Creates a new [`BorrowerData`] for the given market and account. See [`BorrowerData`]
		/// for more information.
		pub fn create_borrower_data(
			market_id: &<Self as Lending>::MarketId,
			account: &<Self as DeFiEngine>::AccountId,
		) -> Result<BorrowerData, DispatchError> {
			let market = Self::get_market(market_id)?;

			let collateral_balance_value = Self::get_price(
				market.collateral_asset,
				Self::collateral_of_account(market_id, account)?,
			)?;

			let account_total_debt_with_interest =
				Self::total_debt_with_interest(market_id, account)?.unwrap_or_zero();
			let borrow_balance_value = Self::get_price(
				T::Vault::asset_id(&market.borrow_asset_vault)?,
				account_total_debt_with_interest,
			)?;

			let borrower =
				BorrowerData::new(
					collateral_balance_value,
					borrow_balance_value,
					market
						.collateral_factor
						.try_into_validated()
						.map_err(|_| Error::<T>::CollateralFactorMustBeMoreThanOne)?, /* TODO: Use a proper error mesage */
					market.under_collateralized_warn_percent,
				);

			Ok(borrower)
		}

		/// Whether or not an account should be liquidated. See [`BorrowerData::should_liquidate()`]
		/// for more information.
		pub fn should_liquidate(
			market_id: &<Self as Lending>::MarketId,
			account: &<Self as DeFiEngine>::AccountId,
		) -> Result<bool, DispatchError> {
			let borrower = Self::create_borrower_data(market_id, account)?;
			let should_liquidate = borrower.should_liquidate()?;
			Ok(should_liquidate)
		}

		pub fn soon_under_collateralized(
			market_id: &<Self as Lending>::MarketId,
			account: &<Self as DeFiEngine>::AccountId,
		) -> Result<bool, DispatchError> {
			let borrower = Self::create_borrower_data(market_id, account)?;
			let should_warn = borrower.should_warn()?;
			Ok(should_warn)
		}

		pub fn liquidate_internal(
			liquidator: &<Self as DeFiEngine>::AccountId,
			market_id: &<Self as Lending>::MarketId,
			borrowers: Vec<<Self as DeFiEngine>::AccountId>,
		) -> Result<(), DispatchError> {
			for account in borrowers.iter() {
				if Self::should_liquidate(market_id, account)? {
					let market = Self::get_market(market_id)?;
					let borrow_asset = T::Vault::asset_id(&market.borrow_asset_vault)?;
					let collateral_to_liquidate = Self::collateral_of_account(market_id, account)?;
					let source_target_account = Self::account_id(market_id);
					let unit_price = T::Oracle::get_ratio(CurrencyPair::new(
						market.collateral_asset,
						borrow_asset,
					))?;
					let sell = Sell::new(
						market.collateral_asset,
						borrow_asset,
						collateral_to_liquidate,
						unit_price,
					);
					T::Liquidation::liquidate(&source_target_account, sell, market.liquidators)?;

					if let Some(deposit) = BorrowRent::<T>::get(market_id, account) {
						let market_account = Self::account_id(market_id);
						<T as Config>::NativeCurrency::transfer(
							&market_account,
							liquidator,
							deposit,
							false,
						)?;
					}
				}
			}
			Ok(())
		}
	}

	// crate-public helper functions
	impl<T: Config> Pallet<T> {
		pub(crate) fn initialize_block(
			block_number: T::BlockNumber,
		) -> InitializeBlockCallCounters {
			let mut call_counters = InitializeBlockCallCounters::default();
			let _ = with_transaction(|| {
				let now = Self::now();
				call_counters.now += 1;

				let mut errors = Markets::<T>::iter()
					.map(|(market_id, config)| {
						call_counters.read_markets += 1;
						Self::accrue_interest(&market_id, now)?;
						call_counters.accrue_interest += 1;
						let market_account = Self::account_id(&market_id);
						call_counters.account_id += 1;
						// NOTE(hussein-aitlahcen):
						// It would probably be more perfomant to handle theses
						// case while borrowing/repaying.
						//
						// I don't know whether we would face any issue by doing that.
						//
						// borrow:
						//  - withdrawable = transfer(vault->market) + transfer(market->user)
						//  - depositable = error(not enough borrow asset) // vault asking for
						//    reserve to be fullfilled
						//  - mustliquidate = error(market is closing)
						// repay:
						// 	- (withdrawable || depositable || mustliquidate) =
						//    transfer(user->market) + transfer(market->vault)
						//
						// The intermediate transfer(vault->market) while borrowing would
						// allow the vault to update the strategy balance (market = borrow vault
						// strategy).
						match Self::available_funds(&config, &market_account)? {
							FundsAvailability::Withdrawable(balance) => {
								Self::handle_withdrawable(&config, &market_account, balance)?;
								call_counters.handle_withdrawable += 1;
							},
							FundsAvailability::Depositable(balance) => {
								Self::handle_depositable(&config, &market_account, balance)?;
								call_counters.handle_depositable += 1;
							},
							FundsAvailability::MustLiquidate => {
								Self::handle_must_liquidate(&config, &market_account)?;
								call_counters.handle_must_liquidate += 1;
							},
							FundsAvailability::None => {},
						}

						call_counters.available_funds += 1;

						Result::<(), DispatchError>::Ok(())
					})
					.filter_map(|r| match r {
						Ok(_) => None,
						Err(err) => Some(err),
					})
					.peekable();

				if errors.peek().is_none() {
					LastBlockTimestamp::<T>::put(now);
					TransactionOutcome::Commit(Ok(1000))
				} else {
					errors.for_each(|e| {
						log::error!(
							"This should never happen, could not initialize block!!! {:#?} {:#?}",
							block_number,
							e
						)
					});
					TransactionOutcome::Rollback(Err(DispatchError::Other(
						"failed to initialize block",
					)))
				}
			});
			call_counters
		}

		pub(crate) fn now() -> u64 {
			T::UnixTime::now().as_secs()
		}

		pub(crate) fn available_funds(
			config: &MarketConfigOf<T>,
			market_account: &T::AccountId,
		) -> Result<FundsAvailability<T::Balance>, DispatchError> {
			<T::Vault as StrategicVault>::available_funds(
				&config.borrow_asset_vault,
				market_account,
			)
		}

		pub(crate) fn handle_withdrawable(
			config: &MarketConfigOf<T>,
			market_account: &T::AccountId,
			balance: T::Balance,
		) -> Result<(), DispatchError> {
			<T::Vault as StrategicVault>::withdraw(
				&config.borrow_asset_vault,
				market_account,
				balance,
			)
		}

		pub(crate) fn handle_depositable(
			config: &MarketConfigOf<T>,
			market_account: &T::AccountId,
			balance: T::Balance,
		) -> Result<(), DispatchError> {
			let asset_id = <T::Vault as Vault>::asset_id(&config.borrow_asset_vault)?;
			let balance =
				<T as Config>::MultiCurrency::reducible_balance(asset_id, market_account, false)
					.min(balance);
			<T::Vault as StrategicVault>::deposit(
				&config.borrow_asset_vault,
				market_account,
				balance,
			)
		}

		pub(crate) fn handle_must_liquidate(
			config: &MarketConfigOf<T>,
			market_account: &T::AccountId,
		) -> Result<(), DispatchError> {
			let asset_id = <T::Vault as Vault>::asset_id(&config.borrow_asset_vault)?;
			let balance =
				<T as Config>::MultiCurrency::reducible_balance(asset_id, market_account, false);
			<T::Vault as StrategicVault>::deposit(
				&config.borrow_asset_vault,
				market_account,
				balance,
			)
		}

		/// Returns the borrow and debt assets for the given market, if it exists.
		pub(crate) fn get_assets_for_market(
			market_id: &MarketIndex,
		) -> Result<MarketAssets<T>, DispatchError> {
			let borrow_asset =
				T::Vault::asset_id(&Self::get_market(market_id)?.borrow_asset_vault)?;
			let debt_asset =
				DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

			Ok(MarketAssets { borrow_asset, debt_asset })
		}
	}

	// private helper functions
	impl<T: Config> Pallet<T> {
		fn get_market(market_id: &MarketIndex) -> Result<MarketConfigOf<T>, DispatchError> {
			Markets::<T>::get(market_id).ok_or_else(|| Error::<T>::MarketDoesNotExist.into())
		}

		fn get_price(
			asset_id: <T as DeFiComposableConfig>::MayBeAssetId,
			amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			<T::Oracle as Oracle>::get_price(asset_id, amount)
				.map(|p| p.price)
				.map_err(|_| Error::<T>::AssetPriceNotFound.into())
		}

		/// Some of these checks remain to provide better errors. See [this clickup task](task) for
		/// more information.
		///
		/// [task]: <https://sharing.clickup.com/20465559/t/h/27yd3wt/7IB0QYYHXP0TZZT>
		fn can_borrow(
			market_id: &MarketIndex,
			debt_owner: &T::AccountId,
			amount_to_borrow: BorrowAmountOf<Self>,
			market: MarketConfigOf<T>,
			market_account: &T::AccountId,
		) -> Result<(), DispatchError> {
			// this check prevents free flash loans
			if let Some(latest_borrow_timestamp) = BorrowTimestamp::<T>::get(market_id, debt_owner)
			{
				if latest_borrow_timestamp >= LastBlockTimestamp::<T>::get() {
					return Err(Error::<T>::InvalidTimestampOnBorrowRequest.into())
				}
			}

			let borrow_asset = T::Vault::asset_id(&market.borrow_asset_vault)?;
			let borrow_limit = Self::get_borrow_limit(market_id, debt_owner)?;
			let borrow_amount_value = Self::get_price(borrow_asset, amount_to_borrow)?;
			ensure!(borrow_limit >= borrow_amount_value, Error::<T>::NotEnoughCollateralToBorrow);

			ensure!(
				<T as Config>::MultiCurrency::can_withdraw(
					borrow_asset,
					market_account,
					amount_to_borrow
				)
				.into_result()
				.is_ok(),
				Error::<T>::NotEnoughBorrowAsset,
			);

			if !BorrowRent::<T>::contains_key(market_id, debt_owner) {
				let deposit = T::WeightToFee::calc(&T::WeightInfo::liquidate(1));

				// See note 1
				ensure!(
					<T as Config>::NativeCurrency::can_withdraw(debt_owner, deposit)
						.into_result()
						.is_ok(),
					Error::<T>::NotEnoughRent,
				);
			}

			ensure!(
				!matches!(
					T::Vault::available_funds(&market.borrow_asset_vault, market_account)?,
					FundsAvailability::MustLiquidate
				),
				Error::<T>::MarketIsClosing
			);

			Ok(())
		}

		/// Check is price actual yet
		fn ensure_price_is_recent(market: &MarketConfigOf<T>) -> Result<(), DispatchError> {
			use sp_runtime::traits::CheckedSub as _;

			let borrow_asset = T::Vault::asset_id(&market.borrow_asset_vault)?;

			let current_block = frame_system::Pallet::<T>::block_number();
			let blocks_count = market.max_price_age;
			let edge_block = current_block.checked_sub(&blocks_count).unwrap_or_default();

			// check borrow asset
			let price_block =
				<T::Oracle as Oracle>::get_price(borrow_asset, BorrowAmountOf::<Self>::default())?
					.block;
			ensure!(price_block >= edge_block, Error::<T>::PriceTooOld);

			// check collateral asset
			let collateral_asset = market.collateral_asset;
			let price_block = <T::Oracle as Oracle>::get_price(
				collateral_asset,
				BorrowAmountOf::<Self>::default(),
			)?
			.block;
			ensure!(price_block >= edge_block, Error::<T>::PriceTooOld);

			Ok(())
		}
	}

	impl<T: Config> DeFiEngine for Pallet<T> {
		type MayBeAssetId = <T as DeFiComposableConfig>::MayBeAssetId;

		type Balance = <T as DeFiComposableConfig>::Balance;

		type AccountId = <T as frame_system::Config>::AccountId;
	}

	impl<T: Config> Lending for Pallet<T> {
		type VaultId = <T::Vault as Vault>::VaultId;
		type MarketId = MarketIndex;
		type BlockNumber = T::BlockNumber;
		type LiquidationStrategyId = <T as Config>::LiquidationStrategyId;

		fn create(
			manager: Self::AccountId,
			config_input: CreateInput<
				Self::LiquidationStrategyId,
				Self::MayBeAssetId,
				Self::BlockNumber,
			>,
			keep_alive: bool,
		) -> Result<(Self::MarketId, Self::VaultId), DispatchError> {
			// TODO: Replace with `Validate`
			ensure!(
				config_input.updatable.collateral_factor > 1.into(),
				Error::<T>::CollateralFactorMustBeMoreThanOne
			);

			ensure!(
				<T::Oracle as Oracle>::is_supported(config_input.borrow_asset())?,
				Error::<T>::BorrowAssetNotSupportedByOracle
			);
			ensure!(
				<T::Oracle as Oracle>::is_supported(config_input.collateral_asset())?,
				Error::<T>::CollateralAssetNotSupportedByOracle
			);

			LendingCount::<T>::try_mutate(|MarketIndex(previous_market_index)| {
				let market_id = {
					// TODO: early mutation of `previous_market_index` value before check.
					*previous_market_index += 1;
					ensure!(
						*previous_market_index <= T::MaxMarketCount::get(),
						Error::<T>::ExceedLendingCount
					);
					MarketIndex(*previous_market_index)
				};

				let borrow_asset_vault = T::Vault::create(
					Deposit::Existential,
					VaultConfig {
						asset_id: config_input.borrow_asset(),
						reserved: config_input.reserved_factor(),
						manager: manager.clone(),
						strategies: [(
							Self::account_id(&market_id),
							// Borrowable = 100% - reserved
							// REVIEW: Review use of `saturating_sub` here - I'm pretty sure this
							// can never error, but if `Perquintill` can be `>`
							// `Perquintill::one()` then we might want to re-evaluate the logic
							// here.
							Perquintill::one().saturating_sub(config_input.reserved_factor()),
						)]
						.into_iter()
						.collect(),
					},
				)?;

				let initial_pool_size =
					Self::calculate_initial_pool_size(config_input.borrow_asset())?;

				ensure!(
					initial_pool_size > T::Balance::zero(),
					Error::<T>::PriceOfInitialBorrowVaultShouldBeGreaterThanZero
				);

				// transfer `initial_pool_size` worth of borrow asset from the manager to the market
				T::MultiCurrency::transfer(
					config_input.borrow_asset(),
					&manager,
					&Self::account_id(&market_id),
					initial_pool_size,
					keep_alive,
				)?;

				let market_config = MarketConfig {
					manager,
					max_price_age: config_input.updatable.max_price_age,
					borrow_asset_vault: borrow_asset_vault.clone(),
					collateral_asset: config_input.collateral_asset(),
					collateral_factor: config_input.updatable.collateral_factor,
					interest_rate_model: config_input.updatable.interest_rate_model,
					under_collateralized_warn_percent: config_input
						.updatable
						.under_collateralized_warn_percent,
					liquidators: config_input.updatable.liquidators,
				};
				// TODO: pass ED from API,
				let debt_token_id = T::CurrencyFactory::reserve_lp_token_id(T::Balance::default())?;

				DebtTokenForMarket::<T>::insert(market_id, debt_token_id);
				Markets::<T>::insert(market_id, market_config);
				BorrowIndex::<T>::insert(market_id, FixedU128::one());

				Ok((market_id, borrow_asset_vault))
			})
		}

		fn account_id(market_id: &Self::MarketId) -> Self::AccountId {
			T::PalletId::get().into_sub_account(market_id)
		}

		fn deposit_collateral(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
			amount: CollateralLpAmountOf<Self>,
		) -> Result<(), DispatchError> {
			ensure!(amount > Self::Balance::zero(), Error::<T>::CannotDepositZeroCollateral);
			let market = Self::get_market(market_id)?;
			let market_account = Self::account_id(market_id);

			AccountCollateral::<T>::try_mutate(market_id, account, |collateral_balance| {
				let new_collateral_balance =
					collateral_balance.unwrap_or_default().safe_add(&amount)?;
				collateral_balance.replace(new_collateral_balance);
				Result::<(), DispatchError>::Ok(())
			})?;

			<T as Config>::MultiCurrency::transfer(
				market.collateral_asset,
				account,
				&market_account,
				amount,
				true,
			)?;
			Ok(())
		}

		fn withdraw_collateral(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
			amount: CollateralLpAmountOf<Self>,
		) -> Result<(), DispatchError> {
			let market = Self::get_market(market_id)?;

			let collateral_balance = AccountCollateral::<T>::try_get(market_id, account)
				// REVIEW: Perhaps don't default to zero
				// REVIEW: What is expected behaviour if there is no collateral?
				.unwrap_or_else(|_| CollateralLpAmountOf::<Self>::zero());

			ensure!(amount <= collateral_balance, Error::<T>::NotEnoughCollateralToWithdraw);

			let borrow_asset = T::Vault::asset_id(&market.borrow_asset_vault)?;
			let borrower_balance_with_interest =
				Self::total_debt_with_interest(market_id, account)?.unwrap_or_zero();

			let borrow_balance_value =
				Self::get_price(borrow_asset, borrower_balance_with_interest)?;

			let collateral_balance_after_withdrawal_value =
				Self::get_price(market.collateral_asset, collateral_balance.safe_sub(&amount)?)?;

			let borrower_after_withdrawal = BorrowerData::new(
				collateral_balance_after_withdrawal_value,
				borrow_balance_value,
				market
					.collateral_factor
					.try_into_validated()
					.map_err(|_| Error::<T>::Overflow)?, // TODO: Use a proper error mesage?
				market.under_collateralized_warn_percent,
			);

			ensure!(
				!borrower_after_withdrawal.should_liquidate()?,
				Error::<T>::WouldGoUnderCollateralized
			);

			let market_account = Self::account_id(market_id);

			ensure!(
				<T as Config>::MultiCurrency::can_deposit(
					market.collateral_asset,
					account,
					amount,
					false
				) == DepositConsequence::Success,
				Error::<T>::TransferFailed
			);
			ensure!(
				<T as Config>::MultiCurrency::can_withdraw(
					market.collateral_asset,
					&market_account,
					amount
				)
				.into_result()
				.is_ok(),
				Error::<T>::TransferFailed
			);

			AccountCollateral::<T>::try_mutate(market_id, account, |collateral_balance| {
				let new_collateral_balance =
					// REVIEW: Should we default if there's no collateral? Or should an error (something like "NoCollateralToWithdraw") be returned instead?
					collateral_balance.unwrap_or_default().safe_sub(&amount)?;

				collateral_balance.replace(new_collateral_balance);

				Result::<(), DispatchError>::Ok(())
			})?;
			<T as Config>::MultiCurrency::transfer(
				market.collateral_asset,
				&market_account,
				account,
				amount,
				true,
			)
			.expect("impossible; qed;");
			Ok(())
		}

		fn get_markets_for_borrow(borrow: Self::VaultId) -> Vec<Self::MarketId> {
			Markets::<T>::iter()
				.filter_map(|(index, market)| market.borrow_asset_vault.eq(&borrow).then(|| index))
				.collect()
		}

		fn borrow(
			market_id: &Self::MarketId,
			borrowing_account: &Self::AccountId,
			amount_to_borrow: BorrowAmountOf<Self>,
		) -> Result<(), DispatchError> {
			let market = Self::get_market(market_id)?;

			Self::ensure_price_is_recent(&market)?;

			let MarketAssets { borrow_asset, debt_asset: debt_asset_id } =
				Self::get_assets_for_market(market_id)?;

			let market_account = Self::account_id(market_id);

			Self::can_borrow(
				market_id,
				borrowing_account,
				amount_to_borrow,
				market,
				&market_account,
			)?;

			let new_account_interest_index = {
				let market_index =
					BorrowIndex::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

				// previous account interest index
				let account_interest_index = DebtIndex::<T>::get(market_id, borrowing_account)
					.unwrap_or_else(ZeroToOneFixedU128::zero);

				// amount of debt currently
				let existing_principal_amount =
					<T as Config>::MultiCurrency::balance(debt_asset_id, borrowing_account);

				// principal_after_new_borrow
				let principal_after_new_borrow =
					existing_principal_amount.safe_add(&amount_to_borrow)?;

				// amount of principal the account already has
				let existing_borrow_share =
					Percent::from_rational(existing_principal_amount, principal_after_new_borrow);
				// amount of principal the account is adding
				let new_borrow_share =
					Percent::from_rational(amount_to_borrow, principal_after_new_borrow);

				market_index
					.safe_mul(&new_borrow_share.into())?
					.safe_add(&account_interest_index.safe_mul(&existing_borrow_share.into())?)?
			};

			// mint debt token into user and lock it (it's used as a marker of how much the account
			// has borrowed total)
			<T as Config>::MultiCurrency::mint_into(
				debt_asset_id,
				borrowing_account,
				amount_to_borrow,
			)?;
			<T as Config>::MultiCurrency::hold(debt_asset_id, borrowing_account, amount_to_borrow)?;

			// transfer borrow asset from market to the borrower
			<T as Config>::MultiCurrency::transfer(
				borrow_asset,
				&market_account,
				borrowing_account,
				amount_to_borrow,
				false,
			)?;
			DebtIndex::<T>::insert(market_id, borrowing_account, new_account_interest_index);
			BorrowTimestamp::<T>::insert(
				market_id,
				borrowing_account,
				LastBlockTimestamp::<T>::get(),
			);

			if !BorrowRent::<T>::contains_key(market_id, borrowing_account) {
				let deposit = T::WeightToFee::calc(&T::WeightInfo::liquidate(2));
				<T as Config>::NativeCurrency::transfer(
					borrowing_account,
					&market_account,
					deposit,
					true,
				)?;
				BorrowRent::<T>::insert(market_id, borrowing_account, deposit);
			} else {
				// REVIEW
			}

			Ok(())
		}

		/// NOTE: Must be called in transaction!
		fn repay_borrow(
			market_id: &Self::MarketId,
			from: &Self::AccountId,
			beneficiary: &Self::AccountId,
			total_repay_amount: RepayStrategy<BorrowAmountOf<Self>>,
			// TODO: add keep_alive
		) -> Result<BorrowAmountOf<Self>, DispatchError> {
			use crate::repay_borrow::{pay_interest, repay_principal};

			// cannot repay in the same block as the borrow
			let timestamp = BorrowTimestamp::<T>::get(market_id, beneficiary)
				.ok_or(Error::<T>::BorrowDoesNotExist)?;
			ensure!(
				timestamp != LastBlockTimestamp::<T>::get(),
				Error::<T>::BorrowAndRepayInSameBlockIsNotSupported
			);

			// principal + interest
			let beneficiary_total_debt_with_interest =
				match Self::total_debt_with_interest(market_id, beneficiary)? {
					TotalDebtWithInterest::Amount(amount) => amount,
					TotalDebtWithInterest::NoDebt =>
						return Err(Error::<T>::CannotRepayZeroBalance.into()),
				};

			let market_account = Self::account_id(market_id);

			let MarketAssets { borrow_asset, debt_asset } = Self::get_assets_for_market(market_id)?;

			// initial borrow amount
			let beneficiary_borrow_asset_principal =
				<T as Config>::MultiCurrency::balance(debt_asset, beneficiary);
			// interest accrued
			let beneficiary_interest_on_market = beneficiary_total_debt_with_interest
				.safe_sub(&beneficiary_borrow_asset_principal)?;

			ensure!(
				!beneficiary_total_debt_with_interest.is_zero(),
				Error::<T>::CannotRepayZeroBalance
			);

			let repaid_amount = match total_repay_amount {
				RepayStrategy::TotalDebt => {
					// pay interest, from -> market
					// burn debt token interest from market
					pay_interest::<T>(
						borrow_asset,
						debt_asset,
						from,
						&market_account,
						beneficiary_interest_on_market,
						true,
					)?;

					// release and burn debt token from beneficiary and transfer borrow asset to
					// market, paid by `from`
					repay_principal::<T>(
						borrow_asset,
						debt_asset,
						from,
						&market_account,
						beneficiary,
						beneficiary_borrow_asset_principal,
						true,
					)?;

					beneficiary_total_debt_with_interest
				},

				// attempt to repay a partial amount of the debt, paying off interest and principal
				// proportional to how much of each there is.
				RepayStrategy::PartialAmount(partial_repay_amount) => {
					ensure!(
						partial_repay_amount <= beneficiary_total_debt_with_interest,
						Error::<T>::CannotRepayMoreThanTotalDebt
					);

					// INVARIANT: ArithmeticError::Overflow is used as the error here as
					// beneficiary_total_debt_with_interest is known to be non-zero at this point
					// due to the check above (CannotRepayZeroBalance)

					let interest_percentage = FixedU128::checked_from_rational(
						beneficiary_interest_on_market,
						beneficiary_total_debt_with_interest,
					)
					.ok_or(ArithmeticError::Overflow)?;

					let principal_percentage = FixedU128::checked_from_rational(
						beneficiary_borrow_asset_principal,
						beneficiary_total_debt_with_interest,
					)
					.ok_or(ArithmeticError::Overflow)?;

					// pay interest, from -> market
					// burn interest (debt token) from market
					pay_interest::<T>(
						borrow_asset,
						debt_asset,
						from,
						&market_account,
						interest_percentage
							.checked_mul_int::<u128>(partial_repay_amount.into())
							.ok_or(ArithmeticError::Overflow)?
							.into(),
						true,
					)?;

					// release and burn debt token from beneficiary and transfer borrow asset to
					// market, paid by `from`
					repay_principal::<T>(
						borrow_asset,
						debt_asset,
						from,
						&market_account,
						beneficiary,
						principal_percentage
							.checked_mul_int::<u128>(partial_repay_amount.into())
							.ok_or(ArithmeticError::Overflow)?
							.into(),
						true,
					)?;

					// the above will short circuit if amount cannot be paid, so if this is reached
					// then we know `partial_repay_amount` has been repaid
					partial_repay_amount
				},
			};

			// if the borrow is completely repaid, remove the borrow information
			if repaid_amount == beneficiary_total_debt_with_interest {
				// borrow no longer exists as it has been repaid in entirety, remove the
				// timestamp & index
				BorrowTimestamp::<T>::remove(market_id, beneficiary);
				DebtIndex::<T>::remove(market_id, beneficiary);

				// give back rent (rent = deposit)
				let rent = BorrowRent::<T>::get(market_id, beneficiary)
					.ok_or(Error::<T>::BorrowRentDoesNotExist)?;

				<T as Config>::NativeCurrency::transfer(
					&market_account,
					beneficiary,
					rent,
					false, // we do not need to keep the market account alive
				)?;
			}

			Ok(repaid_amount)
		}

		fn total_borrowed_from_market_excluding_interest(
			market_id: &Self::MarketId,
		) -> Result<Self::Balance, DispatchError> {
			let debt_token =
				DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

			// total amount of debt *interest* owned by the market
			let total_debt_interest =
				<T as Config>::MultiCurrency::balance(debt_token, &Self::account_id(market_id));

			let total_issued = <T as Config>::MultiCurrency::total_issuance(debt_token);
			let total_amount_borrowed_from_market = total_issued.safe_sub(&total_debt_interest)?;

			Ok(total_amount_borrowed_from_market)
		}

		fn total_interest(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			let debt_token =
				DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

			// total amount of debt *interest* owned by the market
			let total_debt_interest =
				<T as Config>::MultiCurrency::balance(debt_token, &Self::account_id(market_id));

			Ok(total_debt_interest)
		}

		fn accrue_interest(
			market_id: &Self::MarketId,
			now: Timestamp,
		) -> Result<(), DispatchError> {
			// we maintain original borrow principals intact on hold,
			// but accrue total borrow balance by adding to market debt balance
			// when user pays loan back, we reduce marked accrued debt
			// so no need to loop over each account -> scales to millions of users

			let total_borrowed_from_market_excluding_interest =
				Self::total_borrowed_from_market_excluding_interest(market_id)?;
			let total_available_to_be_borrowed = Self::total_available_to_be_borrowed(market_id)?;

			let utilization_ratio = Self::calculate_utilization_ratio(
				total_available_to_be_borrowed,
				total_borrowed_from_market_excluding_interest,
			)?;

			let delta_time = now.checked_sub(LastBlockTimestamp::<T>::get()).ok_or(
				// REVIEW: INVARIANT: this error should never happen, `now` should always
				// be `> LastBlockTimestamp`
				Error::<T>::Underflow,
			)?;

			let borrow_index =
				BorrowIndex::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;
			let debt_asset_id =
				DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

			let accrued_interest = Markets::<T>::try_mutate(market_id, |market_config| {
				let market_config = market_config.as_mut().ok_or(Error::<T>::MarketDoesNotExist)?;

				accrue_interest_internal::<T, InterestRateModel>(
					utilization_ratio,
					&mut market_config.interest_rate_model,
					borrow_index,
					delta_time,
					total_borrowed_from_market_excluding_interest,
				)
			})?;

			// overwrites
			BorrowIndex::<T>::insert(market_id, accrued_interest.new_borrow_index);
			<T as Config>::MultiCurrency::mint_into(
				debt_asset_id,
				&Self::account_id(market_id),
				accrued_interest.accrued_increment,
			)?;

			Ok(())
		}

		fn total_available_to_be_borrowed(
			market_id: &Self::MarketId,
		) -> Result<Self::Balance, DispatchError> {
			let market = Self::get_market(market_id)?;
			let borrow_asset_id = T::Vault::asset_id(&market.borrow_asset_vault)?;
			Ok(<T as Config>::MultiCurrency::balance(borrow_asset_id, &Self::account_id(market_id)))
		}

		fn calculate_utilization_ratio(
			cash: Self::Balance,
			borrows: Self::Balance,
		) -> Result<Percent, DispatchError> {
			Ok(math::calculate_utilization_ratio(
				LiftedFixedBalance::saturating_from_integer(cash.into()),
				LiftedFixedBalance::saturating_from_integer(borrows.into()),
			)?)
		}

		// previously 'borrow_balance_current'
		fn total_debt_with_interest(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<TotalDebtWithInterest<BorrowAmountOf<Self>>, DispatchError> {
			let debt_token =
				DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

			// Self::get_assets_for_market()?;
			match DebtIndex::<T>::get(market_id, account) {
				Some(account_interest_index) => {
					let market_interest_index =
						BorrowIndex::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

					let account_principal =
						<T as Config>::MultiCurrency::balance_on_hold(debt_token, account);

					if account_principal.is_zero() {
						Ok(TotalDebtWithInterest::NoDebt)
					} else {
						// REVIEW
						let account_principal =
							LiftedFixedBalance::saturating_from_integer(account_principal.into());
						// principal * (market index / debt index)
						let index_ratio =
							market_interest_index.safe_div(&account_interest_index)?;

						let balance = account_principal
							.safe_mul(&index_ratio)?
							// TODO: Balance should be u128 eventually
							.checked_mul_int(1_u64)
							.ok_or(ArithmeticError::Overflow)?;
						Ok(TotalDebtWithInterest::Amount(balance.into()))
					}
				},
				None => Ok(TotalDebtWithInterest::NoDebt),
			}
		}

		fn collateral_of_account(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<CollateralLpAmountOf<Self>, DispatchError> {
			AccountCollateral::<T>::get(market_id, account)
				.ok_or_else(|| Error::<T>::MarketCollateralWasNotDepositedByAccount.into())
		}

		fn collateral_required(
			market_id: &Self::MarketId,
			borrow_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let market = Self::get_market(market_id)?;
			let borrow_asset = T::Vault::asset_id(&market.borrow_asset_vault)?;
			let borrow_amount_value = Self::get_price(borrow_asset, borrow_amount)?;

			Ok(LiftedFixedBalance::saturating_from_integer(borrow_amount_value.into())
				.safe_mul(&market.collateral_factor)?
				.checked_mul_int(1_u64)
				.ok_or(ArithmeticError::Overflow)?
				.into())
		}

		fn get_borrow_limit(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			let collateral_balance = AccountCollateral::<T>::get(market_id, account)
				// REVIEW: I don't think this should default to zero, only to check against zero
				// afterwards.
				.unwrap_or_else(CollateralLpAmountOf::<Self>::zero);

			if collateral_balance > T::Balance::zero() {
				let borrower = Self::create_borrower_data(market_id, account)?;
				let balance = borrower
					.get_borrow_limit()
					.map_err(|_| Error::<T>::BorrowerDataCalculationFailed)?
					.checked_mul_int(1_u64)
					.ok_or(ArithmeticError::Overflow)?;
				Ok(balance.into())
			} else {
				Ok(Self::Balance::zero())
			}
		}
	}

	// various helper functions

	/// given collateral information, how much of borrow asset can get?
	pub fn swap(
		collateral_balance: &LiftedFixedBalance,
		collateral_price: &LiftedFixedBalance,
		collateral_factor: &MoreThanOneFixedU128,
	) -> Result<LiftedFixedBalance, ArithmeticError> {
		collateral_balance.safe_mul(collateral_price)?.safe_div(collateral_factor)
	}

	/// ```python
	/// delta_interest_rate = delta_time / period_interest_rate
	/// debt_delta = debt_principal * delta_interest_rate
	/// new_accrued_debt = accrued_debt + debt_delta
	/// total_debt = debt_principal + new_accrued_debt
	/// ```
	pub(crate) fn accrue_interest_internal<T: Config, I: InterestRate>(
		utilization_ratio: Percent,
		interest_rate_model: &mut I,
		borrow_index: OneOrMoreFixedU128,
		delta_time: DurationSeconds,
		total_borrows: T::Balance,
	) -> Result<AccruedInterest<T>, DispatchError> {
		let total_borrows: FixedU128 =
			FixedU128::checked_from_integer(Into::<u128>::into(total_borrows))
				.ok_or(ArithmeticError::Overflow)?;

		let borrow_rate = interest_rate_model
			.get_borrow_rate(utilization_ratio)
			.ok_or(Error::<T>::BorrowRateDoesNotExist)?;

		// borrow_rate * index * delta_time / SECONDS_PER_YEAR_NAIVE + index
		let borrow_rate_delta = borrow_rate
			.safe_mul(&FixedU128::saturating_from_integer(delta_time))?
			.safe_div(&FixedU128::saturating_from_integer(SECONDS_PER_YEAR_NAIVE))?;

		let new_borrow_index =
			borrow_rate_delta.safe_mul(&borrow_index)?.safe_add(&borrow_index)?;

		let accrued_increment = total_borrows
			.safe_mul(&borrow_rate_delta)?
			.checked_mul_int(1_u64)
			.ok_or(ArithmeticError::Overflow)?
			.into();

		Ok(AccruedInterest { accrued_increment, new_borrow_index })
	}

	#[derive(Debug, PartialEqNoBound)]
	pub(crate) struct AccruedInterest<T: Config> {
		pub(crate) accrued_increment: T::Balance,
		pub(crate) new_borrow_index: FixedU128,
	}

	/// Retrieve the current interest rate for the given `market_id`.
	pub fn current_interest_rate<T: Config>(market_id: MarketId) -> Result<Rate, DispatchError> {
		let market_id = MarketIndex::new(market_id);
		let total_borrowed_from_market_excluding_interest =
			Pallet::<T>::total_borrowed_from_market_excluding_interest(&market_id)?;
		let total_available_to_be_borrowed =
			Pallet::<T>::total_available_to_be_borrowed(&market_id)?;
		let utilization_ratio = Pallet::<T>::calculate_utilization_ratio(
			total_available_to_be_borrowed,
			total_borrowed_from_market_excluding_interest,
		)?;

		Markets::<T>::try_get(market_id)
			.map_err(|_| Error::<T>::MarketDoesNotExist)?
			.interest_rate_model
			.get_borrow_rate(utilization_ratio)
			.ok_or(Error::<T>::BorrowRateDoesNotExist)
			.map_err(Into::into)
	}
}
