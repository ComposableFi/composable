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

mod rate_model;

#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, EncodeLike, FullCodec};
	use composable_traits::{
		lending::{Lending, LendingConfigInput},
		oracle::Oracle,
		vault::{Deposit, Vault, VaultConfig},
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::{fungibles::MutateHold, DepositConsequence},
		},
		PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor, Config as SystemConfig};
	use num_traits::{Bounded, SaturatingSub};
	use sp_runtime::{
		helpers_128bit::multiply_by_rational,
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Convert,
			Hash, Zero,
		},
		Permill, Perquintill,
	};
	use sp_std::fmt::Debug;

	#[derive(Default, Copy, Clone, Encode, Decode)]
	#[repr(transparent)]
	pub struct LendingIndex(u32);

	pub const PALLET_ID: PalletId = PalletId(*b"Lending!");

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Oracle: Oracle<AssetId = Self::AssetId>;
		type Vault: Vault<AssetId = Self::AssetId>;
		type Balance;
		type AssetId: core::cmp::Ord;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		Overflow,
		/// vault provided does not exist
		VaultNotFound,
		/// Only assets for which we can track price are supported
		AssetWithoutPrice,
	}

	#[derive(Encode, Decode, Default)]
	pub struct LendingConfig {
		pub reserve_factor: Permill,
		pub collateral_factor: Permill,
	}

	/// Lending instances counter
	#[pallet::storage]
	#[pallet::getter(fn lending_count)]
	pub type LendingCount<T: Config> = StorageValue<_, LendingIndex, ValueQuery>;

	/// Indexed lending instances
	#[pallet::storage]
	#[pallet::getter(fn pairs)]
	pub type Lendings<T: Config> =
		StorageMap<_, Twox64Concat, LendingIndex, LendingConfig, ValueQuery>;

	impl<T: Config> Lending for Pallet<T> {
		type VaultId = <T::Vault as Vault>::VaultId;

		type AccountId = T::AccountId;

		type LendingId = LendingIndex;

		type Error = Error<T>;

		type Balance = T::Balance;

		type BlockNumber = T::BlockNumber;

		fn create_or_update(
			deposit: <T::Vault as Vault>::VaultId,
			collateral: <T::Vault as Vault>::VaultId,
			config_input: LendingConfigInput<Self::AccountId>,
		) -> Result<(), DispatchError> {
			LendingCount::<T>::try_mutate(|LendingIndex(previous_lending_index)| {
				let lending_index = {
					*previous_lending_index += 1;
					LendingIndex(*previous_lending_index)
				};
				let collateral_asset = T::Vault::asset_id(&collateral)?;
				let deposit_asset = T::Vault::asset_id(&deposit)?;
				let config = LendingConfig {
					reserve_factor: config_input.reserve_factor,
					collateral_factor: config_input.collateral_factor,
				};

				<T::Oracle as Oracle>::get_price(collateral_asset)
					.map_err(|_| Error::<T>::AssetWithoutPrice)?;
				<T::Oracle as Oracle>::get_price(deposit_asset)
					.map_err(|_| Error::<T>::AssetWithoutPrice)?;

				Lendings::<T>::insert(lending_index, config);

				Ok(())
			})
		}

		fn account_id(lending_id: &Self::LendingId) -> Self::AccountId {
			PALLET_ID.into_sub_account(lending_id)
		}

		fn get_pair_in_vault(vault: Self::VaultId) -> Result<Vec<Self::LendingId>, Self::Error> {
			todo!()
		}

		fn get_pairs_all() -> Result<Vec<Self::LendingId>, Self::Error> {
			todo!()
		}

		fn borrow(
			lending_id: &Self::LendingId,
			debt_owner: &Self::AccountId,
			amount_to_borrow: Self::Balance,
		) -> Result<(), Self::Error> {
			todo!()
		}

		fn repay_borrow(
			lending_id: &Self::LendingId,
			from: &Self::AccountId,
			beneficiary: &Self::AccountId,
			repay_amount: Self::Balance,
		) -> Result<(), Self::Error> {
			todo!()
		}

		fn total_borrows(lending_id: &Self::LendingId) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn accrue_interest(lending_id: &Self::LendingId) -> Result<(), Self::Error> {
			todo!()
		}

		fn borrow_balance_current(
			lending_id: &Self::LendingId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn collateral_of_account(
			lending_id: &Self::LendingId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn collateral_required(
			lending_id: &Self::LendingId,
			borrow_amount: Self::Balance,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn get_borrow_limit(
			lending_id: &Self::LendingId,
			account: Self::AccountId,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}
	}
}
