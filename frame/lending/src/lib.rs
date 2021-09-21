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

pub use crate::weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;
	use codec::{Codec, FullCodec};
	use composable_traits::{
		currency::CurrencyFactory,
		lending::{BorrowAmountOf, CollateralLpAmountOf, Lending, MarketConfig, MarketConfigInput},
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
	use frame_system::pallet_prelude::*;
	use num_traits::{CheckedDiv, SaturatingSub};
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
			Saturating, Zero,
		},
		ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128, Percent, Perquintill,
	};
	use sp_std::{fmt::Debug, vec::Vec};

	use composable_traits::rate_model::{LiftedFixedBalance, SafeArithmetic};

	#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq)]
	#[repr(transparent)]
	pub struct MarketIndex(u32);

	impl MarketIndex {
		pub fn new(i: u32) -> Self {
			Self(i)
		}
	}

	pub const PALLET_ID: PalletId = PalletId(*b"Lending!");

	#[pallet::config]
	#[cfg(not(feature = "runtime-benchmarks"))]
	pub trait Config: frame_system::Config {
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

		/// vault owned - can transfer, cannot mint
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>;

		/// market owned - debt token can be minted
		type MarketDebtCurrency: Transfer<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>
			+ Mutate<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>
			+ MutateHold<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>
			+ InspectHold<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>;

		type UnixTime: UnixTime;
		type WeightInfo: WeightInfo;
	}
	#[cfg(feature = "runtime-benchmarks")]
	pub trait Config: frame_system::Config + pallet_oracle::Config {
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
			+ From<u128>
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

		/// vault owned - can transfer, cannot mint
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>;

		/// market owned - debt token can be minted
		type MarketDebtCurrency: Transfer<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>
			+ Mutate<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>
			+ MutateHold<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>
			+ InspectHold<Self::AccountId, Balance = u128, AssetId = <Self as Config>::AssetId>;

		type UnixTime: UnixTime;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			Self::initialize_block(block_number);
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
		CannotBorrowInCurrentSourceVaultState,
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
	}

	pub struct BorrowerData {
		pub collateral_balance: LiftedFixedBalance,
		pub collateral_price: LiftedFixedBalance,
		pub borrower_balance_with_interest: LiftedFixedBalance,
		pub borrow_price: LiftedFixedBalance,
		pub collateral_factor: NormalizedCollateralFactor,
	}

	impl BorrowerData {
		#[inline]
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
				collateral_factor,
			}
		}

		#[inline]
		pub fn collateral_over_borrow(&self) -> Result<LiftedFixedBalance, ArithmeticError> {
			let collateral = self.collateral_balance.safe_mul(&self.collateral_price)?;
			let borrowed = self
				.borrower_balance_with_interest
				.safe_mul(&self.borrow_price)?
				.safe_mul(&self.collateral_factor)?;
			collateral.safe_sub(&borrowed)
		}

		#[inline]
		pub fn borrow_for_collateral(&self) -> Result<LiftedFixedBalance, ArithmeticError> {
			let max_borrow =
				swap(&self.collateral_balance, &self.collateral_price, &self.collateral_factor)?;
			let borrowed = self.borrower_balance_with_interest.safe_mul(&self.borrow_price)?;
			Ok(max_borrow.saturating_sub(borrowed))
		}
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
		MarketConfig<T::VaultId, <T as Config>::AssetId, T::AccountId>,
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

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new lending market.
		/// - `origin` : Sender of this extrinsic. (Also manager for new market to be created.)
		/// - `collateral_asset_id` : AssetId for collateral.
		/// - `reserved_factor` : Reserve factor of market to be created.
		/// - `collateral_factor` : Collateral factor of market to be created.
		#[pallet::weight(1000)]
		#[transactional]
		pub fn create_new_market(
			origin: OriginFor<T>,
			borrow_asset_id: <T as Config>::AssetId,
			collateral_asset_id: <T as Config>::AssetId,
			reserved_factor: Perquintill,
			collateral_factor: NormalizedCollateralFactor,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let market_config = MarketConfigInput {
				reserved: reserved_factor,
				manager: who.clone(),
				collateral_factor,
			};
			let (market_id, vault_id) =
				Self::create(borrow_asset_id, collateral_asset_id, market_config)?;
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
		#[pallet::weight(1000)]
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
		#[pallet::weight(1000)]
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
			config_input: MarketConfigInput<<Self as Lending>::AccountId>,
		) -> Result<(<Self as Lending>::MarketId, <Self as Lending>::VaultId), DispatchError> {
			<Self as Lending>::create(borrow_asset, collateral_asset, config_input)
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

		pub(crate) fn initialize_block(block_number: T::BlockNumber) {
			with_transaction(|| {
				let now = T::UnixTime::now().as_secs();
				let results = Markets::<T>::iter()
					.map(|(market_id, config)| {
						Pallet::<T>::accrue_interest(&market_id, now)?;
						let market_account = Self::account_id(&market_id);
						match <T::Vault as StrategicVault>::available_funds(
							&config.borrow,
							&market_account,
						)? {
							FundsAvailability::Withdrawable(balance) => {
								<T::Vault as StrategicVault>::withdraw(
									&config.borrow,
									&market_account,
									balance,
								)?;
							},
							FundsAvailability::Depositable(balance) => {
								let asset_id = <T::Vault as Vault>::asset_id(&config.borrow)?;
								let balance = <T as Config>::Currency::reducible_balance(
									asset_id,
									&market_account,
									false,
								)
								.min(balance);
								<T::Vault as StrategicVault>::deposit(
									&config.borrow,
									&market_account,
									balance,
								)?;
							},
							FundsAvailability::MustLiquidate => {
								let asset_id = <T::Vault as Vault>::asset_id(&config.borrow)?;
								let balance = <T as Config>::Currency::reducible_balance(
									asset_id,
									&market_account,
									false,
								);
								<T::Vault as StrategicVault>::deposit(
									&config.borrow,
									&market_account,
									balance,
								)?;
							},
						}

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
				let market_id = {
					*previous_market_index += 1;
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
					interest_rate_model: InterestRateModel::default(),
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
			let mut markets = sp_std::vec![];
			for (index, market) in Markets::<T>::iter() {
				if market.borrow == borrow {
					markets.push(index);
				}
			}

			markets
		}

		#[allow(clippy::type_complexity)]
		fn get_all_markets(
		) -> Vec<(Self::MarketId, MarketConfig<T::VaultId, <T as Config>::AssetId, T::AccountId>)> {
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
			let latest_borrow_timestamp = BorrowTimestamp::<T>::get(market_id, debt_owner);
			if let Some(time) = latest_borrow_timestamp {
				if time >= Self::last_block_timestamp() {
					return Err(Error::<T>::InvalidTimestampOnBorrowRequest.into())
				}
			}
			let borrow_limit = Self::get_borrow_limit(market_id, debt_owner)?;
			ensure!(
				borrow_limit >= amount_to_borrow,
				Error::<T>::NotEnoughCollateralToBorrowAmount
			);
			let account_id = Self::account_id(market_id);
			let can_withdraw =
				<T as Config>::Currency::reducible_balance(asset_id, &account_id, true);
			ensure!(can_withdraw >= amount_to_borrow, Error::<T>::NotEnoughBorrowAsset);

			ensure!(
				!matches!(
					T::Vault::available_funds(&market.borrow, &Self::account_id(market_id))?,
					FundsAvailability::MustLiquidate
				),
				Error::<T>::CannotBorrowInCurrentSourceVaultState
			);

			<T as Config>::Currency::transfer(
				asset_id,
				&Self::account_id(market_id),
				debt_owner,
				amount_to_borrow,
				true,
			)?;

			let market_index =
				BorrowIndex::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let account_interest_index =
				DebtIndex::<T>::try_get(market_id, debt_owner).map_or(Ratio::zero(), |index| index);
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let existing_borrow_amount = T::MarketDebtCurrency::balance(debt_asset_id, debt_owner);
			let amount_to_borrow: u128 = amount_to_borrow.into();
			let amount_to_borrow = amount_to_borrow
				.checked_mul(LiftedFixedBalance::accuracy())
				.expect("more detailed currency");
			T::MarketDebtCurrency::mint_into(debt_asset_id, debt_owner, amount_to_borrow)?;
			T::MarketDebtCurrency::hold(debt_asset_id, debt_owner, amount_to_borrow)?;
			let total_borrow_amount = existing_borrow_amount
				.checked_add(amount_to_borrow)
				.ok_or(Error::<T>::Overflow)?;
			let existing_borrow_share =
				Percent::from_rational(existing_borrow_amount, total_borrow_amount);
			let new_borrow_share = Percent::from_rational(amount_to_borrow, total_borrow_amount);
			let new_account_interest_index = (market_index * new_borrow_share.into()) +
				(account_interest_index * existing_borrow_share.into());
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
			let latest_borrow_timestamp = BorrowTimestamp::<T>::get(market_id, beneficiary);
			if latest_borrow_timestamp.is_none() {
				return Err(Error::<T>::BorrowDoesNotExist.into())
			}
			if latest_borrow_timestamp.unwrap() == Self::last_block_timestamp() {
				return Err(Error::<T>::BorrowAndRepayInSameBlockIsNotSupported.into())
			}
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			if let Some(owed) = Self::borrow_balance_current(market_id, beneficiary)? {
				let repay_amount = repay_amount.unwrap_or(owed);
				let borrow_asset_id = T::Vault::asset_id(&market.borrow)?;
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
				let market_account = Self::account_id(market_id);
				ensure!(
					<T as Config>::Currency::can_deposit(
						borrow_asset_id,
						&market_account,
						repay_amount
					)
					.into_result()
					.is_ok(),
					Error::<T>::TransferFailed
				);

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
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let borrow_id = T::Vault::asset_id(&market.borrow)?;
			Ok(<T as Config>::Currency::balance(borrow_id, &T::Vault::account_id(&market.borrow)))
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

			// TODO: total_borrows calculation duplicated, remove
			let total_borrows = Self::total_borrows(market_id)?;
			let total_cash = Self::total_cash(market_id)?;
			let utilization_ratio = Self::calc_utilization_ratio(&total_cash, &total_borrows)?;
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let delta_time =
				now.checked_sub(Self::last_block_timestamp()).ok_or(Error::<T>::Underflow)?;
			let borrow_index =
				BorrowIndex::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let accrued_debt =
				T::MarketDebtCurrency::balance(debt_asset_id, &Self::account_id(market_id));
			let total_issued = T::MarketDebtCurrency::total_issuance(debt_asset_id);

			let (accrued, borrow_index_new) = accrue_interest_internal::<T>(
				utilization_ratio,
				&market.interest_rate_model,
				borrow_index,
				delta_time,
				total_issued,
				accrued_debt,
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
					let market_interest_index = BorrowIndex::<T>::try_get(market_id)
						.map_err(|_| Error::<T>::BorrowIndexDoesNotExist)?;

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
		) -> Result<Self::Balance, DispatchError> {
			AccountCollateral::<T>::try_get(market_id, account)
				.map_err(|_| Error::<T>::MarketCollateralWasNotDepositedByAccount.into())
		}

		fn collateral_required(
			market_id: &Self::MarketId,
			borrow_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let borrow_asset = T::Vault::asset_id(&market.borrow)?;
			let borrow_price = T::Oracle::get_price(&borrow_asset)?.0;
			Ok(swap_back(&borrow_amount.into(), &borrow_price.into(), &market.collateral_factor)?
				.checked_mul_int(1u64)
				.ok_or(ArithmeticError::Overflow)?
				.into())
		}

		fn get_borrow_limit(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let collateral_balance =
				AccountCollateral::<T>::try_get(market_id, account).unwrap_or_default();

			if collateral_balance > T::Balance::zero() {
				let collateral_price = T::Oracle::get_price(&market.collateral)?.0;
				let borrow_asset = T::Vault::asset_id(&market.borrow)?;
				let borrow_price = T::Oracle::get_price(&borrow_asset)?.0;
				let borrower_balance_with_interest =
					Self::borrow_balance_current(market_id, account)?.unwrap_or_default();

				let borrower = BorrowerData::new(
					collateral_balance,
					collateral_price,
					borrower_balance_with_interest,
					borrow_price,
					market.collateral_factor,
				);

				return Ok(borrower
					.borrow_for_collateral()
					.map_err(|_| Error::<T>::NotEnoughCollateralToBorrowAmount)?
					.checked_mul_int(1u64)
					.ok_or(ArithmeticError::Overflow)?
					.into())
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
				collateral_balance,
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
		borrow_balance: &LiftedFixedBalance,
		borrow_price: &LiftedFixedBalance,
		collateral_factor: &NormalizedCollateralFactor,
	) -> Result<LiftedFixedBalance, ArithmeticError> {
		borrow_balance.safe_mul(borrow_price)?.safe_mul(collateral_factor)
	}

	pub fn accrue_interest_internal<T: Config>(
		utilization_ratio: Percent,
		interest_rate_model: &InterestRateModel,
		borrow_index: Rate,
		delta_time: Duration,
		total_issued: u128,
		accrued_debt: u128,
	) -> Result<(u128, Rate), DispatchError> {
		let borrow_rate = interest_rate_model
			.get_borrow_rate(utilization_ratio)
			.ok_or(Error::<T>::BorrowRateDoesNotExist)?;
		let borrow_index_new =
			increment_index(borrow_rate, borrow_index, delta_time)?.safe_add(&borrow_index)?;
		let delta_interest_rate = borrow_rate
			.safe_mul(&FixedU128::saturating_from_integer(delta_time))?
			.safe_div(&FixedU128::saturating_from_integer(SECONDS_PER_YEAR))?;

		let total_borrows = total_issued - accrued_debt;

		let accrue_increment = LiftedFixedBalance::from_inner(total_borrows)
			.safe_mul(&delta_interest_rate)?
			.into_inner();
		Ok((accrue_increment, borrow_index_new))
	}
}
