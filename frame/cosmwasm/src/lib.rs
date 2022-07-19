#![feature(generic_associated_types)]
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
)]
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

pub mod instrument;
pub mod runtimes;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use core::fmt::Debug;

	use crate::types::CodeInfo;
	use cosmwasm_minimal_std::{Env, Addr};
	use cosmwasm_vm::system::CosmwasmCodeId;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::{AssetId, Balance},
			Get,
		},
	};
	use sp_runtime::traits::MaybeDisplay;

	pub(crate) type TrieId = Vec<u8>;
	pub(crate) type CodeHash<T> = <T as frame_system::Config>::Hash;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type MaxCodeSizeOf<T> = <T as Config>::MaxCodeSize;
	pub(crate) type MaxInstrumentedCodeSizeOf<T> = <T as Config>::MaxInstrumentedCodeSize;
	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Max accepted code size.
		type MaxCodeSize: Get<u32>;

		/// Max code size after gas instrumentation.
		type MaxInstrumentedCodeSize: Get<u32>;

		/// The user account identifier type for the runtime.
		type AccountId: Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ Debug
			+ MaybeDisplay
			+ Ord
			+ MaxEncodedLen
			+ Into<String>
			+ TryFrom<String>;

		/// Type of an account balance.
		type Balance: Balance;

		/// Type of a tradable asset id.
		type AssetId: AssetId + Into<String> + TryFrom<String>;

		/// Interface from which we are going to execute assets operations.
		type Assets: Inspect<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ Transfer<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ Mutate<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// A mapping from an original code hash to the original code, untouched by instrumentation.
	#[pallet::storage]
	pub(crate) type PristineCode<T: Config> =
		StorageMap<_, Identity, CosmwasmCodeId, BoundedVec<u8, MaxCodeSizeOf<T>>>;

	/// A mapping between an original code hash and instrumented wasm code, ready for execution.
	#[pallet::storage]
	pub(crate) type InstrumentedCode<T: Config> =
		StorageMap<_, Identity, CosmwasmCodeId, BoundedVec<u8, MaxInstrumentedCodeSizeOf<T>>>;

	/// Momotonic counter incremented on code creation.
	#[pallet::storage]
	pub(crate) type CodeIdCounter<T: Config> = StorageValue<_, CosmwasmCodeId, ValueQuery>;

	/// A mapping between an original code hash and its metadata.
	#[pallet::storage]
	pub(crate) type CodeIdToInfo<T: Config> =
		StorageMap<_, Identity, CosmwasmCodeId, CodeInfo<AccountIdOf<T>>>;

	/// A mapping between a code hash and it's unique ID.
	#[pallet::storage]
	pub(crate) type CodeHashToId<T: Config> = StorageMap<_, Identity, CodeHash<T>, CosmwasmCodeId>;

	/// This is a **monotonic** counter incremented on contract instantiation.
	///
	/// This is used in order to generate unique trie ids for contracts.
	/// The trie id of a new contract is calculated from hash(account_id, nonce).
	/// The nonce is required because otherwise the following sequence would lead to
	/// a possible collision of storage:
	///
	/// 1. Create a new contract.
	/// 2. Terminate the contract.
	/// 3. Immediately recreate the contract with the same account_id.
	///
	/// This is bad because the contents of a trie are deleted lazily and there might be
	/// storage of the old instantiation still in it when the new contract is created. Please
	/// note that we can't replace the counter by the block number because the sequence above
	/// can happen in the same block. We also can't keep the account counter in memory only
	/// because storage is the only way to communicate across different extrinsics in the
	/// same block.
	///
	/// # Note
	///
	/// Do not use it to determine the number of contracts. It won't be decremented if
	/// a contract is destroyed.
	#[pallet::storage]
	pub(crate) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}
