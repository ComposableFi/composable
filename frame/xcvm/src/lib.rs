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
#![deny(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
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
#![doc = include_str!("../README.md")]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::storage::bounded_btree_map::BoundedBTreeMap;
	use frame_support::traits::tokens::AssetId;
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect as FungiblesInspect, Transfer as FungiblesTransfer},
		traits::tokens::Balance,
	};
	use frame_system::pallet_prelude::*;
	use xcvm_core::{AbiEncoded, XCVMInstruction, XCVMNetwork};

	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub(crate) type MaxTransferAssetsOf<T> = <T as Config>::MaxTransferAssets;
	pub(crate) type MaxInstructionsOf<T> = <T as Config>::MaxInstructions;
	pub(crate) type XCVMInstructionOf<T> = XCVMInstruction<
		XCVMNetwork,
		AbiEncoded,
		AccountIdOf<T>,
		BoundedBTreeMap<AssetIdOf<T>, BalanceOf<T>, MaxTransferAssetsOf<T>>,
	>;
	pub(crate) type XCVMInstructionsOf<T> = BoundedVec<XCVMInstructionOf<T>, MaxInstructionsOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Executed { instruction: XCVMInstructionOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AssetId: AssetId + Ord;
		type Assets: FungiblesInspect<Self::AccountId, AssetId = AssetIdOf<Self>, Balance = BalanceOf<Self>>
			+ FungiblesTransfer<Self::AccountId>;
		type Balance: Balance;
		type MaxTransferAssets: Get<u32>;
		type MaxInstructions: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn execute(
			origin: OriginFor<T>,
			instructions: XCVMInstructionsOf<T>,
		) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
	}
}
