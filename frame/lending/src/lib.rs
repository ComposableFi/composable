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

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

mod models;

pub use crate::weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use crate::{models::BorrowerData, weights::WeightInfo};
	use codec::{Codec, FullCodec};
	use composable_traits::{
		currency::{CurrencyFactory, PriceableAsset},
		lending::{BorrowAmountOf, CollateralLpAmountOf, Lending, MarketConfig, MarketConfigInput},
		liquidation::Liquidation,
		loans::{DurationSeconds, PriceStructure, Timestamp},
		math::{LiftedFixedBalance, SafeArithmetic},
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
			UnixTime,
		},
		transactional, PalletId,
	};
	use frame_system::{
		offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer},
		pallet_prelude::*,
	};
	use num_traits::{CheckedDiv, SaturatingSub};
	use scale_info::TypeInfo;
	use sp_core::crypto::KeyTypeId;
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
			Saturating, Zero,
		},
		ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128,
		KeyTypeId as CryptoKeyTypeId, Percent, Perquintill,
	};
	use sp_std::{fmt::Debug, vec, vec::Vec};

	type MarketConfiguration<T> = MarketConfig<
		<T as Config>::VaultId,
		<T as Config>::AssetId,
		<T as frame_system::Config>::AccountId,
		<T as Config>::GroupId,
	>;

	#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq, TypeInfo)]
	#[repr(transparent)]
	pub struct MarketIndex(u32);

	impl MarketIndex {
		pub fn new(i: u32) -> Self {
			Self(i)
		}
	}

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

	pub const PALLET_ID: PalletId = PalletId(*b"Lending!");
	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"lend");
	pub const CRYPTO_KEY_TYPE: CryptoKeyTypeId = CryptoKeyTypeId(*b"lend");

	pub mod crypto {
		use super::KEY_TYPE;
		use sp_core::sr25519::Signature as Sr25519Signature;
		use sp_runtime::{
			app_crypto::{app_crypto, sr25519},
			traits::Verify,
			MultiSignature, MultiSigner,
		};
		app_crypto!(sr25519, KEY_TYPE);

		pub struct TestAuthId;

		// implementation for runtime
		impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}

		// implementation for mock runtime in test
		impl
			frame_system::offchain::AppCrypto<
				<Sr25519Signature as Verify>::Signer,
				Sr25519Signature,
			> for TestAuthId
		{
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}
	}

	#[pallet::config]
	#[cfg(not(feature = "runtime-benchmarks"))]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Oracle: Oracle<AssetId = <Self as Config>::AssetId, Balance = Self::Balance>;
		type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;
		type Vault: StrategicVault<
			VaultId = Self::VaultId,
			AssetId = <Self as Config>::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
		>;

		type CurrencyFactory: CurrencyFactory<<Self as Config>::AssetId>;
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo
			+ PriceableAsset;
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

		/// vault owned - can transfer, cannot mint
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>;

		/// market owned - debt token can be minted
		type MarketDebtCurrency: Transfer<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>
			+ Mutate<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>
			+ MutateHold<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>
			+ InspectHold<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>;

		type Liquidation: Liquidation<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			GroupId = Self::GroupId,
		>;

		type UnixTime: UnixTime;
		type MaxLendingCount: Get<u32>;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		type WeightInfo: WeightInfo;
		type GroupId: FullCodec + Default + PartialEq + Clone + Debug + TypeInfo;
	}
	#[cfg(feature = "runtime-benchmarks")]
	pub trait Config:
		CreateSignedTransaction<Call<Self>> + frame_system::Config + pallet_oracle::Config
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Oracle: Oracle<AssetId = <Self as Config>::AssetId, Balance = Self::Balance>;
		type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter + From<u64>;
		type Vault: StrategicVault<
			VaultId = Self::VaultId,
			AssetId = <Self as Config>::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
		>;

		type CurrencyFactory: CurrencyFactory<<Self as Config>::AssetId>;
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ From<u128>
			+ Debug
			+ Default
			+ TypeInfo;
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

		/// vault owned - can transfer, cannot mint
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>;

		/// market owned - debt token can be minted
		type MarketDebtCurrency: Transfer<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>
			+ Mutate<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>
			+ MutateHold<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>
			+ InspectHold<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>;

		type Liquidation: Liquidation<
			AssetId = <Self as Config>::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
		>;
		type UnixTime: UnixTime;
		type MaxLendingCount: Get<u32>;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		type WeightInfo: WeightInfo;

		type GroupId;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
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
				<T as Config>::WeightInfo::accrue_interest();
			weight += u64::from(call_counters.account_id) * <T as Config>::WeightInfo::account_id();
			weight += u64::from(call_counters.available_funds) *
				<T as Config>::WeightInfo::available_funds();
			weight += u64::from(call_counters.handle_withdrawable) *
				<T as Config>::WeightInfo::handle_withdrawable();
			weight += u64::from(call_counters.handle_depositable) *
				<T as Config>::WeightInfo::handle_depositable();
			weight += u64::from(call_counters.handle_must_liquidate) *
				<T as Config>::WeightInfo::handle_must_liquidate();

			// TODO: move following loop to OCW
			for (market_id, account, _) in DebtIndex::<T>::iter() {
				if Self::liquidate_internal(&market_id, &account).is_ok() {
					Self::deposit_event(Event::LiquidationInitiated { market_id, account });
				}
			}

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
				let results = signer.send_signed_transaction(|_account| {
					// call `liquidate` extrinsic
					Call::liquidate { market_id, borrower: account.clone() }
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
		/// Only assets for which we can track price are supported
		AssetNotSupportedByOracle,
		AssetPriceNotFound,
		/// The market could not be found
		MarketDoesNotExist,
		CollateralDepositFailed,
		MarketCollateralWasNotDepositedByAccount,
		CollateralFactorIsLessOrEqualOne,
		MarketAndAccountPairNotFound,
		NotEnoughCollateralToBorrowAmount,
		MarketIsClosing,
		InvalidTimestampOnBorrowRequest,
		NotEnoughBorrowAsset,
		NotEnoughCollateral,
		TransferFailed,
		CannotWithdrawFromProvidedBorrowAccount,
		CannotRepayMoreThanBorrowAmount,
		BorrowRateDoesNotExist,
		BorrowIndexDoesNotExist,
		BorrowAndRepayInSameBlockIsNotSupported,
		BorrowDoesNotExist,
		RepayAmountMustBeGraterThanZero,
		ExceedLendingCount,
		LiquidationFailed,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when new lending market is created.
		NewMarketCreated {
			market_id: MarketIndex,
			vault_id: T::VaultId,
			manager: T::AccountId,
			borrow_asset_id: <T as Config>::AssetId,
			collateral_asset_id: <T as Config>::AssetId,
			reserved_factor: Perquintill,
			collateral_factor: NormalizedCollateralFactor,
		},
		/// Event emitted when collateral is deposited.
		CollateralDeposited { sender: T::AccountId, market_id: MarketIndex, amount: T::Balance },
		/// Event emitted when collateral is withdrawed.
		CollateralWithdrawed { sender: T::AccountId, market_id: MarketIndex, amount: T::Balance },
		/// Event emitted when user borrows from given market.
		Borrowed { sender: T::AccountId, market_id: MarketIndex, amount: T::Balance },
		/// Event emitted when user repays borrow of beneficiary in given market.
		RepaidBorrow {
			sender: T::AccountId,
			market_id: MarketIndex,
			beneficiary: T::AccountId,
			amount: T::Balance,
		},
		/// Event emitted when a liquidation is initiated for a loan.
		LiquidationInitiated { market_id: MarketIndex, account: T::AccountId },
		/// Event emitted to warn that loan may go under collaterized soon.
		SoonMayUnderCollaterized { market_id: MarketIndex, account: T::AccountId },
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
		MarketConfig<T::VaultId, <T as Config>::AssetId, T::AccountId, T::GroupId>,
		ValueQuery,
	>;

	/// Original debt values are on balances.
	/// Debt token allows to simplify some debt management and implementation of features
	#[pallet::storage]
	#[pallet::getter(fn debt_currencies)]

	pub type DebtMarkets<T: Config> =
		StorageMap<_, Twox64Concat, MarketIndex, <T as Config>::AssetId, ValueQuery>;

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

	/// Latest timestamp at which account borrowed from market.
	#[pallet::storage]
	#[pallet::getter(fn borrow_timestamp)]
	pub type BorrowTimestamp<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		MarketIndex,
		Twox64Concat,
		T::AccountId,
		Timestamp,
		OptionQuery,
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
	#[derive(Default)]
	pub struct GenesisConfig {}

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

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new lending market.
		/// - `origin` : Sender of this extrinsic. (Also manager for new market to be created.)
		/// - `collateral_asset_id` : AssetId for collateral.
		/// - `reserved_factor` : Reserve factor of market to be created.
		/// - `collateral_factor` : Collateral factor of market to be created.
		/// - `under_collaterized_warn_percent` : warn borrower when loan's collateral/debt ratio
		///   given percentage short to be under collaterized
		#[pallet::weight(<T as Config>::WeightInfo::create_new_market())]
		#[transactional]
		#[allow(clippy::too_many_arguments)]
		pub fn create_new_market(
			origin: OriginFor<T>,
			borrow_asset_id: <T as Config>::AssetId,
			collateral_asset_id: <T as Config>::AssetId,
			reserved_factor: Perquintill,
			collateral_factor: NormalizedCollateralFactor,
			under_collaterized_warn_percent: Percent,
			interest_rate_model: InterestRateModel,
			liquidator: Option<T::GroupId>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let market_config = MarketConfigInput {
				reserved: reserved_factor,
				manager: who.clone(),
				collateral_factor,
				under_collaterized_warn_percent,
				liquidator,
			};
			let (market_id, vault_id) = Self::create(
				borrow_asset_id,
				collateral_asset_id,
				market_config,
				&interest_rate_model,
			)?;
			Self::deposit_event(Event::<T>::NewMarketCreated {
				market_id,
				vault_id,
				manager: who,
				borrow_asset_id,
				collateral_asset_id,
				reserved_factor,
				collateral_factor,
			});
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
			Self::deposit_collateral_internal(&market_id, &sender, amount)?;
			Self::deposit_event(Event::<T>::CollateralDeposited { sender, market_id, amount });
			Ok(().into())
		}

		/// Withdraw collateral from market.
		/// - `origin` : Sender of this extrinsic.
		/// - `market_id` : Market index from which collateral will be withdraw.
		/// - `amount` : Amount of collateral to be withdrawed.
		#[pallet::weight(<T as Config>::WeightInfo::withdraw_collateral())]
		#[transactional]
		pub fn withdraw_collateral(
			origin: OriginFor<T>,
			market_id: MarketIndex,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			Self::withdraw_collateral_internal(&market_id, &sender, amount)?;
			Self::deposit_event(Event::<T>::CollateralWithdrawed { sender, market_id, amount });
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
			Self::borrow_internal(&market_id, &sender, amount_to_borrow)?;
			Self::deposit_event(Event::<T>::Borrowed {
				sender,
				market_id,
				amount: amount_to_borrow,
			});
			Ok(().into())
		}

		/// Repay borrow for beneficiary account.
		/// - `origin` : Sender of this extrinsic. (Also the user who repays beneficiary's borrow.)
		/// - `market_id` : Market index to which user wants to repay borrow.
		/// - `beneficiary` : AccountId which has borrowed asset. (This can be same or differnt than
		/// origin).
		/// - `repay_amount` : Amount which user wants to borrow.
		#[pallet::weight(<T as Config>::WeightInfo::repay_borrow())]
		#[transactional]
		pub fn repay_borrow(
			origin: OriginFor<T>,
			market_id: MarketIndex,
			beneficiary: T::AccountId,
			repay_amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			Self::repay_borrow_internal(&market_id, &sender, &beneficiary, Some(repay_amount))?;
			Self::deposit_event(Event::<T>::RepaidBorrow {
				sender,
				market_id,
				beneficiary,
				amount: repay_amount,
			});
			Ok(().into())
		}

		/// Check if borrow for `borrower` account is required to be liquidated, initiate
		/// liquidation.
		/// - `origin` : Sender of this extrinsic.
		/// - `market_id` : Market index from which `borrower` has taken borrow.
		#[pallet::weight(1000)]
		#[transactional]
		pub fn liquidate(
			origin: OriginFor<T>,
			market_id: MarketIndex,
			borrower: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let _sender = ensure_signed(origin)?;
			// TODO: should this be restricted to certain users?
			Self::liquidate_internal(&market_id, &borrower)?;
			Self::deposit_event(Event::LiquidationInitiated { market_id, account: borrower });
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn total_interest_accurate(
			market_id: &<Self as Lending>::MarketId,
		) -> Result<u128, DispatchError> {
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let total_interest =
				T::MarketDebtCurrency::balance(debt_asset_id, &Self::account_id(market_id));
			Ok(total_interest)
		}

		pub fn account_id(market_id: &<Self as Lending>::MarketId) -> <Self as Lending>::AccountId {
			<Self as Lending>::account_id(market_id)
		}
		pub fn calc_utilization_ratio(
			cash: &<Self as Lending>::Balance,
			borrows: &<Self as Lending>::Balance,
		) -> Result<Percent, DispatchError> {
			<Self as Lending>::calc_utilization_ratio(cash, borrows)
		}
		pub fn create(
			borrow_asset: <Self as Lending>::AssetId,
			collateral_asset: <Self as Lending>::AssetId,
			config_input: MarketConfigInput<<Self as Lending>::AccountId, T::GroupId>,
			interest_rate_model: &InterestRateModel,
		) -> Result<(<Self as Lending>::MarketId, <Self as Lending>::VaultId), DispatchError> {
			<Self as Lending>::create(
				borrow_asset,
				collateral_asset,
				config_input,
				interest_rate_model,
			)
		}
		pub fn deposit_collateral_internal(
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
		pub fn withdraw_collateral_internal(
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

		pub fn borrow_internal(
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

		pub fn repay_borrow_internal(
			market_id: &<Self as Lending>::MarketId,
			from: &<Self as Lending>::AccountId,
			beneficiary: &<Self as Lending>::AccountId,
			repay_amount: Option<BorrowAmountOf<Self>>,
		) -> Result<(), DispatchError> {
			<Self as Lending>::repay_borrow(market_id, from, beneficiary, repay_amount)
		}

		pub fn create_borrower_data(
			market_id: &<Self as Lending>::MarketId,
			account: &<Self as Lending>::AccountId,
		) -> Result<BorrowerData, DispatchError> {
			let market = Self::get_market(market_id)?;
			let collateral_balance = Self::collateral_of_account(market_id, account)?;
			let collateral_balance_value = Self::get_price(market.collateral, collateral_balance)?;
			let borrow_asset = T::Vault::asset_id(&market.borrow)?;
			let borrower_balance_with_interest = Self::borrow_balance_current(market_id, account)?
				.unwrap_or_else(BorrowAmountOf::<Self>::zero);
			let borrow_balance_value =
				Self::get_price(borrow_asset, borrower_balance_with_interest)?;
			let borrower = BorrowerData::new(
				collateral_balance_value,
				borrow_balance_value,
				market.collateral_factor,
				market.under_collaterized_warn_percent,
			);
			Ok(borrower)
		}

		pub fn should_liquidate(
			market_id: &<Self as Lending>::MarketId,
			account: &<Self as Lending>::AccountId,
		) -> Result<bool, DispatchError> {
			let borrower = Self::create_borrower_data(market_id, account)?;
			let should_liquidate = borrower.should_liquidate()?;
			Ok(should_liquidate)
		}

		pub fn soon_under_collaterized(
			market_id: &<Self as Lending>::MarketId,
			account: &<Self as Lending>::AccountId,
		) -> Result<bool, DispatchError> {
			let borrower = Self::create_borrower_data(market_id, account)?;
			let should_warn = borrower.should_warn()?;
			Ok(should_warn)
		}

		/// if liquidation is not required returns `ok(false)`
		/// if liquidation is required and `liquidate` is successful then return `Ok(true)`
		/// if there is any error then propagate that error.
		pub fn liquidate_internal(
			market_id: &<Self as Lending>::MarketId,
			account: &<Self as Lending>::AccountId,
		) -> Result<(), DispatchError> {
			if Self::should_liquidate(market_id, account)? {
				let market = Self::get_market(market_id)?;
				let borrow_asset_id = T::Vault::asset_id(&market.borrow)?;
				let collateral_to_liquidate = Self::collateral_of_account(market_id, account)?;
				let collateral_price =
					Self::get_price(market.collateral, market.collateral.unit())?;
				let source_target_account = Self::account_id(market_id);
				T::Liquidation::liquidate(
					&source_target_account,
					market.collateral,
					PriceStructure::new(collateral_price),
					borrow_asset_id,
					&Self::account_id(market_id),
					collateral_to_liquidate,
					//borrow_asset,
					//&source_target_account,
					// collateral_to_liquidate,
				)
				.map(|_| ())
			} else {
				Ok(())
			}
		}

		pub(crate) fn initialize_block(
			block_number: T::BlockNumber,
		) -> InitializeBlockCallCounters {
			let mut call_counters = InitializeBlockCallCounters::default();
			with_transaction(|| {
				let now = Self::now();
				call_counters.now += 1;
				let results = Markets::<T>::iter()
					.map(|(market_id, config)| {
						call_counters.read_markets += 1;
						Self::accrue_interest(&market_id, now)?;
						call_counters.accrue_interest += 1;
						let market_account = Self::account_id(&market_id);
						/* NOTE(hussein-aitlahcen):
						 It would probably be more perfomant to handle theses
						 case while borrowing/repaying.

						 I don't know whether we would face any issue by doing that.

						 borrow:
						   - withdrawable = transfer(vault->market) + transfer(market->user)
						   - depositable = error(not enough borrow asset) // vault asking for reserve to be fullfilled
						   - mustliquidate = error(market is closing)
						 repay:
							- (withdrawable || depositable || mustliquidate)
							  = transfer(user->market) + transfer(market->vault)

						 The intermediate transfer(vault->market) while borrowing would
						 allow the vault to update the strategy balance (market = borrow vault strategy).
						*/
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
						}

						call_counters.available_funds += 1;

						Ok(())
					})
					.collect::<Vec<Result<(), DispatchError>>>();
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
			call_counters
		}

		pub(crate) fn now() -> u64 {
			T::UnixTime::now().as_secs()
		}

		pub(crate) fn available_funds(
			config: &MarketConfiguration<T>,
			market_account: &T::AccountId,
		) -> Result<FundsAvailability<T::Balance>, DispatchError> {
			<T::Vault as StrategicVault>::available_funds(&config.borrow, market_account)
		}

		pub(crate) fn handle_withdrawable(
			config: &MarketConfiguration<T>,
			market_account: &T::AccountId,
			balance: T::Balance,
		) -> Result<(), DispatchError> {
			<T::Vault as StrategicVault>::withdraw(&config.borrow, market_account, balance)
		}

		pub(crate) fn handle_depositable(
			config: &MarketConfiguration<T>,
			market_account: &T::AccountId,
			balance: T::Balance,
		) -> Result<(), DispatchError> {
			let asset_id = <T::Vault as Vault>::asset_id(&config.borrow)?;
			let balance =
				<T as Config>::Currency::reducible_balance(asset_id, market_account, false)
					.min(balance);
			<T::Vault as StrategicVault>::deposit(&config.borrow, market_account, balance)
		}

		pub(crate) fn handle_must_liquidate(
			config: &MarketConfiguration<T>,
			market_account: &T::AccountId,
		) -> Result<(), DispatchError> {
			let asset_id = <T::Vault as Vault>::asset_id(&config.borrow)?;
			let balance =
				<T as Config>::Currency::reducible_balance(asset_id, market_account, false);
			<T::Vault as StrategicVault>::deposit(&config.borrow, market_account, balance)
		}

		fn get_market(market_id: &MarketIndex) -> Result<MarketConfiguration<T>, DispatchError> {
			Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist.into())
		}

		fn get_borrow_index(market_id: &MarketIndex) -> Result<FixedU128, DispatchError> {
			BorrowIndex::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist.into())
		}

		fn get_price(
			asset_id: <T as Config>::AssetId,
			amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			<T::Oracle as Oracle>::get_price(asset_id, amount)
				.map(|p| p.price)
				.map_err(|_| Error::<T>::AssetPriceNotFound.into())
		}

		fn updated_account_interest_index(
			market_id: &MarketIndex,
			debt_owner: &T::AccountId,
			amount: T::Balance,
		) -> Result<FixedU128, DispatchError> {
			let market_index =
				BorrowIndex::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let account_interest_index =
				DebtIndex::<T>::try_get(market_id, debt_owner).map_or(Ratio::zero(), |index| index);
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let existing_borrow_amount = T::MarketDebtCurrency::balance(debt_asset_id, debt_owner);
			let amount_to_borrow: u128 = amount.into();
			let amount_to_borrow = amount_to_borrow
				.checked_mul(LiftedFixedBalance::accuracy())
				.ok_or(Error::<T>::Overflow)?;
			T::MarketDebtCurrency::mint_into(debt_asset_id, debt_owner, amount_to_borrow)?;
			T::MarketDebtCurrency::hold(debt_asset_id, debt_owner, amount_to_borrow)?;
			let total_borrow_amount = existing_borrow_amount
				.checked_add(amount_to_borrow)
				.ok_or(Error::<T>::Overflow)?;
			let existing_borrow_share =
				Percent::from_rational(existing_borrow_amount, total_borrow_amount);
			let new_borrow_share = Percent::from_rational(amount_to_borrow, total_borrow_amount);
			Ok((market_index * new_borrow_share.into()) +
				(account_interest_index * existing_borrow_share.into()))
		}

		fn can_borrow(
			market_id: &MarketIndex,
			debt_owner: &T::AccountId,
			amount_to_borrow: BorrowAmountOf<Self>,
			asset_id: <T as Config>::AssetId,
			market: MarketConfiguration<T>,
			market_account: &T::AccountId,
		) -> Result<(), DispatchError> {
			let latest_borrow_timestamp = BorrowTimestamp::<T>::get(market_id, debt_owner);
			if let Some(time) = latest_borrow_timestamp {
				if time >= Self::last_block_timestamp() {
					return Err(Error::<T>::InvalidTimestampOnBorrowRequest.into())
				}
			}

			let borrow_asset = T::Vault::asset_id(&market.borrow)?;
			let borrow_limit_value = Self::get_borrow_limit(market_id, debt_owner)?;
			let borrow_amount_value = Self::get_price(borrow_asset, amount_to_borrow)?;
			ensure!(
				borrow_limit_value >= borrow_amount_value,
				Error::<T>::NotEnoughCollateralToBorrowAmount
			);

			ensure!(
				<T as Config>::Currency::can_withdraw(asset_id, market_account, amount_to_borrow)
					.into_result()
					.is_ok(),
				Error::<T>::NotEnoughBorrowAsset,
			);
			ensure!(
				!matches!(
					T::Vault::available_funds(&market.borrow, market_account)?,
					FundsAvailability::MustLiquidate
				),
				Error::<T>::MarketIsClosing
			);

			Ok(())
		}

		fn can_repay_borrow(
			market_id: &MarketIndex,
			from: &T::AccountId,
			beneficiary: &T::AccountId,
			repay_amount: BorrowAmountOf<Self>,
			owed: BorrowAmountOf<Self>,
			borrow_asset_id: <T as Config>::AssetId,
			market_account: &T::AccountId,
		) -> Result<(), DispatchError> {
			let latest_borrow_timestamp = BorrowTimestamp::<T>::get(market_id, beneficiary);
			ensure!(latest_borrow_timestamp.is_some(), Error::<T>::BorrowDoesNotExist);
			if let Some(timestamp) = latest_borrow_timestamp {
				ensure!(
					timestamp != Self::last_block_timestamp(),
					Error::<T>::BorrowAndRepayInSameBlockIsNotSupported
				);
			}
			ensure!(
				repay_amount > <Self as Lending>::Balance::zero(),
				Error::<T>::RepayAmountMustBeGraterThanZero
			);
			ensure!(repay_amount <= owed, Error::<T>::CannotRepayMoreThanBorrowAmount);
			ensure!(
				<T as Config>::Currency::can_withdraw(borrow_asset_id, from, repay_amount)
					.into_result()
					.is_ok(),
				Error::<T>::CannotWithdrawFromProvidedBorrowAccount
			);
			ensure!(
				<T as Config>::Currency::can_deposit(borrow_asset_id, market_account, repay_amount)
					.into_result()
					.is_ok(),
				Error::<T>::TransferFailed
			);

			Ok(())
		}
	}

	impl<T: Config> Lending for Pallet<T> {
		/// we are operating only on vault types, so restricted by these
		type AssetId = <T as Config>::AssetId;
		type VaultId = <T::Vault as Vault>::VaultId;
		type AccountId = <T::Vault as Vault>::AccountId;
		type Balance = T::Balance;

		type MarketId = MarketIndex;

		type BlockNumber = T::BlockNumber;
		type GroupId = T::GroupId;

		fn create(
			borrow_asset: Self::AssetId,
			collateral_asset: Self::AssetId,
			config_input: MarketConfigInput<Self::AccountId, Self::GroupId>,
			interest_rate_model: &InterestRateModel,
		) -> Result<(Self::MarketId, Self::VaultId), DispatchError> {
			ensure!(
				config_input.collateral_factor > 1.into(),
				Error::<T>::CollateralFactorIsLessOrEqualOne
			);

			let collateral_asset_supported = <T::Oracle as Oracle>::is_supported(collateral_asset)?;
			let borrow_asset_supported = <T::Oracle as Oracle>::is_supported(borrow_asset)?;
			ensure!(
				collateral_asset_supported && borrow_asset_supported,
				Error::<T>::AssetNotSupportedByOracle
			);

			LendingCount::<T>::try_mutate(|MarketIndex(previous_market_index)| {
				let market_id = {
					*previous_market_index += 1;
					ensure!(
						*previous_market_index <= T::MaxLendingCount::get(),
						Error::<T>::ExceedLendingCount
					);
					MarketIndex(*previous_market_index)
				};

				let borrow_asset_vault = T::Vault::create(
					Deposit::Existential,
					VaultConfig {
						asset_id: borrow_asset,
						reserved: config_input.reserved,
						manager: config_input.manager.clone(),
						strategies: [(
							Self::account_id(&market_id),
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
					interest_rate_model: *interest_rate_model,
					under_collaterized_warn_percent: config_input.under_collaterized_warn_percent,
					liquidator: config_input.liquidator,
				};

				let debt_asset_id = T::CurrencyFactory::create()?;
				DebtMarkets::<T>::insert(market_id, debt_asset_id);
				Markets::<T>::insert(market_id, config);
				BorrowIndex::<T>::insert(market_id, Ratio::one());

				Ok((market_id, borrow_asset_vault))
			})
		}

		fn account_id(market_id: &Self::MarketId) -> Self::AccountId {
			PALLET_ID.into_sub_account(market_id)
		}

		fn get_markets_for_borrow(borrow: Self::VaultId) -> Vec<Self::MarketId> {
			// allow to be slow until it becomes write transaction (not the case now and then)
			let mut markets = vec![];
			for (index, market) in Markets::<T>::iter() {
				if market.borrow == borrow {
					markets.push(index);
				}
			}

			markets
		}

		fn get_all_markets() -> Vec<(Self::MarketId, MarketConfiguration<T>)> {
			Markets::<T>::iter().map(|(index, config)| (index, config)).collect()
		}

		fn borrow(
			market_id: &Self::MarketId,
			debt_owner: &Self::AccountId,
			amount_to_borrow: BorrowAmountOf<Self>,
		) -> Result<(), DispatchError> {
			let market = Self::get_market(market_id)?;
			let borrow_asset = T::Vault::asset_id(&market.borrow)?;
			let market_account = Self::account_id(market_id);

			Self::can_borrow(
				market_id,
				debt_owner,
				amount_to_borrow,
				borrow_asset,
				market,
				&market_account,
			)?;

			let new_account_interest_index =
				Self::updated_account_interest_index(market_id, debt_owner, amount_to_borrow)?;

			<T as Config>::Currency::transfer(
				borrow_asset,
				&market_account,
				debt_owner,
				amount_to_borrow,
				true,
			)?;
			DebtIndex::<T>::insert(market_id, debt_owner, new_account_interest_index);
			BorrowTimestamp::<T>::insert(market_id, debt_owner, Self::last_block_timestamp());

			Ok(())
		}

		fn repay_borrow(
			market_id: &Self::MarketId,
			from: &Self::AccountId,
			beneficiary: &Self::AccountId,
			repay_amount: Option<BorrowAmountOf<Self>>,
		) -> Result<(), DispatchError> {
			let market = Self::get_market(market_id)?;
			if let Some(owed) = Self::borrow_balance_current(market_id, beneficiary)? {
				let repay_amount = repay_amount.unwrap_or(owed);
				let borrow_asset_id = T::Vault::asset_id(&market.borrow)?;
				let market_account = Self::account_id(market_id);

				Self::can_repay_borrow(
					market_id,
					from,
					beneficiary,
					repay_amount,
					owed,
					borrow_asset_id,
					&market_account,
				)?;

				let debt_asset_id = DebtMarkets::<T>::get(market_id);

				let burn_amount: u128 =
					<T as Config>::Currency::balance(debt_asset_id, beneficiary).into();
				let total_repay_amount: u128 = repay_amount.into();
				let mut remaining_borrow_amount =
					T::MarketDebtCurrency::balance(debt_asset_id, &market_account);
				if total_repay_amount <= burn_amount {
					// only repay interest
					T::MarketDebtCurrency::release(
						debt_asset_id,
						beneficiary,
						total_repay_amount,
						true,
					)
					.expect("can always release held debt balance");
					T::MarketDebtCurrency::burn_from(
						debt_asset_id,
						beneficiary,
						total_repay_amount,
					)
					.expect("can always burn debt balance");
				} else {
					let repay_borrow_amount = total_repay_amount - burn_amount;

					remaining_borrow_amount -= repay_borrow_amount;
					T::MarketDebtCurrency::burn_from(debt_asset_id, &market_account, repay_borrow_amount).expect(
						"debt balance of market must be of parts of debts of borrowers and can reduce it",
					);
					T::MarketDebtCurrency::release(debt_asset_id, beneficiary, burn_amount, true)
						.expect("can always release held debt balance");
					T::MarketDebtCurrency::burn_from(debt_asset_id, beneficiary, burn_amount)
						.expect("can always burn debt balance");
				}
				// TODO: fuzzing is must to uncover cases when sum != total
				<T as Config>::Currency::transfer(
					borrow_asset_id,
					from,
					&market_account,
					repay_amount,
					false,
				)
				.expect("must be able to transfer because of above checks");

				if remaining_borrow_amount == 0 {
					BorrowTimestamp::<T>::remove(market_id, beneficiary);
					DebtIndex::<T>::remove(market_id, beneficiary);
				}
			}

			Ok(())
		}

		fn total_borrows(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let accrued_debt =
				T::MarketDebtCurrency::balance(debt_asset_id, &Self::account_id(market_id));
			let total_issued = T::MarketDebtCurrency::total_issuance(debt_asset_id);
			let total_borrows = (total_issued - accrued_debt) / LiftedFixedBalance::accuracy();
			Ok((total_borrows as u64).into())
		}

		fn total_interest(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let total_interest =
				T::MarketDebtCurrency::balance(debt_asset_id, &Self::account_id(market_id));
			Ok(((total_interest / LiftedFixedBalance::accuracy()) as u64).into())
		}

		fn total_cash(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			let market = Self::get_market(market_id)?;
			let borrow_id = T::Vault::asset_id(&market.borrow)?;
			Ok(<T as Config>::Currency::balance(borrow_id, &Self::account_id(market_id)))
		}

		fn calc_utilization_ratio(
			cash: &Self::Balance,
			borrows: &Self::Balance,
		) -> Result<Percent, DispatchError> {
			Ok(composable_traits::rate_model::calc_utilization_ratio(
				(*cash).into(),
				(*borrows).into(),
			)?)
		}

		fn accrue_interest(
			market_id: &Self::MarketId,
			now: Timestamp,
		) -> Result<(), DispatchError> {
			// we maintain original borrow principals intact on hold,
			// but accrue total borrow balance by adding to market debt balance
			// when user pays loan back, we reduce marked accrued debt
			// so no need to loop over each account -> scales to millions of users

			let total_borrows = Self::total_borrows(market_id)?;
			let total_cash = Self::total_cash(market_id)?;
			let utilization_ratio = Self::calc_utilization_ratio(&total_cash, &total_borrows)?;
			let mut market = Self::get_market(market_id)?;
			let delta_time =
				now.checked_sub(Self::last_block_timestamp()).ok_or(Error::<T>::Underflow)?;
			let borrow_index = Self::get_borrow_index(market_id)?;
			let debt_asset_id = DebtMarkets::<T>::get(market_id);

			let (accrued, borrow_index_new) = accrue_interest_internal::<T, InterestRateModel>(
				utilization_ratio,
				&mut market.interest_rate_model,
				borrow_index,
				delta_time,
				total_borrows.into(),
			)?;

			BorrowIndex::<T>::insert(market_id, borrow_index_new);
			T::MarketDebtCurrency::mint_into(debt_asset_id, &Self::account_id(market_id), accrued)?;

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
					let principal = T::MarketDebtCurrency::balance_on_hold(debt_asset_id, account);
					let market_interest_index = Self::get_borrow_index(market_id)?;

					let balance = borrow_from_principal::<T>(
						((principal / LiftedFixedBalance::accuracy()) as u64).into(),
						market_interest_index,
						account_interest_index,
					)?;

					Ok(balance.map(Into::into))
				},
				// no active borrow on  market for given account
				Err(()) => Ok(Some(BorrowAmountOf::<Self>::zero())),
			}
		}

		fn collateral_of_account(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<CollateralLpAmountOf<Self>, DispatchError> {
			AccountCollateral::<T>::try_get(market_id, account)
				.map_err(|_| Error::<T>::MarketCollateralWasNotDepositedByAccount.into())
		}

		fn collateral_required(
			market_id: &Self::MarketId,
			borrow_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let market = Self::get_market(market_id)?;
			let borrow_asset = T::Vault::asset_id(&market.borrow)?;
			let borrow_amount_value = Self::get_price(borrow_asset, borrow_amount)?;
			Ok(swap_back(borrow_amount_value.into(), &market.collateral_factor)?
				.checked_mul_int(1u64)
				.ok_or(ArithmeticError::Overflow)?
				.into())
		}

		fn get_borrow_limit(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			let collateral_balance = AccountCollateral::<T>::try_get(market_id, account)
				.unwrap_or_else(|_| CollateralLpAmountOf::<Self>::zero());

			if collateral_balance > T::Balance::zero() {
				let borrower = Self::create_borrower_data(market_id, account)?;
				Ok(borrower
					.borrow_for_collateral()
					.map_err(|_| Error::<T>::NotEnoughCollateralToBorrowAmount)?
					.checked_mul_int(1u64)
					.ok_or(ArithmeticError::Overflow)?
					.into())
			} else {
				Ok(Self::Balance::zero())
			}
		}

		fn deposit_collateral(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
			amount: CollateralLpAmountOf<Self>,
		) -> Result<(), DispatchError> {
			let market = Self::get_market(market_id)?;
			let market_account = Self::account_id(market_id);
			ensure!(
				<T as Config>::Currency::can_withdraw(market.collateral, account, amount)
					.into_result()
					.is_ok(),
				Error::<T>::TransferFailed
			);

			ensure!(
				<T as Config>::Currency::can_deposit(market.collateral, &market_account, amount) ==
					DepositConsequence::Success,
				Error::<T>::TransferFailed
			);

			AccountCollateral::<T>::try_mutate(market_id, account, |collateral_balance| {
				let new_collateral_balance =
					(*collateral_balance).checked_add(&amount).ok_or(Error::<T>::Overflow)?;
				*collateral_balance = new_collateral_balance;
				Result::<(), Error<T>>::Ok(())
			})?;
			<T as Config>::Currency::transfer(
				market.collateral,
				account,
				&market_account,
				amount,
				true,
			)
			.expect("impossible; qed;");
			Ok(())
		}

		fn withdraw_collateral(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
			amount: CollateralLpAmountOf<Self>,
		) -> Result<(), DispatchError> {
			let market = Self::get_market(market_id)?;

			let collateral_balance = AccountCollateral::<T>::try_get(market_id, account)
				.unwrap_or_else(|_| CollateralLpAmountOf::<Self>::zero());

			ensure!(amount <= collateral_balance, Error::<T>::NotEnoughCollateral);

			let borrow_asset = T::Vault::asset_id(&market.borrow)?;
			let borrower_balance_with_interest = Self::borrow_balance_current(market_id, account)?
				.unwrap_or_else(BorrowAmountOf::<Self>::zero);
			let borrow_balance_value =
				Self::get_price(borrow_asset, borrower_balance_with_interest)?;

			let collateral_balance_after_withdrawal_value =
				Self::get_price(market.collateral, collateral_balance.safe_sub(&amount)?)?;

			let borrower_after_withdrawal = BorrowerData::new(
				collateral_balance_after_withdrawal_value,
				borrow_balance_value,
				market.collateral_factor,
				market.under_collaterized_warn_percent,
			);

			ensure!(
				!borrower_after_withdrawal.should_liquidate()?,
				Error::<T>::NotEnoughCollateral
			);

			let market_account = Self::account_id(market_id);
			ensure!(
				<T as Config>::Currency::can_deposit(market.collateral, account, amount) ==
					DepositConsequence::Success,
				Error::<T>::TransferFailed
			);
			ensure!(
				<T as Config>::Currency::can_withdraw(market.collateral, &market_account, amount)
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
			<T as Config>::Currency::transfer(
				market.collateral,
				&market_account,
				account,
				amount,
				true,
			)
			.expect("impossible; qed;");
			Ok(())
		}
	}

	/// If borrowBalance = 0 then borrow index is likely also 0.
	/// Rather than failing the calculation with a division by 0, we immediately return 0 in
	/// this case.
	fn borrow_from_principal<T: Config>(
		principal: <T as Config>::Balance,
		market_interest_index: Ratio,
		account_interest_index: Ratio,
	) -> Result<Option<u64>, DispatchError> {
		if principal.is_zero() {
			return Ok(None)
		}
		let principal: LiftedFixedBalance = principal.into();
		let balance = principal
			.checked_mul(&market_interest_index)
			.and_then(|from_start_total| from_start_total.checked_div(&account_interest_index))
			.and_then(|x| x.checked_mul_int(1u64))
			.ok_or(ArithmeticError::Overflow)?;
		Ok(Some(balance))
	}

	/// given collateral information, how much of borrow asset can get?
	pub fn swap(
		collateral_balance: &LiftedFixedBalance,
		collateral_price: &LiftedFixedBalance,
		collateral_factor: &NormalizedCollateralFactor,
	) -> Result<LiftedFixedBalance, ArithmeticError> {
		collateral_balance.safe_mul(collateral_price)?.safe_div(collateral_factor)
	}

	pub fn swap_back(
		borrow_balance_value: LiftedFixedBalance,
		collateral_factor: &NormalizedCollateralFactor,
	) -> Result<LiftedFixedBalance, ArithmeticError> {
		borrow_balance_value.safe_mul(collateral_factor)
	}

	pub fn accrue_interest_internal<T: Config, I: InterestRate>(
		utilization_ratio: Percent,
		interest_rate_model: &mut I,
		borrow_index: Rate,
		delta_time: DurationSeconds,
		total_borrows: u128,
	) -> Result<(u128, Rate), DispatchError> {
		let borrow_rate = interest_rate_model
			.get_borrow_rate(utilization_ratio)
			.ok_or(Error::<T>::BorrowRateDoesNotExist)?;
		let borrow_index_new =
			increment_index(borrow_rate, borrow_index, delta_time)?.safe_add(&borrow_index)?;
		let delta_interest_rate = borrow_rate
			.safe_mul(&FixedU128::saturating_from_integer(delta_time))?
			.safe_div(&FixedU128::saturating_from_integer(SECONDS_PER_YEAR))?;

		let total_borrows = total_borrows.safe_mul(&LiftedFixedBalance::accuracy())?;
		let accrue_increment = LiftedFixedBalance::from_inner(total_borrows)
			.safe_mul(&delta_interest_rate)?
			.into_inner();
		Ok((accrue_increment, borrow_index_new))
	}
}
