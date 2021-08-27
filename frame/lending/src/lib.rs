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


#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, FullCodec};
	use composable_traits::vault::{Deposit, Vault, VaultConfig};
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

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::error]
	pub enum Error<T> {
		Overflow,
	}

	trait Lending {}

	impl<T: Config> Lending for Pallet<T> {

	}
}
