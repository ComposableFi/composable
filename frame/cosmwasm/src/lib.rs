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

extern crate alloc;

pub use pallet::*;

pub mod runtimes;
pub mod types;
pub mod instrument;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		runtimes::wasmi::{CosmwasmAccount, CosmwasmVM, CosmwasmVMError},
		types::{CodeInfo, ContractInfo},
	};
	use alloc::{borrow::ToOwned, string::String};
	use core::{fmt::Debug, num::NonZeroU32};
	use cosmwasm_minimal_std::{
		Addr, BlockInfo, ContractInfo as CosmwasmContractInfo, Empty, Env, Event as CosmwasmEvent,
		MessageInfo, Timestamp, TransactionInfo,
	};
	use cosmwasm_vm::{
		executor::{cosmwasm_call, InstantiateInput},
		system::{cosmwasm_system_entrypoint, CosmwasmCodeId},
		vm::VmErrorOf,
	};
	use cosmwasm_vm_wasmi::{host_functions, new_wasmi_vm, WasmiImportResolver, WasmiVM};
	use frame_support::{
		pallet_prelude::*,
		storage::child::ChildInfo,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::{AssetId, Balance},
			Get,
		},
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_runtime::traits::{Convert, MaybeDisplay};
	use sp_runtime::SaturatedConversion;
	use sp_std::vec::Vec;
	use wasm_instrument::gas_metering::Rules;
  use frame_support::StorageHasher;

	pub(crate) type Nonce = u64;
	pub(crate) type ContractTrieIdOf<T> = BoundedVec<u8, MaxContractTrieIdSizeOf<T>>;
	pub(crate) type ContractLabelOf<T> = BoundedVec<u8, MaxContractLabelSizeOf<T>>;
	pub(crate) type CodeHash<T> = <T as frame_system::Config>::Hash;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type MaxCodeSizeOf<T> = <T as Config>::MaxCodeSize;
	pub(crate) type MaxInstrumentedCodeSizeOf<T> = <T as Config>::MaxInstrumentedCodeSize;
	pub(crate) type MaxMessageSizeOf<T> = <T as Config>::MaxMessageSize;
	pub(crate) type MaxContractLabelSizeOf<T> = <T as Config>::MaxContractLabelSize;
	pub(crate) type MaxContractTrieIdSizeOf<T> = <T as Config>::MaxContractTrieIdSize;
	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub type ContractInfoOf<T> =
		ContractInfo<AccountIdOf<T>, ContractLabelOf<T>, ContractTrieIdOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Uploaded { code_hash: CodeHash<T>, code_id: CosmwasmCodeId },
		Instantiated { contract: AccountIdOf<T>, info: ContractInfoOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		InstrumentationFailed,
		VmCreationFailed,
		ExecutionFailed,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Max accepted code size.
		type MaxCodeSize: Get<u32>;

		/// Max code size after gas instrumentation.
		type MaxInstrumentedCodeSize: Get<u32>;

		/// Max message size.
		type MaxMessageSize: Get<u32>;

		/// Max contract label size.
		type MaxContractLabelSize: Get<u32>;

		/// Max contract trie id size.
		type MaxContractTrieIdSize: Get<u32>;

		/// A way to convert from our native account to cosmwasm `Addr`.
		type AccountToAddr: Convert<AccountIdOf<Self>, String>
			+ Convert<String, Result<AccountIdOf<Self>, ()>>;

		/// Type of an account balance.
		type Balance: Balance + From<u128>;

		/// Type of a tradable asset id.
		type AssetId: AssetId;

		/// A way to convert from our native currency to cosmwasm `Denom`.
		type AssetToDenom: Convert<AssetIdOf<Self>, String>
			+ Convert<String, Result<AssetIdOf<Self>, ()>>;

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
	pub(crate) type CurrentCodeId<T: Config> = StorageValue<_, CosmwasmCodeId, ValueQuery>;

	/// A mapping between an original code hash and its metadata.
	#[pallet::storage]
	pub(crate) type CodeIdToInfo<T: Config> =
		StorageMap<_, Identity, CosmwasmCodeId, CodeInfo<AccountIdOf<T>>>;

	/// A mapping between a code hash and it's unique ID.
	#[pallet::storage]
	pub(crate) type CodeHashToId<T: Config> = StorageMap<_, Identity, CodeHash<T>, CosmwasmCodeId>;

	/// This is a **monotonic** counter incremented on contract instantiation.
	/// The purpose of this nonce is just to make sure that contract trie are unique.
	#[pallet::storage]
	pub(crate) type CurrentNonce<T: Config> = StorageValue<_, Nonce, ValueQuery>;

	/// A mapping between a contract and it's metadata.
	#[pallet::storage]
	pub(crate) type ContractToInfo<T: Config> =
		StorageMap<_, Identity, AccountIdOf<T>, ContractInfoOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn upload(
			origin: OriginFor<T>,
			code: BoundedVec<u8, MaxCodeSizeOf<T>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Self::do_upload(&who)?;
			Ok(().into())
		}

		// #[pallet::weight(0)]
		// pub fn instantiate(
		// 	origin: OriginFor<T>,
		// 	code_id: CosmwasmCodeId,
		// 	message: BoundedVec<u8, MaxMessageSizeOf<T>>,
		// ) -> DispatchResultWithPostInfo {
		// 	let who = ensure_signed(origin)?;
		// 	Self::do_instantiate(&CosmwasmAccount::new(who), &code, &message)?;
		// 	Ok(().into())
		// }
	}

	struct DummyCostRules;
	impl Rules for DummyCostRules {
		fn instruction_cost(
			&self,
			_: &wasm_instrument::parity_wasm::elements::Instruction,
		) -> Option<u32> {
			Some(42)
		}

		fn memory_grow_cost(&self) -> wasm_instrument::gas_metering::MemoryGrowCost {
			wasm_instrument::gas_metering::MemoryGrowCost::Linear(
				NonZeroU32::new(1024).expect("impossible"),
			)
		}
	}

	impl<T: Config> Pallet<T> {
		fn do_upload(who: &AccountIdOf<T>) -> DispatchResult {
			Ok(())
		}

		// fn do_instantiate(
		// 	who: &CosmwasmAccount<T, AccountIdOf<T>>,
		// 	code: &[u8],
		// 	message: &[u8],
		// ) -> DispatchResult {
		// 	let instrumented_code =
		// 		gas_and_stack_instrumentation("env", code, 1000, &DummyCostRules)
		// 			.map_err(|_| Error::<T>::InstrumentationFailed)?;
		// 	let host_functions_definitions = WasmiImportResolver(host_functions::definitions());
		// 	let module = new_wasmi_vm(&host_functions_definitions, &instrumented_code)
		// 		.map_err(|_| Error::<T>::VmCreationFailed)?;
		// 	let mut vm = WasmiVM(CosmwasmVM::<T> {
		// 		host_functions: host_functions_definitions
		// 			.0
		// 			.clone()
		// 			.into_iter()
		// 			.flat_map(|(_, modules)| modules.into_iter().map(|(_, function)| function))
		// 			.collect(),
		// 		executing_module: module,
		// 		env: Env {
		// 			block: BlockInfo {
		// 				height: frame_system::Pallet::<T>::block_number().saturated_into(),
		// 				time: Timestamp(0),
		// 				chain_id: "picasso".to_owned(),
		// 			},
		// 			transaction: frame_system::Pallet::<T>::extrinsic_index()
		// 				.map(|index| TransactionInfo { index }),
		// 			contract: CosmwasmContractInfo {
		// 				address: Addr::unchecked(Into::<String>::into(who.clone())),
		// 			},
		// 		},
		// 		info: MessageInfo {
		// 			sender: Addr::unchecked(Into::<String>::into(who.clone())),
		// 			funds: Default::default(),
		// 		},
		// 		_marker: PhantomData,
		// 	});
		// 	let result = cosmwasm_system_entrypoint::<InstantiateInput<Empty>, _>(&mut vm, message);
		// 	log::debug!(target: "runtime::contracts", "Result: {:#?}", result);
		// 	Ok(())
		// }

	  fn determine_contract_address(
		  instantiator: &T::AccountId,
		  code_hash: &CodeHash<T>,
		  salt: &[u8],
	  ) -> T::AccountId {
		  let buf: Vec<_> = instantiator
			  .as_ref()
			  .iter()
			  .chain(code_hash.as_ref())
			  .chain(salt)
			  .cloned()
			  .collect();
		  T::Hashing::hash(&buf)
	  }

		fn contract_info(
			contract: &CosmwasmAccount<T, AccountIdOf<T>>,
		) -> Result<ContractInfoOf<T>, Error<T>> {
			todo!()
		}

		fn contract_child_trie(trie_id: &[u8]) -> ChildInfo {
			ChildInfo::new_default(trie_id)
		}

		fn determine_contract_trie_id(
			contract: &CosmwasmAccount<T, AccountIdOf<T>>,
			nonce: Nonce,
		) -> ContractTrieIdOf<T> {
			todo!()
		}

		fn do_db_read(
			who: &CosmwasmAccount<T, AccountIdOf<T>>,
			key: Vec<u8>,
		) -> Result<Vec<u8>, Error<T>> {
			todo!()
		}

		fn do_db_write(
			who: &CosmwasmAccount<T, AccountIdOf<T>>,
			key: Vec<u8>,
			value: Vec<u8>,
		) -> Result<(), Error<T>> {
			todo!()
		}
	}
}
