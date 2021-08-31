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
// TODO remove me!
#![allow(missing_docs)]

mod models;
mod rent;
mod traits;

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
	use composable_traits::vault::{
		Deposit, FundsAvailability, ReportableStrategicVault, Vault, VaultConfig,
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
	use frame_system::{ensure_signed, pallet_prelude::OriginFor, Config as SystemConfig};
	use num_traits::SaturatingSub;
	use sp_runtime::{
		helpers_128bit::multiply_by_rational,
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Convert,
			Zero,
		},
		DispatchError, Perquintill,
	};
	use sp_std::fmt::Debug;

	// TODO(kaiserkarel) name this better
	pub const PALLET_ID: PalletId = PalletId(*b"Vaults!!");

	pub type CurrencyIdOf<T> =
		<<T as Config>::Currency as Inspect<<T as SystemConfig>::AccountId>>::AssetId;
	pub type AccountIdOf<T> = <T as SystemConfig>::AccountId;
	pub type BlockNumberOf<T> = <T as SystemConfig>::BlockNumber;
	pub type BalanceOf<T> = <T as Config>::Balance;

	pub type VaultInfo<T> =
		crate::models::VaultInfo<AccountIdOf<T>, BalanceOf<T>, CurrencyIdOf<T>, BlockNumberOf<T>>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
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
			+ Zero
			+ From<u64>;
		type CurrencyFactory: CurrencyFactory<Self::CurrencyId>;
		type CurrencyId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default;
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::CurrencyId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::CurrencyId>
			+ MutateHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::CurrencyId>;

		type Convert: Convert<Self::Balance, u128> + Convert<u128, Self::Balance>;
		type MaxStrategies: Get<usize>;

		#[pallet::constant]
		type NativeAssetId: Get<Self::CurrencyId>;

		#[pallet::constant]
		type CreationDeposit: Get<Self::Balance>;

		#[pallet::constant]
		type ExistentialDeposit: Get<Self::Balance>;

		#[pallet::constant]
		type RentPerBlock: Get<Self::Balance>;

		type StrategyReport: FullCodec + Default;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn vault_count)]
	pub type VaultCount<T: Config> = StorageValue<_, VaultIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn vault_data)]
	pub type Vaults<T: Config> = StorageMap<_, Twox64Concat, VaultIndex, VaultInfo<T>, ValueQuery>;

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

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emitted after a vault has been succesfully created.
		VaultCreated(VaultIndex, VaultInfo<T>),
		// Deposited(Value, LpShareMinted)
		Deposited(T::AccountId, T::Balance, T::Balance),
		// Withdrawn(LpShareBurnt, ShareValue)
		Withdrawn(T::AccountId, T::Balance, T::Balance),
	}

	#[pallet::error]
	pub enum Error<T> {
		InsufficientBalance,
		CannotCreateAsset,
		InconsistentStorage,
		TransferFromFailed,
		MintFailed,
		InsufficientLpTokens,
		LookupError,
		VaultDoesNotExist,
		NoFreeVaultAllocation,
		AllocationMustSumToOne,
		TooManyStrategies,
		OverflowError,
		InsufficientFunds,
		AmountMustBePositive,
		NotEnoughLiquidity,
		DepositIsTooLow,
		InvalidSurchargeClaim,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create(
			origin: OriginFor<T>,
			vault: VaultConfig<AccountIdOf<T>, CurrencyIdOf<T>>,
			deposit: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;

			ensure!(deposit >= T::CreationDeposit::get(), Error::<T>::DepositIsTooLow);

			let native_id = T::NativeAssetId::get();
			T::Currency::hold(native_id, &from, deposit)?;

			let deposit = if deposit > T::ExistentialDeposit::get() {
				Deposit::Existential
			} else {
				Deposit::Rent { amount: deposit, at: <frame_system::Pallet<T>>::block_number() }
			};
			<Self as Vault>::create(deposit, vault)?;
			Ok(().into())
		}

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

		// TODO: weight
		#[pallet::weight(10_000)]
		pub fn deposit(
			origin: OriginFor<T>,
			vault: VaultIndex,
			asset_amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			let lp_amount = <Self as Vault>::deposit(&vault, &from, asset_amount)?;
			Self::deposit_event(Event::Deposited(from, asset_amount, lp_amount));
			Ok(().into())
		}

		// TODO: weight
		#[pallet::weight(10_000)]
		pub fn withdraw(
			origin: OriginFor<T>,
			vault: VaultIndex,
			lp_amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let to = ensure_signed(origin)?;
			let asset_amount = <Self as Vault>::withdraw(&vault, &to, lp_amount)?;
			Self::deposit_event(Event::Withdrawn(to, lp_amount, asset_amount));
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn do_create_vault(
			deposit: Deposit<BalanceOf<T>, BlockNumberOf<T>>,
			config: VaultConfig<T::AccountId, T::CurrencyId>,
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

				let lp_token_id = {
					T::CurrencyFactory::create().map_err(|e| {
						log::debug!("failed to create asset: {:?}", e);
						Error::<T>::CannotCreateAsset
					})?
				};

				config.strategies.into_iter().for_each(|(account_id, allocation)| {
					Allocations::<T>::insert(id, account_id, allocation);
				});

				Allocations::<T>::insert(id, Self::account_id(), config.reserved);

				let vault_info = crate::models::VaultInfo {
					lp_token_id,
					manager: config.manager,
					asset_id: config.asset_id,
					deposit,
				};

				Vaults::<T>::insert(id, vault_info.clone());

				Ok((id, vault_info))
			})
		}

		fn account_id() -> T::AccountId {
			PALLET_ID.into_account()
		}

		/// Computes the sum of all the assets that the vault currently controls.
		fn assets_under_management(vault_id: &VaultIndex) -> Result<T::Balance, Error<T>> {
			let vault =
				Vaults::<T>::try_get(vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;
			let owned = T::Currency::balance(vault.asset_id, &Self::account_id());
			let outstanding = CapitalStructure::<T>::iter_prefix_values(vault_id)
				.fold(T::Balance::zero(), |sum, item| sum + item.balance);
			Ok(owned + outstanding)
		}

		fn do_withdraw(
			vault_id: &VaultIndex,
			to: &T::AccountId,
			lp_amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let vault =
				Vaults::<T>::try_get(&vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;

			let vault_aum = Self::assets_under_management(vault_id)?;
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

			// Should represent the deposited funds + interests
			let lp_shares_value_amount =
				<T::Convert as Convert<u128, T::Balance>>::convert(lp_shares_value);

			let vault_owned_amount = T::Currency::balance(vault.asset_id, &Self::account_id());

			// TODO(hussein-aitlahcen): should we provide what we can to reduce the available
			// liquidity in order to force strategies to rebalance?
			ensure!(lp_shares_value_amount <= vault_owned_amount, Error::<T>::NotEnoughLiquidity);

			ensure!(
				T::Currency::can_withdraw(vault.lp_token_id, &to, lp_amount)
					.into_result()
					.is_ok(),
				Error::<T>::InsufficientLpTokens
			);

			let from = Self::account_id();
			ensure!(
				T::Currency::can_withdraw(vault.asset_id, &from, lp_shares_value_amount)
					.into_result()
					.is_ok(),
				Error::<T>::TransferFromFailed
			);

			T::Currency::burn_from(vault.lp_token_id, &to, lp_amount)
				.map_err(|_| Error::<T>::InsufficientLpTokens)?;
			T::Currency::transfer(vault.asset_id, &from, &to, lp_shares_value_amount, true)
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

			ensure!(
				T::Currency::can_withdraw(vault.asset_id, &from, amount).into_result().is_ok(),
				Error::<T>::TransferFromFailed
			);

			let to = Self::account_id();

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

				ensure!(lp > T::Balance::zero(), Error::<T>::DepositIsTooLow);

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
	}

	impl<T: Config> Vault for Pallet<T> {
		type AccountId = T::AccountId;
		type Balance = T::Balance;
		type BlockNumber = T::BlockNumber;
		type VaultId = VaultIndex;
		type AssetId = CurrencyIdOf<T>;

		fn asset_id(vault_id: &Self::VaultId) -> Result<Self::AssetId, DispatchError> {
			let vault =
				Vaults::<T>::try_get(vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;
			Ok(vault.asset_id)
		}

		fn account_id() -> Self::AccountId {
			Pallet::<T>::account_id()
		}

		fn create(
			deposit: Deposit<Self::Balance, Self::BlockNumber>,
			config: VaultConfig<Self::AccountId, Self::AssetId>,
		) -> Result<Self::VaultId, DispatchError> {
			Self::do_create_vault(deposit, config).map(|(id, _)| id)
		}

		fn deposit(
			vault: &Self::VaultId,
			from: &Self::AccountId,
			asset_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			ensure!(asset_amount > T::Balance::zero(), Error::<T>::AmountMustBePositive);
			Pallet::<T>::do_deposit(&vault, &from, asset_amount)
		}

		fn withdraw(
			vault: &Self::VaultId,
			to: &Self::AccountId,
			lp_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			ensure!(lp_amount > T::Balance::zero(), Error::<T>::AmountMustBePositive);
			Pallet::<T>::do_withdraw(&vault, &to, lp_amount)
		}

		fn lp_asset_id(vault: &Self::VaultId) -> Result<Self::AssetId, DispatchError> {
			todo!()
		}
	}

	impl<T: Config> StrategicVault for Pallet<T> {
		fn available_funds(
			vault_id: &Self::VaultId,
			account: &Self::AccountId,
		) -> Result<FundsAvailability<Self::Balance>, DispatchError> {
			let allocation = match Allocations::<T>::try_get(vault_id, &account) {
				Ok(allocation) => allocation,
				// The strategy was removed by the fund manager or governance, so all funds should
				// be returned.
				Err(_) => return Ok(FundsAvailability::MustLiquidate),
			};

			let aum = Self::assets_under_management(vault_id)?;
			let max_allowed = allocation.mul_floor(aum);

			// if a strategy has an allocation, it must have an associated capital structure too.
			let state = CapitalStructure::<T>::try_get(vault_id, &account)
				.map_err(|_| Error::<T>::InconsistentStorage)?;
			if state.balance >= max_allowed {
				Ok(FundsAvailability::Depositable(state.balance - max_allowed))
			} else {
				Ok(FundsAvailability::Withdrawable(max_allowed - state.balance))
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
				state.balance.checked_add(&amount).ok_or(Error::<T>::OverflowError)?;
				// This can definitely overflow. Perhaps it should be a BigUint?
				state.lifetime_withdrawn.checked_add(&amount).ok_or(Error::<T>::OverflowError)?;
				T::Currency::transfer(vault.asset_id, &Self::account_id(), to, amount, true)
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
				state.balance.saturating_sub(&amount);
				// This can definitely overflow. Perhaps it should be a BigUint?
				state.lifetime_deposited.checked_add(&amount).ok_or(Error::<T>::OverflowError)?;
				T::Currency::transfer(vault.asset_id, from, &Self::account_id(), amount, true)
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
}
