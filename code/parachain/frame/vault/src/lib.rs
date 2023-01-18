#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
	dead_code,
	bad_style,
	bare_trait_objects,
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
// Some substrate macros are expanded in such a way that their items cannot be documented. For now,
// it's best to just set this to warn during development.
#![allow(missing_docs)]
#![doc = include_str!("../README.md")]

mod capabilities;
pub mod models;
mod rent;
mod traits;
mod validation;

pub use crate::weights::WeightInfo;
pub use capabilities::Capabilities;
pub use pallet::*;

pub mod mocks;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
pub mod weights;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use core::ops::AddAssign;

	use crate::{
		models::StrategyOverview,
		rent::{self, Verdict},
		traits::{CurrencyFactory, StrategicVault},
		validation::{ValidateCreationDeposit, ValidateMaxStrategies},
		weights::WeightInfo,
	};
	use codec::{Codec, FullCodec};
	use composable_support::validation::Validated;
	use composable_traits::{
		currency::RangeId,
		defi::Rate,
		vault::{
			CapabilityVault, Deposit, FundsAvailability, ReportableStrategicVault, Vault,
			VaultConfig,
		},
	};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		ensure,
		pallet_prelude::*,
		traits::{
			fungible::{
				Inspect as InspectNative, Mutate as MutateNative, MutateHold as MutateHoldNative,
				Transfer as TransferNative,
			},
			fungibles::{Inspect, Mutate, MutateHold, Transfer},
		},
		transactional, PalletId,
	};
	use frame_system::{
		ensure_root, ensure_signed, pallet_prelude::OriginFor, Config as SystemConfig,
	};
	use num_traits::{One, SaturatingSub};
	use scale_info::TypeInfo;
	use sp_arithmetic::Rounding;
	use sp_runtime::{
		helpers_128bit::multiply_by_rational_with_rounding,
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Convert,
			Zero,
		},
		ArithmeticError, DispatchError, FixedPointNumber, Perquintill,
	};
	use sp_std::{cmp::Ordering, fmt::Debug};

	#[allow(missing_docs)]
	pub type AssetIdOf<T> =
		<<T as Config>::Currency as Inspect<<T as SystemConfig>::AccountId>>::AssetId;
	#[allow(missing_docs)]
	pub type AccountIdOf<T> = <T as SystemConfig>::AccountId;
	#[allow(missing_docs)]
	pub type BlockNumberOf<T> = <T as SystemConfig>::BlockNumber;
	#[allow(missing_docs)]
	pub type BalanceOf<T> = <T as Config>::Balance;

	/// Type alias exists mainly since `crate::models::VaultInfo` has many generic
	/// parameters.
	pub type VaultInfo<T> =
		crate::models::VaultInfo<AccountIdOf<T>, BalanceOf<T>, AssetIdOf<T>, BlockNumberOf<T>>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The Balance type used by the pallet for bookkeeping. `Config::Convert` is used for
		/// conversions to `u128`, which are used in the computations.
		type Balance: Default
			+ Parameter
			+ Codec
			+ MaxEncodedLen
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ SaturatingSub
			+ AtLeast32BitUnsigned
			+ Zero;

		/// The pallet creates new LP tokens for every created vault. It uses `CurrencyFactory`, as
		/// `orml`'s currency traits do not provide an interface to obtain asset ids (to avoid id
		/// collisions).
		type CurrencyFactory: CurrencyFactory<AssetId = Self::AssetId, Balance = Self::Balance>;

		/// The `AssetId` used by the pallet. Corresponds the the Ids used by the Currency pallet.
		type AssetId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo;

		/// The asset used to pay for rent and other fees.
		type NativeCurrency: TransferNative<Self::AccountId, Balance = Self::Balance>
			+ MutateNative<Self::AccountId, Balance = Self::Balance>
			+ MutateHoldNative<Self::AccountId, Balance = Self::Balance>;

		/// Currency is used for the assets managed by the vaults.
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ MutateHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;

		/// Key type for the vaults. `VaultId` uniquely identifies a vault. The identifiers are
		type VaultId: AddAssign
			+ FullCodec
			+ MaxEncodedLen
			+ One
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo
			+ Into<u128>;

		type WeightInfo: WeightInfo;

		/// Converts the `Balance` type to `u128`, which internally is used in calculations.
		type Convert: Convert<Self::Balance, u128> + Convert<u128, Self::Balance>;

		/// The maximum number of strategies a newly created vault can have.
		type MaxStrategies: Get<usize>;

		/// The minimum amount needed to deposit in a vault and receive LP tokens.
		#[pallet::constant]
		type MinimumDeposit: Get<Self::Balance>;

		/// The minimum amount of LP tokens to withdraw funds from a vault.
		#[pallet::constant]
		type MinimumWithdrawal: Get<Self::Balance>;

		/// The minimum native asset needed to create a vault.
		#[pallet::constant]
		type CreationDeposit: Get<Self::Balance>;

		/// The deposit needed for a vault to never be cleaned up. Should be significantly higher
		/// than the rent.
		#[pallet::constant]
		type ExistentialDeposit: Get<Self::Balance>;

		/// The duration that a vault may remain tombstoned before it can be deleted.
		#[pallet::constant]
		type TombstoneDuration: Get<Self::BlockNumber>;

		/// The rent being charged per block for vaults which have not committed the
		/// `ExistentialDeposit`.
		#[pallet::constant]
		type RentPerBlock: Get<Self::Balance>;

		/// The id used as the `AccountId` of the vault. This should be unique across all pallets to
		/// avoid name collisions with other pallets and vaults.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// The number of vaults, also used to generate the next vault identifier.
	///
	/// # Note
	///
	/// Cleaned up vaults do not decrement the counter.
	#[pallet::storage]
	#[pallet::getter(fn vault_count)]
	#[allow(clippy::disallowed_types)]
	// TODO: This is a nonce, rename to VaultId
	pub type VaultCount<T: Config> = StorageValue<_, T::VaultId, ValueQuery>;

	/// Info for each specific vaults.
	#[pallet::storage]
	#[pallet::getter(fn vault_data)]
	pub type Vaults<T: Config> = StorageMap<_, Twox64Concat, T::VaultId, VaultInfo<T>, OptionQuery>;

	/// Associated LP token for each vault.
	#[pallet::storage]
	#[pallet::getter(fn lp_tokens_to_vaults)]
	pub type LpTokensToVaults<T: Config> =
		StorageMap<_, Twox64Concat, T::AssetId, T::VaultId, OptionQuery>;

	/// Overview of the allocation & balances at each strategy. Does not contain the balance held by
	/// the vault itself.
	#[pallet::storage]
	#[pallet::getter(fn capital_structure)]
	// Bit questionable to have this be ValueQuery, as technically that makes it difficult to
	// determine if a strategy is connected to a vault vs not having an allocation at all.
	#[allow(clippy::disallowed_types)]
	pub type CapitalStructure<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::VaultId,
		Blake2_128Concat,
		T::AccountId,
		StrategyOverview<T::Balance>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emitted after a vault has been successfully created.
		VaultCreated {
			/// The (incremented) ID of the created vault.
			id: T::VaultId,
		},
		/// Emitted after a user deposits funds into the vault.
		Deposited {
			/// The account making the vault deposit.
			account: T::AccountId,
			/// The amount of assets deposited in the vault.
			asset_amount: T::Balance,
			/// The number of LP tokens minted for the deposit.
			lp_amount: T::Balance,
		},
		LiquidateStrategy {
			account: T::AccountId,
			amount: T::Balance,
		},
		/// Emitted after a user exchanges LP tokens back for underlying assets
		Withdrawn {
			/// The account ID making the withdrawal.
			account: T::AccountId,
			/// Amount of LP tokens exchanged for the withdrawal.
			lp_amount: T::Balance,
			/// Assets received in exchange for the withdrawal.
			asset_amount: T::Balance,
		},
		/// Emitted after a successful emergency shutdown.
		EmergencyShutdown {
			/// The ID of the vault.
			vault: T::VaultId,
		},
		/// Emitted after a vault is restarted.
		VaultStarted {
			/// The ID of the vault.
			vault: T::VaultId,
		},
	}

	#[allow(missing_docs)]
	#[pallet::error]
	pub enum Error<T> {
		/// It is not possible to perform a privileged action using an ordinary account
		AccountIsNotManager,
		/// Failures in creating LP tokens during vault creation result in `CannotCreateAsset`.
		CannotCreateAsset,
		/// Failures to transfer funds from the vault to users or vice- versa result in
		/// `TransferFromFailed`.
		TransferFromFailed,
		/// Minting failures result in `MintFailed`. In general this should never occur.
		MintFailed,
		/// Requesting withdrawals for more LP tokens than available to the user result in
		/// `InsufficientLpTokens`
		InsufficientLpTokens,
		/// Querying/operating on invalid vault id's result in `VaultDoesNotExist`.
		VaultDoesNotExist,
		/// If the vault contains too many assets (close to the `Balance::MAX`), it is considered
		/// full as arithmetic starts overflowing.
		NoFreeVaultAllocation,
		/// Vaults must allocate the proper ratio between reserved and strategies, so that the
		/// ratio sums up to one.
		AllocationMustSumToOne,
		/// Vaults may have up to [`MaxStrategies`](Config::MaxStrategies) strategies.
		TooManyStrategies,
		/// Vaults may have insufficient funds for withdrawals, as well as users wishing to deposit
		/// an incorrect amount.
		InsufficientFunds,
		/// Deposit amounts not exceeding [`MinimumDeposit`](Config::MinimumDeposit) are declined
		/// and result in `AmountMustGteMinimumDeposit`.
		AmountMustGteMinimumDeposit,
		/// Withdrawal amounts not exceeding [`MinimumWithdrawal`](Config::MinimumWithdrawal) are
		/// declined and result in `AmountMustGteMinimumWithdrawal`.
		AmountMustGteMinimumWithdrawal,
		/// When trying to withdraw too much from the vault, `NotEnoughLiquidity` is returned.
		// TODO: perhaps it is better to not fail on withdrawals, but instead return what we can?
		NotEnoughLiquidity,
		/// Creating vaults with invalid creation deposits results in
		/// `InsufficientCreationDeposit`.
		InsufficientCreationDeposit,
		/// Attempting to tombstone a vault which has rent remaining results in
		/// `InvalidSurchargeClaim`.
		InvalidSurchargeClaim,
		/// Not all vaults have an associated LP token. Attempting to perform LP token related
		/// operations result in `NotVaultLpToken`.
		NotVaultLpToken,
		/// The vault has deposits halted, see [Capabilities](crate::capabilities::Capabilities).
		DepositsHalted,
		/// The vault has withdrawals halted, see
		/// [Capabilities](crate::capabilities::Capabilities).
		WithdrawalsHalted,
		OnlyManagerCanDoThisOperation,
		InvalidDeletionClaim,
		/// The vault could not be deleted, as it was not yet tombstoned.
		VaultNotTombstoned,
		/// The vault could not be deleted, as it was not tombstoned for long enough.
		TombstoneDurationNotExceeded,
		/// Existentially funded vaults do not require extra funds.
		InvalidAddSurcharge,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates a new vault, locking up the deposit. If the deposit is greater than the
		/// `ExistentialDeposit` + `CreationDeposit`, the vault will remain alive forever, else it
		/// can be `tombstoned` after `deposit / RentPerBlock `. Accounts may deposit more funds to
		/// keep the vault alive.
		///
		/// # Emits
		///  - [`Event::VaultCreated`](Event::VaultCreated)
		///
		/// # Errors
		///  - When the origin is not signed.
		///  - When `deposit < CreationDeposit`.
		///  - Origin has insufficient funds to lock the deposit.
		#[transactional]
		#[pallet::weight(<T as Config>::WeightInfo::create())]
		pub fn create(
			origin: OriginFor<T>,
			vault: VaultConfig<AccountIdOf<T>, AssetIdOf<T>>,
			deposit_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;

			let (deposit_amount, deletion_reward) = {
				let deletion_reward = T::CreationDeposit::get();
				(
					deposit_amount
						.checked_sub(&deletion_reward)
						.ok_or(Error::<T>::InsufficientCreationDeposit)?,
					deletion_reward,
				)
			};

			// TODO(kaiserkarel): determine if we return the amount to the creator/manager after
			// deletion of the vault, or immediately to the treasury. (leaning towards the
			// second).
			let deposit = rent::deposit_from_balance::<T>(deposit_amount);

			let id = <Self as Vault>::create(deposit, vault)?;
			T::NativeCurrency::transfer(
				&from,
				&Self::deletion_reward_account(id),
				deletion_reward,
				true,
			)?;

			T::NativeCurrency::transfer(&from, &Self::rent_account(id), deposit_amount, true)?;
			Self::deposit_event(Event::VaultCreated { id });
			Ok(().into())
		}

		/// Subtracts rent from a vault, rewarding the caller if successful with a small fee and
		/// possibly tombstoning the vault.
		///
		/// A tombstoned vault still allows for withdrawals but blocks deposits, and requests all
		/// strategies to return their funds.
		#[pallet::weight(<T as Config>::WeightInfo::claim_surcharge())]
		pub fn claim_surcharge(
			origin: OriginFor<T>,
			dest: T::VaultId,
			address: Option<AccountIdOf<T>>,
		) -> DispatchResultWithPostInfo {
			let origin = origin.into();

			let reward_address = match (origin, address) {
				(Ok(frame_system::RawOrigin::Signed(account)), None) => account,
				(Ok(frame_system::RawOrigin::None), Some(address)) => address,
				_ => return Err(Error::<T>::InvalidSurchargeClaim.into()),
			};

			Vaults::<T>::try_mutate_exists(dest, |vault| -> DispatchResultWithPostInfo {
				let mut vault = vault.as_mut().ok_or(Error::<T>::VaultDoesNotExist)?;
				let current_block = <frame_system::Pallet<T>>::block_number();

				match rent::evaluate_eviction::<T>(current_block, vault.deposit) {
					Verdict::Exempt => Ok(().into()),
					Verdict::Evict => {
						vault.deposit = Deposit::Rent { amount: Zero::zero(), at: current_block };
						vault.capabilities.set_tombstoned();
						let account = &Self::rent_account(dest);
						// Clean up anything that remains in the vault's account. The reward for
						// cleaning up the tombstoned vault is in `deletion_reward_account`.
						let reward = T::NativeCurrency::reducible_balance(account, false);
						T::NativeCurrency::transfer(account, &reward_address, reward, false)?;
						Ok(().into())
					},
					Verdict::Charge { remaining, payable } => {
						vault.deposit = Deposit::Rent { amount: remaining, at: current_block };
						// If this transfer call fails due to the vaults account not being kept
						// alive, the caller should come back later, and evict the vault, which
						// empties the entire account. This ensures that the deposit accurately
						// reflects the account balance of the vault.
						T::NativeCurrency::transfer(
							&Self::rent_account(dest),
							&reward_address,
							payable,
							true,
						)?;
						Ok(().into())
					},
				}
			})
		}

		#[pallet::weight(<T as Config>::WeightInfo::add_surcharge())]
		pub fn add_surcharge(
			origin: OriginFor<T>,
			dest: T::VaultId,
			amount: Validated<BalanceOf<T>, ValidateCreationDeposit<T>>,
		) -> DispatchResultWithPostInfo {
			let origin = ensure_signed(origin)?;

			let amount = amount.value();

			Vaults::<T>::try_mutate_exists(dest, |vault| -> DispatchResultWithPostInfo {
				let mut vault = vault.as_mut().ok_or(Error::<T>::VaultDoesNotExist)?;
				let current = match vault.deposit {
					Deposit::Existential => return Err(Error::<T>::InvalidAddSurcharge.into()),
					Deposit::Rent { amount, .. } => amount,
				};
				T::NativeCurrency::transfer(&origin, &Self::rent_account(dest), amount, false)?;
				vault.deposit = rent::deposit_from_balance::<T>(amount + current);
				// since we guaranteed above that we're adding at least CreationDeposit, we can
				// now untombstone it. If it was not tombstoned, this is a noop.
				vault.capabilities.untombstone();
				Ok(().into())
			})
		}

		#[pallet::weight(<T as Config>::WeightInfo::delete_tombstoned())]
		pub fn delete_tombstoned(
			origin: OriginFor<T>,
			dest: T::VaultId,
			address: Option<AccountIdOf<T>>,
		) -> DispatchResultWithPostInfo {
			let reward_address = match (origin.into(), address) {
				(Ok(frame_system::RawOrigin::Signed(account)), None) => account,
				(Ok(frame_system::RawOrigin::None), Some(address)) => address,
				_ => return Err(Error::<T>::InvalidSurchargeClaim.into()),
			};

			Vaults::<T>::try_mutate_exists(dest, |v| -> DispatchResultWithPostInfo {
				let vault = v.as_mut().ok_or(Error::<T>::VaultDoesNotExist)?;
				ensure!(vault.capabilities.is_tombstoned(), Error::<T>::VaultNotTombstoned);

				if !rent::evaluate_deletion::<T>(
					<frame_system::Pallet<T>>::block_number(),
					vault.deposit,
				) {
					return Err(Error::<T>::TombstoneDurationNotExceeded.into())
				} else {
					let deletion_reward_account = &Self::deletion_reward_account(dest);
					let reward =
						T::NativeCurrency::reducible_balance(deletion_reward_account, false);
					// No need to keep `deletion_reward_account` alive. After this operation, the
					// vault has no associated data anymore.
					T::NativeCurrency::transfer(
						deletion_reward_account,
						&reward_address,
						reward,
						false,
					)?;
					LpTokensToVaults::<T>::remove(vault.asset_id);
					v.take();
				}
				Ok(().into())
			})
		}

		/// Deposit funds in the vault and receive LP tokens in return.
		/// # Emits
		///  - Event::Deposited
		///
		/// # Errors
		///  - When the origin is not signed.
		///  - When `deposit < MinimumDeposit`.
		#[pallet::weight(<T as Config>::WeightInfo::deposit())]
		pub fn deposit(
			origin: OriginFor<T>,
			vault: T::VaultId,
			asset_amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			let lp_amount = <Self as Vault>::deposit(&vault, &from, asset_amount)?;
			Self::deposit_event(Event::Deposited { account: from, asset_amount, lp_amount });
			Ok(().into())
		}

		/// Withdraw funds
		///
		/// # Emits
		///  - Event::Withdrawn
		///
		/// # Errors
		///  - When the origin is not signed.
		///  - When `lp_amount < MinimumWithdrawal`.
		///  - When the vault has insufficient amounts reserved.
		#[pallet::weight(<T as Config>::WeightInfo::withdraw())]
		pub fn withdraw(
			origin: OriginFor<T>,
			vault: T::VaultId,
			lp_amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let to = ensure_signed(origin)?;
			let asset_amount = <Self as Vault>::withdraw(&vault, &to, lp_amount)?;
			Self::deposit_event(Event::Withdrawn { account: to, lp_amount, asset_amount });
			Ok(().into())
		}

		/// Stops a vault. To be used in case of severe protocol flaws.
		///
		/// # Emits
		///  - Event::EmergencyShutdown
		///
		/// # Errors
		///  - When the origin is not root.
		///  - When `vault` does not exist.
		#[pallet::weight(<T as Config>::WeightInfo::emergency_shutdown())]
		pub fn emergency_shutdown(
			origin: OriginFor<T>,
			vault: T::VaultId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			<Self as CapabilityVault>::stop(&vault)?;
			Self::deposit_event(Event::EmergencyShutdown { vault });
			Ok(().into())
		}

		/// (Re)starts a vault after emergency shutdown.
		///
		/// # Emits
		///  - Event::VaultStarted
		///
		/// # Errors
		///  - When the origin is not root.
		///  - When `vault` does not exist.
		#[pallet::weight(<T as Config>::WeightInfo::start_())]
		pub fn start(origin: OriginFor<T>, vault: T::VaultId) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			<Self as CapabilityVault>::start(&vault)?;
			Self::deposit_event(Event::VaultStarted { vault });
			Ok(().into())
		}

		/// Turns an existent strategy account `strategy_account` of a vault determined by
		/// `vault_idx` into a liquidation state where withdrawn funds should be returned as soon
		/// as possible.
		///
		/// Only the vault's manager will be able to call this method.
		///
		/// # Emits
		///  - Event::LiquidateStrategy
		#[pallet::weight(10_000)]
		pub fn liquidate_strategy(
			origin: OriginFor<T>,
			vault_idx: T::VaultId,
			strategy_account_id: T::AccountId,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			ensure!(Self::vault_info(&vault_idx)?.manager == from, Error::<T>::AccountIsNotManager);
			let balance = CapitalStructure::<T>::try_get(vault_idx, &strategy_account_id)
				.map_err(|_err| DispatchError::CannotLookup)?
				.balance;
			CapitalStructure::<T>::remove(vault_idx, &strategy_account_id);
			Self::deposit_event(Event::LiquidateStrategy {
				account: strategy_account_id,
				amount: balance,
			});
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn do_create_vault(
			deposit: Deposit<BalanceOf<T>, BlockNumberOf<T>>,
			config: Validated<VaultConfig<T::AccountId, T::AssetId>, ValidateMaxStrategies<T>>,
		) -> Result<(T::VaultId, VaultInfo<T>), DispatchError> {
			// 1. check config
			// 2. lock endowment
			// 3. mint LP token
			// 4. insert vault (do we check if the strategy addresses even exists?)
			VaultCount::<T>::try_mutate(|id| {
				let id = {
					*id += One::one();
					*id
				};

				// Perhaps later on, we'll make this configurable per creator account id, if we want
				// to allow special projects to create more complex vaults.
				let config = config.value();

				// We do allow vaults without strategies, since strategies can be decided on later
				// through governance. If strategies are present, their allocations must sum up to
				// 1.
				let sum = config
					.strategies
					.iter()
					.fold(Some(config.reserved.deconstruct()), |sum, (_, allocation)| {
						sum.and_then(|sum| sum.checked_add(allocation.deconstruct()))
					})
					.ok_or(Error::<T>::AllocationMustSumToOne)?;

				ensure!(
					sum == Perquintill::one().deconstruct(),
					Error::<T>::AllocationMustSumToOne
				);

				let lp_token_id = {
					T::CurrencyFactory::create(RangeId::LP_TOKENS)
						.map_err(|_| Error::<T>::CannotCreateAsset)?
				};

				config.strategies.into_iter().for_each(|(account_id, allocation)| {
					CapitalStructure::<T>::insert(
						id,
						account_id,
						StrategyOverview {
							allocation,
							balance: T::Balance::zero(),
							lifetime_withdrawn: T::Balance::zero(),
							lifetime_deposited: T::Balance::zero(),
						},
					);
				});

				let vault_info = crate::models::VaultInfo {
					lp_token_id,
					manager: config.manager,
					asset_id: config.asset_id,
					deposit,
					capabilities: Default::default(),
				};

				Vaults::<T>::insert(id, vault_info.clone());
				LpTokensToVaults::<T>::insert(lp_token_id, id);

				Ok((id, vault_info))
			})
		}

		fn rent_account(vault_id: T::VaultId) -> T::AccountId {
			let vault_id: u128 = vault_id.into();
			T::PalletId::get()
				.into_sub_account_truncating([b"rent_account____", &vault_id.to_le_bytes()])
		}

		fn deletion_reward_account(vault_id: T::VaultId) -> T::AccountId {
			let vault_id: u128 = vault_id.into();
			T::PalletId::get()
				.into_sub_account_truncating([b"deletion_account", &vault_id.to_le_bytes()])
		}

		/// Computes the sum of all the assets that the vault currently controls.
		fn assets_under_management(vault_id: &T::VaultId) -> Result<T::Balance, Error<T>> {
			let vault =
				Vaults::<T>::try_get(vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;
			Self::do_assets_under_management(vault_id, &vault)
		}

		fn do_withdraw(
			vault_id: &T::VaultId,
			to: &T::AccountId,
			lp_amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let vault = Self::vault_info(vault_id)?;

			ensure!(vault.capabilities.withdrawals_allowed(), Error::<T>::WithdrawalsHalted);

			let lp_shares_value_amount = Self::do_lp_share_value(vault_id, &vault, lp_amount)?;

			let vault_owned_amount =
				T::Currency::balance(vault.asset_id, &Self::account_id(vault_id));

			// TODO(hussein-aitlahcen): should we provide what we can to reduce the available
			// liquidity in order to force strategies to rebalance?
			ensure!(lp_shares_value_amount <= vault_owned_amount, Error::<T>::NotEnoughLiquidity);

			ensure!(
				T::Currency::can_withdraw(vault.lp_token_id, to, lp_amount)
					.into_result()
					.is_ok(),
				Error::<T>::InsufficientLpTokens
			);

			let from = Self::account_id(vault_id);
			ensure!(
				T::Currency::can_withdraw(vault.asset_id, &from, lp_shares_value_amount)
					.into_result()
					.is_ok(),
				Error::<T>::TransferFromFailed
			);

			T::Currency::burn_from(vault.lp_token_id, to, lp_amount)
				.map_err(|_| Error::<T>::InsufficientLpTokens)?;
			T::Currency::transfer(vault.asset_id, &from, to, lp_shares_value_amount, true)
				.map_err(|_| Error::<T>::TransferFromFailed)?;
			Ok(lp_shares_value_amount)
		}

		fn do_deposit(
			vault_id: &T::VaultId,
			from: &T::AccountId,
			amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let vault = Self::vault_info(vault_id)?;

			ensure!(vault.capabilities.deposits_allowed(), Error::<T>::DepositsHalted);
			ensure!(
				T::Currency::can_withdraw(vault.asset_id, from, amount).into_result().is_ok(),
				Error::<T>::TransferFromFailed
			);

			let to = Self::account_id(vault_id);

			let lp = Self::do_calculate_lp_tokens_to_mint(vault_id, &vault, amount)?;

			T::Currency::transfer(vault.asset_id, from, &to, amount, true)
				.map_err(|_| Error::<T>::TransferFromFailed)?;
			T::Currency::mint_into(vault.lp_token_id, from, lp)
				.map_err(|_| Error::<T>::MintFailed)?;
			Ok(lp)
		}

		fn do_calculate_lp_tokens_to_mint(
			vault_id: &T::VaultId,
			vault: &VaultInfo<T>,
			amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let vault_aum = Self::assets_under_management(vault_id)?;
			if vault_aum.is_zero() {
				// No assets in the vault means we should have no outstanding LP tokens, we can thus
				// freely mint new tokens without performing the calculation.
				Ok(amount)
			} else {
				// Compute how much of the underlying assets are deposited. LP tokens are allocated
				// such that, if the deposit is N% of the `aum`, N% of the LP token supply are
				// minted to the depositor.
				//
				// TODO(kaiserkarel): Get this reviewed, integer arithmetic is hard after all.
				// reference:
				// https://medium.com/coinmonks/programming-defi-uniswap-part-2-13a6428bf892
				let outstanding = T::Currency::total_issuance(vault.lp_token_id);
				let lp = Self::convert_and_multiply_by_rational(amount, outstanding, vault_aum)
					.map_err(|_| Error::<T>::NoFreeVaultAllocation)?;

				ensure!(lp > T::Balance::zero(), Error::<T>::InsufficientCreationDeposit);
				Ok(lp)
			}
		}

		fn do_lp_share_value(
			vault_id: &T::VaultId,
			vault: &VaultInfo<T>,
			lp_amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			/*
			   a = total lp issued
			   b = asset under management
			   lp_share_percent = lp / a
			   lp_shares_value	= lp_share_percent * b
								= lp / a * b
								= lp * b / a
			*/

			let vault_aum = Self::do_assets_under_management(vault_id, vault)?;
			let lp_total_issuance = T::Currency::total_issuance(vault.lp_token_id);

			let shares_amount =
				Self::convert_and_multiply_by_rational(lp_amount, vault_aum, lp_total_issuance)?;

			Ok(shares_amount)
		}

		fn do_amount_of_lp_token_for_added_liquidity(
			vault_id: &T::VaultId,
			vault: &VaultInfo<T>,
			asset_amount: T::Balance,
		) -> Result<BalanceOf<T>, DispatchError> {
			let total_lp_issuance = T::Currency::total_issuance(vault.lp_token_id);
			let aum = Self::assets_under_management(vault_id)?;

			let shares_amount =
				Self::convert_and_multiply_by_rational(asset_amount, total_lp_issuance, aum)?;

			Ok(shares_amount)
		}

		fn convert_and_multiply_by_rational(
			a: T::Balance,
			b: T::Balance,
			c: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let a = <T::Convert as Convert<T::Balance, u128>>::convert(a);
			let b = <T::Convert as Convert<T::Balance, u128>>::convert(b);
			let c = <T::Convert as Convert<T::Balance, u128>>::convert(c);

			let res = multiply_by_rational_with_rounding(a, b, c, Rounding::Down)
				.ok_or(ArithmeticError::Overflow)?;

			let res = <T::Convert as Convert<u128, T::Balance>>::convert(res);
			Ok(res)
		}

		/// Computes the sum of all the assets that the vault currently controls.
		fn do_assets_under_management(
			vault_id: &T::VaultId,
			vault: &VaultInfo<T>,
		) -> Result<T::Balance, Error<T>> {
			let owned = T::Currency::balance(vault.asset_id, &Self::account_id(vault_id));
			let outstanding = CapitalStructure::<T>::iter_prefix_values(vault_id)
				.fold(T::Balance::zero(), |sum, item| sum + item.balance);
			Ok(owned + outstanding)
		}

		/// Tries to fetch a stored [VaultInfo] through its index.
		fn vault_info(vault_idx: &T::VaultId) -> Result<VaultInfo<T>, DispatchError> {
			Ok(Vaults::<T>::try_get(vault_idx).map_err(|_err| Error::<T>::VaultDoesNotExist)?)
		}
	}

	impl<T: Config> Vault for Pallet<T> {
		type AccountId = T::AccountId;
		type Balance = T::Balance;
		type BlockNumber = T::BlockNumber;
		type VaultId = T::VaultId;
		type AssetId = AssetIdOf<T>;

		fn asset_id(vault_id: &Self::VaultId) -> Result<Self::AssetId, DispatchError> {
			Ok(Self::vault_info(vault_id)?.asset_id)
		}

		fn lp_asset_id(vault_id: &Self::VaultId) -> Result<Self::AssetId, DispatchError> {
			Ok(Self::vault_info(vault_id)?.lp_token_id)
		}

		fn account_id(vault: &Self::VaultId) -> Self::AccountId {
			T::PalletId::get().into_sub_account_truncating(vault)
		}

		fn create(
			deposit: Deposit<Self::Balance, Self::BlockNumber>,
			config: VaultConfig<Self::AccountId, Self::AssetId>,
		) -> Result<Self::VaultId, DispatchError> {
			match Validated::new(config) {
				Ok(validated_config) =>
					Self::do_create_vault(deposit, validated_config).map(|(id, _)| id),
				Err(_) => Err(DispatchError::from(Error::<T>::TooManyStrategies)),
			}
		}

		fn deposit(
			vault_id: &Self::VaultId,
			from: &Self::AccountId,
			asset_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			ensure!(
				asset_amount > T::MinimumDeposit::get(),
				Error::<T>::AmountMustGteMinimumDeposit
			);
			Pallet::<T>::do_deposit(vault_id, from, asset_amount)
		}

		fn withdraw(
			vault: &Self::VaultId,
			to: &Self::AccountId,
			lp_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			ensure!(
				lp_amount > T::MinimumWithdrawal::get(),
				Error::<T>::AmountMustGteMinimumWithdrawal
			);
			Pallet::<T>::do_withdraw(vault, to, lp_amount)
		}

		fn stock_dilution_rate(vault_id: &Self::VaultId) -> Result<Rate, DispatchError> {
			let vault = Self::vault_info(vault_id)?;
			let lp_total_issuance = T::Currency::total_issuance(vault.lp_token_id);
			let lp_total_issuance_value =
				<T::Convert as Convert<T::Balance, u128>>::convert(lp_total_issuance);
			// If we don't have issued any LP token, the rate is 1:1
			if lp_total_issuance_value == 0 {
				Ok(Rate::from(1))
			} else {
				// Otherwise, we basically return the base/issued rate
				let base_asset_amount = Self::do_assets_under_management(vault_id, &vault)?;
				let base_asset_amount_value =
					<T::Convert as Convert<T::Balance, u128>>::convert(base_asset_amount);
				let rate =
					Rate::checked_from_rational(base_asset_amount_value, lp_total_issuance_value)
						.unwrap_or_else(Rate::zero);
				Ok(rate)
			}
		}

		fn token_vault(token: Self::AssetId) -> Result<Self::VaultId, DispatchError> {
			let vault_index =
				LpTokensToVaults::<T>::try_get(token).map_err(|_| Error::<T>::NotVaultLpToken)?;
			Ok(vault_index)
		}

		fn calculate_lp_tokens_to_mint(
			vault_id: &Self::VaultId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let vault = Self::vault_info(vault_id)?;
			let lp = Self::do_calculate_lp_tokens_to_mint(vault_id, &vault, amount)?;
			Ok(lp)
		}

		fn lp_share_value(
			vault_id: &Self::VaultId,
			lp_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let vault = Self::vault_info(vault_id)?;
			let amount = Self::do_lp_share_value(vault_id, &vault, lp_amount)?;
			Ok(amount)
		}

		fn amount_of_lp_token_for_added_liquidity(
			vault_id: &Self::VaultId,
			asset_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let vault = Self::vault_info(vault_id)?;
			let lp =
				Self::do_amount_of_lp_token_for_added_liquidity(vault_id, &vault, asset_amount)?;
			Ok(lp)
		}
	}

	impl<T: Config> StrategicVault for Pallet<T> {
		fn available_funds(
			vault_id: &Self::VaultId,
			account: &Self::AccountId,
		) -> Result<FundsAvailability<Self::Balance>, DispatchError> {
			match (
				Vaults::<T>::try_get(vault_id),
				CapitalStructure::<T>::try_get(vault_id, account),
			) {
				(Ok(vault), Ok(StrategyOverview { allocation, balance, .. }))
					if !vault.capabilities.is_stopped() && !vault.capabilities.is_tombstoned() =>
				{
					let aum = Self::assets_under_management(vault_id)?;
					let max_allowed = <T::Convert as Convert<u128, T::Balance>>::convert(
						allocation
							.mul_floor(<T::Convert as Convert<T::Balance, u128>>::convert(aum)),
					);
					match balance.cmp(&max_allowed) {
						Ordering::Greater =>
							Ok(FundsAvailability::Depositable(balance - max_allowed)),
						Ordering::Less =>
							Ok(FundsAvailability::Withdrawable(max_allowed - balance)),
						Ordering::Equal => Ok(FundsAvailability::None),
					}
				},
				(_, _) => Ok(FundsAvailability::MustLiquidate),
			}
		}

		fn withdraw(
			vault_id: &Self::VaultId,
			to: &Self::AccountId,
			amount: Self::Balance,
		) -> Result<(), DispatchError> {
			// TODO: should we check the allocation here? Pallets are technically trusted, so it
			// would only add unnecessary overhead. The extrinsic/ChainExtension interface should
			// check however
			let vault = Self::vault_info(vault_id)?;
			CapitalStructure::<T>::try_mutate(vault_id, to, |state| {
				// I do not thing balance can actually overflow, since the total_issuance <=
				// T::Balance::Max
				state.balance =
					state.balance.checked_add(&amount).ok_or(ArithmeticError::Overflow)?;
				// This can definitely overflow. Perhaps it should be a BigUint?
				state.lifetime_withdrawn = state
					.lifetime_withdrawn
					.checked_add(&amount)
					.ok_or(ArithmeticError::Overflow)?;
				T::Currency::transfer(
					vault.asset_id,
					&Self::account_id(vault_id),
					to,
					amount,
					true,
				)
				.map_err(|_| Error::<T>::InsufficientFunds)?;
				Ok(())
			})
		}

		fn deposit(
			vault_id: &Self::VaultId,
			from: &Self::AccountId,
			amount: Self::Balance,
		) -> Result<(), DispatchError> {
			let vault = Self::vault_info(vault_id)?;
			CapitalStructure::<T>::try_mutate(vault_id, from, |state| {
				// A strategy can return more than it has withdrawn through profits.
				state.balance = state.balance.saturating_sub(&amount);
				// This can definitely overflow. Perhaps it should be a BigUint?
				state.lifetime_deposited = state
					.lifetime_deposited
					.checked_add(&amount)
					.ok_or(ArithmeticError::Overflow)?;
				T::Currency::transfer(
					vault.asset_id,
					from,
					&Self::account_id(vault_id),
					amount,
					true,
				)
				.map_err(|_| Error::<T>::InsufficientFunds)?;
				Ok(())
			})
		}
	}

	impl<T: Config> ReportableStrategicVault for Pallet<T> {
		type Report = T::Balance;

		fn update_strategy_report(
			vault: &Self::VaultId,
			strategy: &Self::AccountId,
			report: &Self::Report,
		) -> Result<(), DispatchError> {
			CapitalStructure::<T>::mutate(vault, strategy, |state| state.balance = *report);
			Ok(())
		}
	}

	impl<T: Config> CapabilityVault for Pallet<T> {
		fn stop(vault_id: &Self::VaultId) -> DispatchResult {
			Vaults::<T>::try_mutate_exists(vault_id, |vault| {
				if let Some(vault) = vault {
					ensure!(
						!vault.capabilities.is_tombstoned(),
						DispatchError::Other("cannot stop a tombstoned vault")
					);
					vault.capabilities.set_stopped();
					Ok(())
				} else {
					Err(DispatchError::CannotLookup)
				}
			})
		}

		fn is_stopped(vault_id: &Self::VaultId) -> Result<bool, DispatchError> {
			Self::vault_info(vault_id).map(|vault| vault.capabilities.is_stopped())
		}

		fn start(vault_id: &Self::VaultId) -> DispatchResult {
			Vaults::<T>::try_mutate_exists(vault_id, |vault| {
				if let Some(vault) = vault {
					ensure!(
						!vault.capabilities.is_tombstoned(),
						DispatchError::Other("cannot start a tombstoned vault")
					);
					vault.capabilities.start();
					Ok(())
				} else {
					Err(DispatchError::CannotLookup)
				}
			})
		}

		fn tombstone(vault_id: &Self::VaultId) -> DispatchResult {
			Vaults::<T>::try_mutate_exists(vault_id, |vault| {
				if let Some(vault) = vault {
					ensure!(
						!vault.capabilities.is_tombstoned(),
						DispatchError::Other("cannot tombstone a tombstoned vault")
					);
					vault.capabilities.set_tombstoned();
					Ok(())
				} else {
					Err(DispatchError::CannotLookup)
				}
			})
		}

		fn untombstone(vault_id: &Self::VaultId) -> DispatchResult {
			Vaults::<T>::try_mutate_exists(vault_id, |vault| {
				if let Some(vault) = vault {
					ensure!(
						vault.capabilities.is_tombstoned(),
						DispatchError::Other("cannot untombstone a non-tombstoned vault")
					);
					vault.capabilities.untombstone();
					Ok(())
				} else {
					Err(DispatchError::CannotLookup)
				}
			})
		}

		fn is_tombstoned(vault_id: &Self::VaultId) -> Result<bool, DispatchError> {
			Self::vault_info(vault_id).map(|vault| vault.capabilities.is_tombstoned())
		}

		fn stop_withdrawals(vault_id: &Self::VaultId) -> DispatchResult {
			Vaults::<T>::try_mutate_exists(vault_id, |vault| {
				if let Some(vault) = vault {
					vault.capabilities.stop_withdrawals();
					Ok(())
				} else {
					Err(DispatchError::CannotLookup)
				}
			})
		}

		fn allow_withdrawals(vault_id: &Self::VaultId) -> DispatchResult {
			Vaults::<T>::try_mutate_exists(vault_id, |vault| {
				if let Some(vault) = vault {
					vault.capabilities.allow_withdrawals();
					Ok(())
				} else {
					Err(DispatchError::CannotLookup)
				}
			})
		}

		fn withdrawals_allowed(vault_id: &Self::VaultId) -> Result<bool, DispatchError> {
			Self::vault_info(vault_id).map(|vault| vault.capabilities.withdrawals_allowed())
		}

		fn stop_deposits(vault_id: &Self::VaultId) -> DispatchResult {
			Vaults::<T>::try_mutate_exists(vault_id, |vault| {
				if let Some(vault) = vault {
					vault.capabilities.stop_deposits();
					Ok(())
				} else {
					Err(DispatchError::CannotLookup)
				}
			})
		}

		fn allow_deposits(vault_id: &Self::VaultId) -> DispatchResult {
			Vaults::<T>::try_mutate_exists(vault_id, |vault| {
				if let Some(vault) = vault {
					ensure!(
						!vault.capabilities.is_tombstoned(),
						DispatchError::Other("cannot allow deposits of a tombstoned vault")
					);
					vault.capabilities.allow_deposits();
					Ok(())
				} else {
					Err(DispatchError::CannotLookup)
				}
			})
		}

		fn deposits_allowed(vault_id: &Self::VaultId) -> Result<bool, DispatchError> {
			Self::vault_info(vault_id).map(|vault| vault.capabilities.deposits_allowed())
		}
	}
}
