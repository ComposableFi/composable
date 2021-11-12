//! # Vaults Pallet
//!
//! A batteries included vault module, usable as liquidity pools, yield farming vaults or embeddable
//! as core infrastructure.
//!
//! ## Overview
//!
//! The Vault module provides functionality for asset pool management of fungible asset classes
//! with a fixed supply, including:
//!
//! * Vault Creation.
//! * Deposits and Withdrawals.
//! * Strategy Re-balancing.
//! * Surcharge Claims and Rent.
//!
//! To use it in your runtime, you need to implement the vault's [`Config`].
//!
//! ## Terminology
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
	dead_code,
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
	unused_extern_crates,
	rustdoc::missing_doc_code_examples
)]
// Some substrate macros are expanded in such a way that their items cannot be documented. For now,
// it's best to just set this to warn during development.
#![allow(missing_docs)]

mod capabilities;
pub mod models;
mod rent;
mod traits;

pub use capabilities::Capabilities;
pub use pallet::*;

pub mod mocks;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		models::StrategyOverview,
		rent::Verdict,
		traits::{CurrencyFactory, StrategicVault},
	};
	use codec::{Codec, FullCodec};
	use composable_traits::{
		rate_model::Rate,
		vault::{
			CapabilityVault, Deposit, FundsAvailability, ReportableStrategicVault, Vault,
			VaultConfig,
		},
	};
	use frame_support::{
		ensure,
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::{fungibles::MutateHold, DepositConsequence},
		},
		PalletId,
	};
	use frame_system::{
		ensure_root, ensure_signed, pallet_prelude::OriginFor, Config as SystemConfig,
	};
	use num_traits::SaturatingSub;
	use scale_info::TypeInfo;
	use sp_runtime::{
		helpers_128bit::multiply_by_rational,
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Convert,
			Zero,
		},
		DispatchError, FixedPointNumber, Perquintill,
	};
	use sp_std::fmt::Debug;

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
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Balance type used by the pallet for bookkeeping. `Config::Convert` is used for
		/// conversions to `u128`, which are used in the computations.
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
			+ Zero;

		/// The pallet creates new LP tokens for every created vault. It uses `CurrencyFactory`, as
		/// `orml`'s currency traits do not provide an interface to obtain asset ids (to avoid id
		/// collisions).
		type CurrencyFactory: CurrencyFactory<Self::AssetId>;

		/// The `AssetId` used by the pallet. Corresponds the the Ids used by the Currency pallet.
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo;

		/// Generic Currency bounds. These functions are provided by the `[orml-tokens`](https://github.com/open-web3-stack/open-runtime-module-library/tree/HEAD/currencies) pallet.
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ MutateHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;

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

		/// The asset ID used to pay for rent.
		#[pallet::constant]
		type NativeAssetId: Get<Self::AssetId>;

		/// The minimum native asset needed to create a vault.
		#[pallet::constant]
		type CreationDeposit: Get<Self::Balance>;

		/// The deposit needed for a vault to never be cleaned up. Should be significantly higher
		/// than the rent.
		#[pallet::constant]
		type ExistentialDeposit: Get<Self::Balance>;

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
	pub type VaultCount<T: Config> = StorageValue<_, VaultIndex, ValueQuery>;

	/// Info for each specific vaults.
	#[pallet::storage]
	#[pallet::getter(fn vault_data)]
	pub type Vaults<T: Config> = StorageMap<_, Twox64Concat, VaultIndex, VaultInfo<T>, ValueQuery>;

	/// Associated LP token for each vault.
	#[pallet::storage]
	#[pallet::getter(fn lp_tokens_to_vaults)]
	pub type LpTokensToVaults<T: Config> =
		StorageMap<_, Twox64Concat, T::AssetId, VaultIndex, ValueQuery>;

	/// Amounts which each strategy is allowed to access, including the amount reserved for quick
	/// withdrawals for the pallet.
	#[pallet::storage]
	#[pallet::getter(fn allocations)]
	pub type Allocations<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		VaultIndex,
		Blake2_128Concat,
		T::AccountId,
		Perquintill,
		ValueQuery,
	>;

	/// Overview of the balances at each strategy. Does not contain the balance held by the vault
	/// itself.
	#[pallet::storage]
	#[pallet::getter(fn capital_structure)]
	pub type CapitalStructure<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		VaultIndex,
		Blake2_128Concat,
		T::AccountId,
		StrategyOverview<T::Balance>,
		ValueQuery,
	>;

	/// Key type for the vaults. `VaultIndex` uniquely identifies a vault.
	// TODO: should probably be settable through the config
	pub type VaultIndex = u64;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emitted after a vault has been successfully created.
		VaultCreated {
			/// The (incremented) ID of the created vault.
			id: VaultIndex,
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
		/// Emitted after a user exchanges LP tokens back for underlying assets
		Withdrawn {
			/// The account ID making the withdrawal.
			account: T::AccountId,
			/// Amount of LP tokens exchanged for the withdrawal.
			lp_amount: T::Balance,
			/// Assets received in exchange for the withdrawal.
			asset_amount: T::Balance,
		},
		/// Emitted after a succesful emergency shutdown.
		EmergencyShutdown {
			/// The ID of the vault.
			vault: VaultIndex,
		},
		/// Emitted after a vault is restarted.
		VaultStarted {
			/// The ID of the vault.
			vault: VaultIndex,
		},
	}

	#[allow(missing_docs)]
	#[pallet::error]
	pub enum Error<T> {
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
		/// For very large numbers, arithmetic starts failing.
		// TODO: could we fall back to BigInts, and never have math failures?
		OverflowError,
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
		/// The vault has deposits halted, see [Capabilities](crate::capabilities::Capability).
		DepositsHalted,
		/// The vault has withdrawals halted, see [Capabilities](crate::capabilities::Capability).
		WithdrawalsHalted,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates a new vault, locking up the deposit. If the deposit is greater than the
		/// `ExistentialDeposit`, the vault will remain alive forever, else it can be `tombstoned`
		/// after `deposit / RentPerBlock `. Accounts may deposit more funds to keep the vault
		/// alive.
		///
		/// # Emits
		///  - [`Event::VaultCreated`](Event::VaultCreated)
		///
		/// # Errors
		///  - When the origin is not signed.
		///  - When `deposit < CreationDeposit`.
		///  - Origin has insufficient funds to lock the deposit.
		#[pallet::weight(10_000)]
		pub fn create(
			origin: OriginFor<T>,
			vault: VaultConfig<AccountIdOf<T>, AssetIdOf<T>>,
			deposit: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;

			ensure!(deposit >= T::CreationDeposit::get(), Error::<T>::InsufficientCreationDeposit);

			let native_id = T::NativeAssetId::get();
			T::Currency::hold(native_id, &from, deposit)?;

			let deposit = if deposit > T::ExistentialDeposit::get() {
				Deposit::Existential
			} else {
				Deposit::Rent { amount: deposit, at: <frame_system::Pallet<T>>::block_number() }
			};
			let id = <Self as Vault>::create(deposit, vault)?;
			Self::deposit_event(Event::VaultCreated { id });
			Ok(().into())
		}

		/// Tombstones a vault, rewarding the caller if successful with a small fee.
		///
		/// TODO:
		///  - Check that the vault has no more funds, else do something?
		///  - First disable the vault, then after X amount of time delete it
		#[pallet::weight(10_000)]
		pub fn claim_surcharge(
			origin: OriginFor<T>,
			dest: VaultIndex,
			address: Option<AccountIdOf<T>>,
		) -> DispatchResultWithPostInfo {
			let origin = origin.into();

			let (signed, _rewarded) = match (origin, address) {
				(Ok(frame_system::RawOrigin::Signed(account)), None) => (true, account),
				(Ok(frame_system::RawOrigin::None), Some(address)) => (false, address),
				_ => return Err(Error::<T>::InvalidSurchargeClaim.into()),
			};

			// for now, we'll only allow collators to claim surcharges. Once we implement
			// capabilities + tombstoning, we'll evaluate having users call this too.
			ensure!(!signed, Error::<T>::InvalidSurchargeClaim);

			let vault = Vaults::<T>::try_get(dest).map_err(|_| Error::<T>::VaultDoesNotExist)?;
			let current_block = <frame_system::Pallet<T>>::block_number();

			match crate::rent::evaluate_eviction::<T>(current_block, vault.deposit) {
				Verdict::Exempt => {
					todo!("do not reward, but charge less weight")
				},
				Verdict::Evict { .. } => {
					// we should also decide if we are going to drop the vault if there are still
					// assets left in strategies. If some strategy becomes bricked, they will never
					// report or return a balance. Tombstoned vaults would then effectively take up
					// storage forever.
					todo!("clean up all storage associated with the vault, and then reward the caller")
				},
				Verdict::Charge { .. } => {
					todo!("update vault deposit info, charge some of the rent from the `hold`ed balance")
				},
			}
		}

		/// Deposit funds in the vault and receive LP tokens in return.
		/// # Emits
		///  - Event::Deposited
		///
		/// # Errors
		///  - When the origin is not signed.
		///  - When `deposit < MinimumDeposit`.
		#[pallet::weight(10_000)]
		pub fn deposit(
			origin: OriginFor<T>,
			vault: VaultIndex,
			asset_amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			let lp_amount = <Self as Vault>::deposit(&vault, &from, asset_amount)?;
			Self::deposit_event(Event::Deposited { account: from, asset_amount, lp_amount });
			Ok(().into())
		}

		/// Deposit funds in the vault and receive LP tokens in return.
		/// # Emits
		///  - Event::Withdrawn
		///
		/// # Errors
		///  - When the origin is not signed.
		///  - When `lp_amount < MinimumWithdrawal`.
		///  - When the vault has insufficient amounts reserved.
		#[pallet::weight(10_000)]
		pub fn withdraw(
			origin: OriginFor<T>,
			vault: VaultIndex,
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
		#[pallet::weight(10_000)]
		pub fn emergency_shutdown(
			origin: OriginFor<T>,
			vault: VaultIndex,
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
		#[pallet::weight(10_000)]
		pub fn start(origin: OriginFor<T>, vault: VaultIndex) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			<Self as CapabilityVault>::start(&vault)?;
			Self::deposit_event(Event::VaultStarted { vault });
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn do_create_vault(
			deposit: Deposit<BalanceOf<T>, BlockNumberOf<T>>,
			config: VaultConfig<T::AccountId, T::AssetId>,
		) -> Result<(VaultIndex, VaultInfo<T>), DispatchError> {
			// 1. check config
			// 2. lock endowment
			// 3. mint LP token
			// 4. insert vault (do we check if the strategy addresses even exists?)
			VaultCount::<T>::try_mutate(|id| {
				let id = {
					*id += 1;
					*id
				};

				// Perhaps later on, we'll make this configurable per creator account id, if we want
				// to allow special projects to create more complex vaults.
				ensure!(
					config.strategies.len() <= T::MaxStrategies::get(),
					Error::<T>::TooManyStrategies
				);

				// We do allow vaults without strategies, since strategies can be decided on later
				// through governance. If strategies are present, their allocations must sum up to
				// 1.
				let sum = config
					.strategies
					.iter()
					.fold(Some(config.reserved.deconstruct()), |sum, (_, allocation)| {
						sum.map(|sum| sum.checked_add(allocation.deconstruct())).flatten()
					})
					.ok_or(Error::<T>::AllocationMustSumToOne)?;

				ensure!(
					sum == Perquintill::one().deconstruct(),
					Error::<T>::AllocationMustSumToOne
				);

				let lp_token_id =
					{ T::CurrencyFactory::create().map_err(|_| Error::<T>::CannotCreateAsset)? };

				config.strategies.into_iter().for_each(|(account_id, allocation)| {
					Allocations::<T>::insert(id, account_id.clone(), allocation);
					CapitalStructure::<T>::insert(
						id,
						account_id,
						StrategyOverview {
							balance: T::Balance::zero(),
							lifetime_withdrawn: T::Balance::zero(),
							lifetime_deposited: T::Balance::zero(),
						},
					);
				});

				Allocations::<T>::insert(id, Self::account_id(&id), config.reserved);

				let vault_info = crate::models::VaultInfo {
					lp_token_id,
					manager: config.manager,
					asset_id: config.asset_id,
					deposit,
					..Default::default()
				};

				Vaults::<T>::insert(id, vault_info.clone());
				LpTokensToVaults::<T>::insert(lp_token_id, id);

				Ok((id, vault_info))
			})
		}

		/// Computes the sum of all the assets that the vault currently controls.
		fn assets_under_management(vault_id: &VaultIndex) -> Result<T::Balance, Error<T>> {
			let vault =
				Vaults::<T>::try_get(vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;
			Self::do_assets_under_management(vault_id, &vault)
		}

		fn do_withdraw(
			vault_id: &VaultIndex,
			to: &T::AccountId,
			lp_amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let vault =
				Vaults::<T>::try_get(&vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;

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
			vault_id: &VaultIndex,
			from: &T::AccountId,
			amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let vault =
				Vaults::<T>::try_get(&vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;

			ensure!(vault.capabilities.deposits_allowed(), Error::<T>::DepositsHalted);

			ensure!(
				T::Currency::can_withdraw(vault.asset_id, from, amount).into_result().is_ok(),
				Error::<T>::TransferFromFailed
			);

			let to = Self::account_id(vault_id);

			let vault_aum = Self::assets_under_management(vault_id)?;
			if vault_aum.is_zero() {
				ensure!(
					T::Currency::can_deposit(vault.lp_token_id, from, amount) ==
						DepositConsequence::Success,
					Error::<T>::MintFailed
				);

				// No assets in the vault means we should have no outstanding LP tokens, we can thus
				// freely mint new tokens without performing the calculation.
				T::Currency::transfer(vault.asset_id, from, &to, amount, true)
					.map_err(|_| Error::<T>::TransferFromFailed)?;
				T::Currency::mint_into(vault.lp_token_id, from, amount)
					.map_err(|_| Error::<T>::MintFailed)?;
				Ok(amount)
			} else {
				// Compute how much of the underlying assets are deposited. LP tokens are allocated
				// such that, if the deposit is N% of the `aum`, N% of the LP token supply are
				// minted to the depositor.
				//
				// TODO(kaiserkarel): Get this reviewed, integer arithmetic is hard after all.
				// reference:
				// https://medium.com/coinmonks/programming-defi-uniswap-part-2-13a6428bf892
				let deposit = <T::Convert as Convert<T::Balance, u128>>::convert(amount);
				let outstanding = T::Currency::total_issuance(vault.lp_token_id);
				let outstanding = <T::Convert as Convert<T::Balance, u128>>::convert(outstanding);
				let aum = <T::Convert as Convert<T::Balance, u128>>::convert(vault_aum);
				let lp = multiply_by_rational(deposit, outstanding, aum)
					.map_err(|_| Error::<T>::NoFreeVaultAllocation)?;
				let lp = <T::Convert as Convert<u128, T::Balance>>::convert(lp);

				ensure!(lp > T::Balance::zero(), Error::<T>::InsufficientCreationDeposit);

				ensure!(
					T::Currency::can_deposit(vault.lp_token_id, from, lp) ==
						DepositConsequence::Success,
					Error::<T>::MintFailed
				);

				T::Currency::transfer(vault.asset_id, from, &to, amount, true)
					.map_err(|_| Error::<T>::TransferFromFailed)?;
				T::Currency::mint_into(vault.lp_token_id, from, lp)
					.map_err(|_| Error::<T>::MintFailed)?;
				Ok(lp)
			}
		}

		fn do_lp_share_value(
			vault_id: &VaultIndex,
			vault: &VaultInfo<T>,
			lp_amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let vault_aum = Self::do_assets_under_management(vault_id, vault)?;
			let vault_aum_value = <T::Convert as Convert<T::Balance, u128>>::convert(vault_aum);

			let lp_total_issuance = T::Currency::total_issuance(vault.lp_token_id);
			let lp_total_issuance_value =
				<T::Convert as Convert<T::Balance, u128>>::convert(lp_total_issuance);

			let lp_amount_value = <T::Convert as Convert<T::Balance, u128>>::convert(lp_amount);

			/*
			   a = total lp issued
			   b = asset under management
			   lp_share_percent = lp / a
			   lp_shares_value	= lp_share_percent * b
								= lp / a * b
								= lp * b / a
			*/
			let lp_shares_value =
				multiply_by_rational(lp_amount_value, vault_aum_value, lp_total_issuance_value)
					.map_err(|_| Error::<T>::OverflowError)?;

			let lp_shares_value_amount =
				<T::Convert as Convert<u128, T::Balance>>::convert(lp_shares_value);

			Ok(lp_shares_value_amount)
		}

		/// Computes the sum of all the assets that the vault currently controls.
		fn do_assets_under_management(
			vault_id: &VaultIndex,
			vault: &VaultInfo<T>,
		) -> Result<T::Balance, Error<T>> {
			let owned = T::Currency::balance(vault.asset_id, &Self::account_id(vault_id));
			let outstanding = CapitalStructure::<T>::iter_prefix_values(vault_id)
				.fold(T::Balance::zero(), |sum, item| sum + item.balance);
			Ok(owned + outstanding)
		}
	}

	impl<T: Config> Vault for Pallet<T> {
		type AccountId = T::AccountId;
		type Balance = T::Balance;
		type BlockNumber = T::BlockNumber;
		type VaultId = VaultIndex;
		type AssetId = AssetIdOf<T>;

		fn asset_id(vault_id: &Self::VaultId) -> Result<Self::AssetId, DispatchError> {
			let vault =
				Vaults::<T>::try_get(vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;
			Ok(vault.asset_id)
		}

		fn lp_asset_id(vault_id: &Self::VaultId) -> Result<Self::AssetId, DispatchError> {
			let vault =
				Vaults::<T>::try_get(vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;
			Ok(vault.lp_token_id)
		}

		fn account_id(vault: &Self::VaultId) -> Self::AccountId {
			T::PalletId::get().into_sub_account(vault)
		}

		fn create(
			deposit: Deposit<Self::Balance, Self::BlockNumber>,
			config: VaultConfig<Self::AccountId, Self::AssetId>,
		) -> Result<Self::VaultId, DispatchError> {
			Self::do_create_vault(deposit, config).map(|(id, _)| id)
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
			let vault =
				Vaults::<T>::try_get(&vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;
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
				LpTokensToVaults::<T>::try_get(&token).map_err(|_| Error::<T>::NotVaultLpToken)?;
			Ok(vault_index)
		}
	}

	impl<T: Config> StrategicVault for Pallet<T> {
		fn available_funds(
			vault_id: &Self::VaultId,
			account: &Self::AccountId,
		) -> Result<FundsAvailability<Self::Balance>, DispatchError> {
			match (Vaults::<T>::try_get(vault_id), Allocations::<T>::try_get(vault_id, &account)) {
				(Ok(vault), Ok(allocation)) if !vault.capabilities.is_stopped() => {
					let aum = Self::assets_under_management(vault_id)?;
					let max_allowed = <T::Convert as Convert<u128, T::Balance>>::convert(
						allocation
							.mul_floor(<T::Convert as Convert<T::Balance, u128>>::convert(aum)),
					);
					let state = CapitalStructure::<T>::try_get(vault_id, &account).expect("if a strategy has an allocation, it must have an associated capital structure too");

					if state.balance >= max_allowed {
						Ok(FundsAvailability::Depositable(state.balance - max_allowed))
					} else {
						Ok(FundsAvailability::Withdrawable(max_allowed - state.balance))
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
			let vault =
				Vaults::<T>::try_get(&vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;
			CapitalStructure::<T>::try_mutate(vault_id, to, |state| {
				// I do not thing balance can actually overflow, since the total_issuance <=
				// T::Balance::Max
				state.balance =
					state.balance.checked_add(&amount).ok_or(Error::<T>::OverflowError)?;
				// This can definitely overflow. Perhaps it should be a BigUint?
				state.lifetime_withdrawn = state
					.lifetime_withdrawn
					.checked_add(&amount)
					.ok_or(Error::<T>::OverflowError)?;
				T::Currency::transfer(vault.asset_id, &Self::account_id(vault_id), to, amount, true)
					.map_err(|_| Error::<T>::InsufficientFunds)
			})?;
			Ok(())
		}

		fn deposit(
			vault_id: &Self::VaultId,
			from: &Self::AccountId,
			amount: Self::Balance,
		) -> Result<(), DispatchError> {
			let vault =
				Vaults::<T>::try_get(&vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;
			CapitalStructure::<T>::try_mutate(vault_id, from, |state| {
				// A strategy can return more than it has withdrawn through profits.
				state.balance = state.balance.saturating_sub(&amount);
				// This can definitely overflow. Perhaps it should be a BigUint?
				state.lifetime_deposited = state
					.lifetime_deposited
					.checked_add(&amount)
					.ok_or(Error::<T>::OverflowError)?;
				T::Currency::transfer(
					vault.asset_id,
					from,
					&Self::account_id(vault_id),
					amount,
					true,
				)
				.map_err(|_| Error::<T>::InsufficientFunds)
			})?;
			Ok(())
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
			Vaults::<T>::try_get(&vault_id)
				.map_err(|_| DispatchError::CannotLookup)
				.map(|vault| vault.capabilities.is_stopped())
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
			Vaults::<T>::try_get(&vault_id)
				.map_err(|_| DispatchError::CannotLookup)
				.map(|vault| vault.capabilities.is_tombstoned())
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
			Vaults::<T>::try_get(&vault_id)
				.map_err(|_| DispatchError::CannotLookup)
				.map(|vault| vault.capabilities.withdrawals_allowed())
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
			Vaults::<T>::try_get(&vault_id)
				.map_err(|_| DispatchError::CannotLookup)
				.map(|vault| vault.capabilities.deposits_allowed())
		}
	}
}
