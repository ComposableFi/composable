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
pub use pallet::*;
mod rate_model;

#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, FullCodec};
	use composable_traits::{lending::{Lending, LendingConfig}, oracle::Oracle, vault::{Deposit, Vault, VaultConfig}};
	use frame_support::{
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
		Perquintill,
	};
	use sp_std::fmt::Debug;

	pub const PALLET_ID: PalletId = PalletId(*b"Lending!");

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type VaultId: Clone + Codec + Debug + PartialEq;
		type PairId: Clone + Codec + Debug + PartialEq;
		type Oracle : Oracle;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		Overflow,
	}

	/// stores all market pairs of assets to be assets/collateral
	// only assets supported by `Oracle` are possible
	#[pallet::storage]
	#[pallet::getter(fn allocations)]
	pub type Pairs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PairId,
		LendingConfig<T::AccountId, T::VaultId>,
		ValueQuery,
	>;

	impl<T:Config> Lending for Pallet<T> {
		type AssetId = T::AssetId;

		type VaultId = T::VaultId;

		type AccountId = T::AccountId;

		type PairId = T::PairId;

		type Error = Error<T>;

		type Balance;

		type BlockNumber;

		fn create(
			collateral: Self::VaultId,
			asset: Self::VaultId,
			deposit: Deposit<Self::Balance, Self::BlockNumber>,
			config: composable_traits::lending::LendingConfig<Self::AccountId, Self::AssetId>,
		) -> Result<Self::PairId, Self::Error> {
			todo!()
		}

		fn get_pair_in_vault(vault: Self::VaultId) -> Result<Vec<Self::PairId>, Self::Error> {
			todo!()
		}

		fn get_pairs_all() -> Result<Vec<Self::PairId>, Self::Error> {
			todo!()
		}

		fn borrow(
			pair: Self::PairId,
			debt_owner: &Self::AccountId,
			amount_to_borrow: Self::Balance,
		) -> Result<(), Self::Error> {
			todo!()
		}

		fn repay_borrow(
			pair: Self::PairId,
			from: &Self::AccountId,
			beneficiary: &Self::AccountId,
			repay_amount: Self::Balance,
		) -> Result<(), Self::Error> {
			todo!()
		}

		fn redeem(
			pair: Self::PairId,
			to: &Self::AccountId,
			redeem_amount: Self::Balance,
		) -> Result<(), Self::Error> {
			todo!()
		}

		fn calculate_liquidation_fee(amount: Self::Balance) -> Self::Balance {
			todo!()
		}

		fn total_borrows(pair: Self::PairId) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn accrue_interest(pair: Self::PairId) -> Result<(), Self::Error> {
			todo!()
		}

		fn borrow_balance_current(
			pair: Self::PairId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn withdraw_fees(to_withdraw: Self::Balance) -> Result<(), Self::Error> {
			todo!()
		}

		fn collateral_of_account(
			pair: Self::PairId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn collateral_required(
			pair: Self::PairId,
			borrow_amount: Self::Balance,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn get_borrow_limit(
			pair: Self::PairId,
			account: Self::AccountId,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}
	}


}
