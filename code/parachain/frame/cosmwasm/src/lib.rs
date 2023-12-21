#![recursion_limit = "256"]
#![feature(sync_unsafe_cell)]
#![feature(trait_alias)]
#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic,
		clippy::unseparated_literal_suffix,
		clippy::disallowed_types
	)
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
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
#![doc = include_str!("../README.md")]

extern crate alloc;

use alloc::string::ToString;

pub use pallet::*;
pub mod crypto;
pub mod dispatchable_call;
pub mod ibc;
pub mod instrument;
pub mod pallet_hook;
mod prelude;
pub mod runtimes;
pub mod types;
pub mod utils;
pub mod weights;
pub use crate::ibc::NoRelayer;
pub mod entrypoint;
mod mapping;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(any(test, fuzzing))]
pub mod mock;
#[cfg(test)]
mod tests;

const SUBSTRATE_ECDSA_SIGNATURE_LEN: usize = 65;
use crate::{
	entrypoint::*,
	instrument::{gas_and_stack_instrumentation, INSTRUMENTATION_VERSION},
	pallet_hook::PalletHook,
	runtimes::{
		abstraction::{CosmwasmAccount, Gas, VMPallet},
		vm::{
			ContractBackend, CosmwasmVM, CosmwasmVMCache, CosmwasmVMError, CosmwasmVMShared,
			InitialStorageMutability,
		},
	},
	types::*,
};
use alloc::{
	collections::{btree_map::Entry, BTreeMap},
	format,
	string::String,
};
use composable_support::abstractions::utils::increment::Increment;
use cosmwasm_std::{
	Addr, Attribute as CosmwasmEventAttribute, Binary as CosmwasmBinary, BlockInfo,
	CodeInfoResponse, Coin, ContractInfo as CosmwasmContractInfo, ContractInfoResponse, Env,
	Event as CosmwasmEvent, MessageInfo, Timestamp, TransactionInfo,
};
use cosmwasm_vm::{
	executor::{cosmwasm_call, QueryCall, QueryResponse},
	system::{cosmwasm_system_query, CosmwasmCodeId, CosmwasmContractMeta},
};
use cosmwasm_vm_wasmi::{
	new_wasmi_vm,
	validation::{CodeValidation, ValidationError},
	version::{Version, Version1x},
	OwnedWasmiVM,
};
use frame_support::{
	dispatch::{DispatchErrorWithPostInfo, DispatchResultWithPostInfo, PostDispatchInfo},
	pallet_prelude::*,
	storage::child::ChildInfo,
	traits::{
		fungibles::{Inspect as FungiblesInspect, Mutate as FungiblesMutate},
		tokens::Preservation,
		Get, ReservableCurrency, UnixTime,
	},
	ReversibleStorageHasher, StorageHasher,
};
use sp_runtime::traits::SaturatedConversion;
use sp_std::vec::Vec;
use wasmi::AsContext;
use wasmi_validation::PlainValidator;

#[allow(clippy::too_many_arguments)]
#[frame_support::pallet]
pub mod pallet {
	use crate::{
		instrument::CostRules, pallet_hook::PalletHook, runtimes::vm::InitialStorageMutability,
		types::*, weights::WeightInfo,
	};
	use alloc::{string::String, vec};
	use composable_support::abstractions::{
		nonce::Nonce,
		utils::{increment::SafeIncrement, start_at::ZeroInit},
	};
	use core::fmt::Debug;
	use cosmwasm_vm::system::CosmwasmCodeId;

	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect as FungiblesInspect, Mutate as FungiblesMutate},
			tokens::{AssetId, Balance},
			Get, ReservableCurrency, UnixTime,
		},
		transactional, PalletId, Twox64Concat,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_core::crypto::UncheckedFrom;
	use sp_runtime::traits::{Convert, MaybeDisplay};
	use sp_std::vec::Vec;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Uploaded { code_hash: [u8; 32], code_id: CosmwasmCodeId },
		Instantiated { contract: AccountIdOf<T>, info: ContractInfoOf<T> },
		Executed { contract: AccountIdOf<T>, entrypoint: EntryPoint, data: Option<Vec<u8>> },
		ExecutionFailed { contract: AccountIdOf<T>, entrypoint: EntryPoint, error: Vec<u8> },
		Emitted { contract: AccountIdOf<T>, ty: Vec<u8>, attributes: Vec<(Vec<u8>, Vec<u8>)> },
		Migrated { contract: AccountIdOf<T>, to: CosmwasmCodeId },
		AdminUpdated { contract: AccountIdOf<T>, new_admin: Option<AccountIdOf<T>> },
	}

	#[pallet::error]
	pub enum Error<T> {
		Instrumentation,
		VmCreation,
		ContractHasNoInfo,
		CodeDecoding,
		CodeValidation,
		CodeEncoding,
		CodeInstrumentation,
		InstrumentedCodeIsTooBig,
		CodeAlreadyExists,
		CodeNotFound,
		ContractAlreadyExists,
		ContractNotFound,
		SubstrateDispatch,
		AssetConversion,
		TransferFailed,
		LabelTooBig,
		UnknownDenom,
		StackOverflow,
		NotEnoughFundsForUpload,
		NonceOverflow,
		RefcountOverflow,
		VMDepthOverflow,
		SignatureVerificationError,
		IteratorIdOverflow,
		IteratorNotFound,
		IteratorValueNotFound,
		NotAuthorized,
		NotImplemented,
		Unsupported,
		ExecuteDeserialize,
		Ibc,
		FailedToSerialize,
		OutOfGas,
		InvalidGasCheckpoint,
		InvalidSalt,
		InvalidAccount,
		Interpreter,
		VirtualMachine,
		AccountConversionFailure,
		Aborted,
		ReadOnlyViolation,
		Rpc,
		Precompile,
		QueryDeserialize,
		ExecuteSerialize,
		Xcm,
	}

	#[pallet::config]
	pub trait Config:
		frame_system::Config<AccountId = AccountIdOf<Self>> + Send + Sync + Debug
	{
		/// Max number of frames a contract is able to push, a.k.a recursive calls.
		const MAX_FRAMES: u8;

		#[allow(missing_docs)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type AccountIdExtended: Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ Debug
			+ MaybeDisplay
			+ Ord
			+ MaxEncodedLen
			+ UncheckedFrom<Self::Hash>
			+ AsRef<[u8]>;

		/// Pallet unique ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Current chain ID. Provided to the contract via the [`Env`].
		#[pallet::constant]
		type ChainId: Get<&'static str>;

		/// Max accepted code size in bytes.
		#[pallet::constant]
		type MaxCodeSize: Get<u32>;

		/// Max code size after gas instrumentation.
		#[pallet::constant]
		type MaxInstrumentedCodeSize: Get<u32>;

		/// Max message size in bytes.
		#[pallet::constant]
		type MaxMessageSize: Get<u32>;

		/// Max contract label size.
		#[pallet::constant]
		type MaxContractLabelSize: Get<u32>;

		/// Max contract trie id size.
		#[pallet::constant]
		type MaxContractTrieIdSize: Get<u32>;

		/// Max instantiate salt.
		#[pallet::constant]
		type MaxInstantiateSaltSize: Get<u32>;

		/// Max assets in a [`FundsOf`] batch.
		#[pallet::constant]
		type MaxFundsAssets: Get<u32>;

		/// Max wasm table size.
		#[pallet::constant]
		type CodeTableSizeLimit: Get<u32>;

		/// Max wasm globals limit.
		#[pallet::constant]
		type CodeGlobalVariableLimit: Get<u32>;

		/// Max wasm functions parameters limit.
		#[pallet::constant]
		type CodeParameterLimit: Get<u32>;

		/// Max wasm branch table size limit.
		#[pallet::constant]
		type CodeBranchTableSizeLimit: Get<u32>;

		/// Max wasm stack size limit.
		#[pallet::constant]
		type CodeStackLimit: Get<u32>;

		/// Price of a byte when uploading new code.
		/// The price is expressed in [`Self::NativeAsset`].
		/// This amount is reserved from the owner and released when the code is destroyed.
		#[pallet::constant]
		type CodeStorageByteDeposit: Get<u32>;

		/// Price of writing a byte in the storage.
		#[pallet::constant]
		type ContractStorageByteWritePrice: Get<u32>;

		/// Price of extracting a byte from the storage.
		#[pallet::constant]
		type ContractStorageByteReadPrice: Get<u32>;

		#[pallet::constant]
		type WasmCostRules: Get<CostRules<Self>>;

		/// A way to convert from our native account to cosmwasm `Addr`.
		type AccountToAddr: Convert<AccountIdOf<Self>, String>
			+ Convert<String, Result<AccountIdOf<Self>, ()>>
			+ Convert<Vec<u8>, Result<AccountIdOf<Self>, ()>>;

		/// Type of an account balance.
		type Balance: Balance + Into<u128>;

		/// Type of a tradable asset id.
		///
		/// The [`Ord`] constraint is required for [`BoundedBTreeMap`].
		type AssetId: AssetId + Ord;

		/// A way to convert from our native currency to cosmwasm `Denom`.
		type AssetToDenom: Convert<AssetIdOf<Self>, String>
			+ Convert<String, Result<AssetIdOf<Self>, ()>>;

		/// Interface used to pay when uploading code.
		type NativeAsset: ReservableCurrency<AccountIdOf<Self>, Balance = BalanceOf<Self>>;

		/// Interface from which we are going to execute assets operations.
		type Assets: FungiblesInspect<
				AccountIdOf<Self>,
				Balance = BalanceOf<Self>,
				AssetId = AssetIdOf<Self>,
			> + FungiblesMutate<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>;

		/// Source of time.
		type UnixTime: UnixTime;

		/// Weight implementation.
		type WeightInfo: WeightInfo;

		/// account which execute relayer calls IBC exported entry points
		type IbcRelayerAccount: Get<AccountIdOf<Self>>;

		type IbcRelayer: ibc_primitives::IbcHandler<AccountIdOf<Self>>;

		/// A hook into the VM execution semantic, allowing the runtime to hook into a contract
		/// execution.
		type PalletHook: PalletHook<Self>;

		/// Origin to upload a WASM code
		type UploadWasmOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		type ExecuteWasmOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// A mapping from an original code id to the original code, untouched by instrumentation.
	#[pallet::storage]
	pub(crate) type PristineCode<T: Config> =
		StorageMap<_, Twox64Concat, CosmwasmCodeId, BoundedVec<u8, MaxCodeSizeOf<T>>>;

	/// A mapping between an original code id and instrumented wasm code, ready for execution.
	#[pallet::storage]
	pub(crate) type InstrumentedCode<T: Config> =
		StorageMap<_, Twox64Concat, CosmwasmCodeId, ContractInstrumentedCodeOf<T>>;

	/// Monotonic counter incremented on code creation.
	#[allow(clippy::disallowed_types)]
	#[pallet::storage]
	pub(crate) type CurrentCodeId<T: Config> =
		StorageValue<_, CosmwasmCodeId, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	/// A mapping between an original code hash and its metadata.
	#[pallet::storage]
	pub(crate) type CodeIdToInfo<T: Config> =
		StorageMap<_, Twox64Concat, CosmwasmCodeId, CodeInfoOf<T>>;

	/// A mapping between a code hash and it's unique ID.
	#[pallet::storage]
	pub(crate) type CodeHashToId<T: Config> = StorageMap<_, Identity, [u8; 32], CosmwasmCodeId>;

	/// This is a **monotonic** counter incremented on contract instantiation.
	/// The purpose of this nonce is just to make sure that contract trie are unique.
	#[allow(clippy::disallowed_types)]
	#[pallet::storage]
	pub(crate) type CurrentNonce<T: Config> =
		StorageValue<_, u64, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	/// A mapping between a contract's account id and it's metadata.
	#[pallet::storage]
	pub(crate) type ContractToInfo<T: Config> =
		StorageMap<_, Identity, AccountIdOf<T>, ContractInfoOf<T>>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub contracts: sp_std::vec::Vec<(T::AccountIdExtended, ContractCodeOf<T>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { contracts: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for (who, code) in self.contracts.clone() {
				<Pallet<T>>::do_upload(&who, code).expect("contracts in genesis are valid")
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Upload a CosmWasm contract.
		/// The function will ensure that the wasm module is well formed and that it fits the
		/// according limits. The module exports are going to be checked against the expected
		/// CosmWasm export signatures.
		///
		/// * Emits an `Uploaded` event on success.
		///
		/// # Arguments
		///
		/// - `origin` the original dispatching the extrinsic.
		/// - `code` the actual wasm code.
		#[transactional]
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::upload(code.len() as u32))]
		pub fn upload(origin: OriginFor<T>, code: ContractCodeOf<T>) -> DispatchResult {
			T::UploadWasmOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;
			Self::do_upload(&who, code)
		}

		/// Instantiate a previously uploaded code resulting in a new contract being generated.
		///
		/// * Emits an `Instantiated` event on success.
		/// * Emits an `Executed` event.
		/// * Possibly emit `Emitted` events.
		///
		/// # Arguments
		///
		/// * `origin` the origin dispatching the extrinsic.
		/// * `code_identifier` the unique code id generated when the code has been uploaded via
		///   [`upload`].
		/// * `salt` the salt, usually used to instantiate the same contract multiple times.
		/// * `funds` the assets transferred to the contract prior to calling it's `instantiate`
		///   export.
		/// * `gas` the maximum gas to use, the remaining is refunded at the end of the transaction.
		#[pallet::call_index(1)]
		#[transactional]
		// must depend on message too
		#[pallet::weight(T::WeightInfo::instantiate(funds.len() as u32).saturating_add(Weight::from_parts(*gas, 0)))]
		pub fn instantiate(
			origin: OriginFor<T>,
			code_identifier: CodeIdentifier,
			salt: ContractSaltOf<T>,
			admin: Option<AccountIdOf<T>>,
			label: ContractLabelOf<T>,
			funds: FundsOf<T>,
			gas: u64,
			message: ContractMessageOf<T>,
		) -> DispatchResultWithPostInfo {
			T::ExecuteWasmOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;
			let mut shared = Self::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
			let initial_gas = T::WeightInfo::instantiate(funds.len() as u32)
				.saturating_add(Weight::from_parts(gas, 0))
				.ref_time();
			let outcome = Self::do_instantiate(
				&mut shared,
				who,
				code_identifier,
				salt,
				admin,
				label,
				funds,
				message,
			);
			Self::refund_gas(outcome, initial_gas, shared.gas.remaining())
		}

		/// Execute a previously instantiated contract.
		///
		/// * Emits an `Executed` event.
		/// * Possibly emit `Emitted` events.
		///
		/// # Arguments
		///
		/// * `origin` the origin dispatching the extrinsic.
		/// * `code_id` the unique code id generated when the code has been uploaded via [`upload`].
		/// * `salt` the salt, usually used to instantiate the same contract multiple times.
		/// * `funds` the assets transferred to the contract prior to calling it's `instantiate`
		///   export.
		/// * `gas` the maximum gas to use, the remaining is refunded at the end of the transaction.
		#[pallet::call_index(2)]
		#[transactional]
		#[pallet::weight(T::WeightInfo::execute(funds.len() as u32).saturating_add(Weight::from_parts(*gas, 0)))]
		pub fn execute(
			origin: OriginFor<T>,
			contract: AccountIdOf<T>,
			funds: FundsOf<T>,
			gas: u64,
			message: ContractMessageOf<T>,
		) -> DispatchResultWithPostInfo {
			T::ExecuteWasmOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;
			let mut shared = Self::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
			let initial_gas = T::WeightInfo::execute(funds.len() as u32)
				.saturating_add(Weight::from_parts(gas, 0))
				.ref_time();
			let outcome = Self::do_execute(&mut shared, who, contract, funds, message);
			Self::refund_gas(outcome, initial_gas, shared.gas.remaining())
		}

		/// Migrate a previously instantiated contract.
		///
		/// * Emits a `Migrated` event on success.
		/// * Emits an `Executed` event.
		/// * Possibly emit `Emitted` events.
		///
		/// # Arguments
		///
		/// * `origin` the origin dispatching the extrinsic.
		/// * `contract` the address of the contract that we want to migrate
		/// * `new_code_identifier` the code identifier that we want to switch to.
		/// * `gas` the maximum gas to use, the remaining is refunded at the end of the transaction.
		/// * `message` MigrateMsg, that will be passed to the contract.
		#[pallet::call_index(3)]
		#[transactional]
		#[pallet::weight(T::WeightInfo::migrate().saturating_add(Weight::from_parts(*gas, 0)))]
		pub fn migrate(
			origin: OriginFor<T>,
			contract: AccountIdOf<T>,
			new_code_identifier: CodeIdentifier,
			gas: u64,
			message: ContractMessageOf<T>,
		) -> DispatchResultWithPostInfo {
			T::ExecuteWasmOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;
			let mut shared = Self::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
			let initial_gas =
				T::WeightInfo::migrate().saturating_add(Weight::from_parts(gas, 0)).ref_time();
			let outcome =
				Self::do_migrate(&mut shared, who, contract, new_code_identifier, message);
			Self::refund_gas(outcome, initial_gas, shared.gas.remaining())
		}

		/// Update the admin of a contract.
		///
		/// * Emits a `AdminUpdated` event on success.
		///
		/// # Arguments
		///
		/// * `origin` the origin dispatching the extrinsic.
		/// * `contract` the address of the contract that we want to migrate.
		/// * `new_admin` new admin of the contract that we want to update to.
		/// * `gas` the maximum gas to use, the remaining is refunded at the end of the transaction.
		#[pallet::call_index(4)]
		#[transactional]
		#[pallet::weight(T::WeightInfo::update_admin().saturating_add(Weight::from_parts(*gas, 0)))]
		pub fn update_admin(
			origin: OriginFor<T>,
			contract: AccountIdOf<T>,
			new_admin: Option<AccountIdOf<T>>,
			gas: u64,
		) -> DispatchResultWithPostInfo {
			T::ExecuteWasmOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;
			let mut shared = Self::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
			let initial_gas = T::WeightInfo::update_admin()
				.saturating_add(Weight::from_parts(gas, 0))
				.ref_time();
			let outcome =
				Self::do_update_admin(&mut shared, who, contract.clone(), new_admin.clone());
			Self::deposit_event(Event::<T>::AdminUpdated { contract, new_admin });
			Self::refund_gas(outcome, initial_gas, shared.gas.remaining())
		}
	}
}

/// Query cosmwasm contracts
///
/// * `contract` the address of contract to query.
/// * `gas` the maximum gas to use, the remaining is refunded at the end of the transaction.
/// * `query_request` the binary query, which should be deserializable to `QueryRequest`.
pub fn query<T: Config>(
	contract: AccountIdOf<T>,
	gas: u64,
	query_request: Vec<u8>,
) -> Result<QueryResponse, CosmwasmVMError<T>> {
	let mut shared = Pallet::<T>::do_create_vm_shared(gas, InitialStorageMutability::ReadOnly);
	let query_request = serde_json::from_slice(&query_request)
		.map_err(|e| CosmwasmVMError::<T>::Rpc(e.to_string()))?;
	Pallet::<T>::sub_level_dispatch(
		&mut shared,
		contract.clone(),
		contract,
		Default::default(),
		|mut vm| {
			cosmwasm_system_query(&mut vm, query_request)?
				.into_result()
				.map_err(|e| CosmwasmVMError::<T>::Rpc(format!("{:?}", e)))?
				.into_result()
				.map_err(|e| CosmwasmVMError::<T>::Rpc(e))
		},
	)
}

#[allow(clippy::too_many_arguments)]
pub fn instantiate<T: Config>(
	instantiator: AccountIdOf<T>,
	code_id: CosmwasmCodeId,
	salt: Vec<u8>,
	admin: Option<AccountIdOf<T>>,
	label: Vec<u8>,
	funds: BTreeMap<AssetIdOf<T>, (BalanceOf<T>, KeepAlive)>,
	gas: u64,
	message: Vec<u8>,
) -> Result<AccountIdOf<T>, CosmwasmVMError<T>> {
	let salt: ContractSaltOf<T> = salt
		.try_into()
		.map_err(|_| CosmwasmVMError::<T>::Rpc(String::from("'salt' is too large")))?;
	let label: ContractLabelOf<T> = label
		.try_into()
		.map_err(|_| CosmwasmVMError::<T>::Rpc(String::from("'label' is too large")))?;
	let funds: FundsOf<T> = funds
		.try_into()
		.map_err(|_| CosmwasmVMError::<T>::Rpc(String::from("'funds' is too large")))?;
	let message: ContractMessageOf<T> = message
		.try_into()
		.map_err(|_| CosmwasmVMError::<T>::Rpc(String::from("'message' is too large")))?;
	let mut shared = Pallet::<T>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
	setup_instantiate_call(instantiator, code_id, &salt, admin, label)?.top_level_call(
		&mut shared,
		funds,
		message,
	)
}

pub fn execute<T: Config>(
	executor: AccountIdOf<T>,
	contract: AccountIdOf<T>,
	funds: FundsOf<T>,
	gas: u64,
	message: ContractMessageOf<T>,
) -> Result<(), CosmwasmVMError<T>> {
	let mut shared = Pallet::<T>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
	setup_execute_call(executor, contract)?.top_level_call(&mut shared, funds, message)
}

impl<T: Config> VMPallet for T {
	type VmError = CosmwasmVMError<T>;
}

impl<T: Config> Pallet<T> {
	/// Create the shared VM state. Including readonly stack, VM depth, gas metering limits and
	/// code cache.
	///
	/// This state is shared across all VMs (all contracts loaded within a single call) and is
	/// used to optimize some operations as well as track shared state (readonly storage while
	/// doing a `query` etc...)
	pub(crate) fn do_create_vm_shared(
		gas: u64,
		storage_mutability: InitialStorageMutability,
	) -> CosmwasmVMShared {
		CosmwasmVMShared {
			storage_readonly_depth: match storage_mutability {
				InitialStorageMutability::ReadOnly => 1,
				InitialStorageMutability::ReadWrite => 0,
			},
			depth: 0,
			gas: Gas::new(T::MAX_FRAMES, gas),
			cache: CosmwasmVMCache { code: Default::default() },
		}
	}

	/// Wrapper around [`Pallet::<T>::cosmwasm_call`] for extrinsics.
	/// It's purpose is converting the input and emit events based on the result.
	pub(crate) fn top_level_dispatch<F>(
		shared: &mut CosmwasmVMShared,
		entrypoint: EntryPoint,
		sender: AccountIdOf<T>,
		contract: AccountIdOf<T>,
		funds: FundsOf<T>,
		call: F,
	) -> Result<(), CosmwasmVMError<T>>
	where
		F: for<'x> FnOnce(
			OwnedWasmiVM<DefaultCosmwasmVM<'x, T>>,
		) -> Result<
			(Option<CosmwasmBinary>, Vec<CosmwasmEvent>),
			CosmwasmVMError<T>,
		>,
	{
		let cosmwasm_funds = funds
			.into_iter()
			.map(|(asset, (amount, _))| Self::native_asset_to_cosmwasm_asset(asset, amount))
			.collect::<Vec<_>>();

		Self::sub_level_dispatch(shared, sender, contract.clone(), cosmwasm_funds, call).map(
			|(data, events)| {
				for CosmwasmEvent { ty, attributes, .. } in events {
					Self::deposit_event(Event::<T>::Emitted {
						contract: contract.clone(),
						ty: ty.into(),
						attributes: attributes
							.into_iter()
							.map(|CosmwasmEventAttribute { key, value }| (key.into(), value.into()))
							.collect::<Vec<_>>(),
					});
				}
				Self::deposit_event(Event::<T>::Executed {
					contract,
					entrypoint,
					data: data.map(Into::into),
				});
			},
		)
	}

	/// Low-level cosmwasm call over the VM. Transfers the `funds` before calling the callback.
	pub(crate) fn sub_level_dispatch<F, R>(
		shared: &mut CosmwasmVMShared,
		sender: AccountIdOf<T>,
		contract: AccountIdOf<T>,
		funds: Vec<Coin>,
		call: F,
	) -> Result<R, CosmwasmVMError<T>>
	where
		F: for<'x> FnOnce(OwnedWasmiVM<DefaultCosmwasmVM<'x, T>>) -> Result<R, CosmwasmVMError<T>>,
	{
		let vm = Self::cosmwasm_new_vm(shared, sender, contract, funds)?;
		call(vm)
	}

	/// Refund the remaining gas regardless of a contract outcome.
	pub(crate) fn refund_gas(
		outcome: Result<(), CosmwasmVMError<T>>,
		initial_gas: u64,
		remaining_gas: u64,
	) -> DispatchResultWithPostInfo {
		log::debug!(target: "runtime::contracts", "outcome: {:?}", outcome);
		let post_info = PostDispatchInfo {
			actual_weight: Some(Weight::from_parts(initial_gas.saturating_sub(remaining_gas), 0)),
			pays_fee: Pays::Yes,
		};
		match outcome {
			Ok(()) => Ok(post_info),
			Err(error) => {
				log::info!(target: "runtime::contracts", "executing contract error with {}", &error);
				let error = match error {
					CosmwasmVMError::Pallet(e) => e,
					CosmwasmVMError::InvalidGasCheckpoint => Error::<T>::OutOfGas,
					CosmwasmVMError::OutOfGas => Error::<T>::OutOfGas,
					CosmwasmVMError::Interpreter(_) => Error::<T>::Interpreter,
					CosmwasmVMError::VirtualMachine(_) => Error::<T>::VirtualMachine,
					CosmwasmVMError::AccountConvert => Error::<T>::AccountConversionFailure,
					CosmwasmVMError::Aborted(_) => Error::<T>::Aborted,
					CosmwasmVMError::ReadOnlyViolation => Error::<T>::ReadOnlyViolation,
					CosmwasmVMError::Unsupported => Error::<T>::Unsupported,
					CosmwasmVMError::ExecuteDeserialize => Error::<T>::ExecuteDeserialize,
					CosmwasmVMError::ContractNotFound => Error::<T>::ContractNotFound,
					CosmwasmVMError::Rpc(_) => Error::<T>::Rpc,
					CosmwasmVMError::Ibc(_) => Error::<T>::Ibc,
					CosmwasmVMError::SubstrateDispatch(_) => Error::<T>::SubstrateDispatch,
					CosmwasmVMError::AssetConversion => Error::<T>::AssetConversion,
					CosmwasmVMError::QuerySerialize => Error::<T>::FailedToSerialize,
					CosmwasmVMError::Precompile => Error::<T>::Precompile,
					CosmwasmVMError::QueryDeserialize => Error::<T>::QueryDeserialize,
					CosmwasmVMError::ExecuteSerialize => Error::<T>::ExecuteSerialize,
					CosmwasmVMError::NotImplemented => Error::<T>::NotAuthorized,
					CosmwasmVMError::Xcm(_) => Error::<T>::Xcm,
				};
				Err(DispatchErrorWithPostInfo { error: error.into(), post_info })
			},
		}
	}

	/// Set the contract info and update the state accordingly.
	///
	/// This function will update the state if the `code_id` is changing:
	/// 1. Refcount of the new `code_id` is incremented.
	/// 2. Refcount of the old `code_id` is decremented.
	/// 3. Delete every entry related to old `code_id` if the refcount is 0. And unreserve the
	///    bonded funds.
	pub(crate) fn do_set_contract_meta(
		contract: &AccountIdOf<T>,
		code_id: CosmwasmCodeId,
		admin: Option<AccountIdOf<T>>,
		label: String,
	) -> Result<(), Error<T>> {
		let mut info = Self::contract_info(contract)?;

		if info.code_id != code_id {
			// Increase the refcount of `new_code_id`.
			CodeIdToInfo::<T>::try_mutate_exists(code_id, |entry| -> Result<(), Error<T>> {
				let code_info = entry.as_mut().ok_or(Error::<T>::CodeNotFound)?;
				code_info.refcount =
					code_info.refcount.checked_add(1).ok_or(Error::<T>::RefcountOverflow)?;
				Ok(())
			})?;

			// Modify the existing `code_id`'s states and unreserve the bonded funds.
			CodeIdToInfo::<T>::try_mutate_exists(info.code_id, |entry| -> Result<(), Error<T>> {
				// Decrement the refcount
				let code_info = entry.as_mut().ok_or(Error::<T>::CodeNotFound)?;
				code_info.refcount =
					code_info.refcount.checked_sub(1).ok_or(Error::<T>::RefcountOverflow)?;
				if code_info.refcount == 0 {
					// Unreserve the bonded funds for this code
					let code = PristineCode::<T>::try_get(info.code_id)
						.map_err(|_| Error::<T>::CodeNotFound)?;
					let deposit = code.len().saturating_mul(T::CodeStorageByteDeposit::get() as _);
					let _ = T::NativeAsset::unreserve(&code_info.creator, deposit.saturated_into());
					PristineCode::<T>::remove(info.code_id);
					InstrumentedCode::<T>::remove(info.code_id);
					CodeHashToId::<T>::remove(code_info.pristine_code_hash);
					// Code is unused after this point, so it can be removed
					*entry = None;
				}
				Ok(())
			})?;
		}

		info.code_id = code_id;
		info.admin = admin;
		info.label = label
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(|_| crate::Error::<T>::LabelTooBig)?;

		Self::set_contract_info(contract, info);
		Ok(())
	}

	/// Ensure that a contract exists.
	pub(crate) fn contract_exists(contract: &AccountIdOf<T>) -> Result<(), Error<T>> {
		match T::PalletHook::info(contract) {
			Some(_) => Ok(()),
			None if ContractToInfo::<T>::contains_key(contract) => Ok(()),
			_ => Err(Error::<T>::ContractNotFound),
		}
	}

	/// Validate a wasm module against the defined limitations.
	///
	/// Notably
	///
	/// - whether it is well formed and follow the spec.
	/// - memory limit.
	/// - table size limit.
	/// - global variables limit.
	/// - function parameters limit.
	/// - branch table limit.
	/// - ensuring no floating type are used.
	/// - ensuring mandatory exports are present and that their signature matches.
	/// - ensuring that forbidden imports are not imported.
	pub(crate) fn do_validate_code(module: &parity_wasm::elements::Module) -> Result<(), Error<T>> {
		let validation: Result<(), ValidationError> = (|| {
			let _ = CodeValidation::new(module)
				.validate_module::<PlainValidator>(())?
				.validate_memory_limit()?
				.validate_table_size_limit(T::CodeTableSizeLimit::get())?
				.validate_global_variable_limit(T::CodeGlobalVariableLimit::get())?
				.validate_parameter_limit(T::CodeParameterLimit::get())?
				.validate_br_table_size_limit(T::CodeBranchTableSizeLimit::get())?
				.validate_no_floating_types()?
				.validate_exports(Version1x::EXPORTS)?
				// env.gas is banned as injected by instrumentation
				.validate_imports(&[(Version1x::ENV_MODULE, Version1x::ENV_GAS)])?;
			Ok(())
		})();
		validation.map_err(|e| {
			log::debug!(target: "runtime::contracts", "do_validate_code: {:#?}", e);
			Error::<T>::CodeValidation
		})
	}

	/// Instrument the wasm module by injecting stack height limitation along gas metering.
	pub(crate) fn do_instrument_code(
		module: parity_wasm::elements::Module,
	) -> Result<ContractInstrumentedCodeOf<T>, Error<T>> {
		Self::do_validate_code(&module)?;
		let instrumented_module = gas_and_stack_instrumentation(
			module,
			Version1x::ENV_MODULE,
			T::CodeStackLimit::get(),
			&T::WasmCostRules::get(),
		);
		instrumented_module
			.map_err(|e| {
				log::debug!(target: "runtime::contracts", "do_instrument_code: {:#?}", e);
				Error::<T>::CodeInstrumentation
			})?
			.into_bytes()
			.map_err(|e| {
				log::debug!(target: "runtime::contracts", "do_instrument_code: {:#?}", e);
				Error::<T>::CodeEncoding
			})?
			.try_into()
			.map_err(|_| Error::<T>::InstrumentedCodeIsTooBig)
	}

	/// Check whether the instrumented code of a contract is up to date.
	/// If the instrumentation is outdated, re-instrument the pristine code.
	pub(crate) fn do_check_for_reinstrumentation(
		code_id: CosmwasmCodeId,
	) -> Result<Option<Vec<u8>>, Error<T>> {
		CodeIdToInfo::<T>::try_mutate(code_id, |entry| {
			let code_info = entry.as_mut().ok_or(Error::<T>::CodeNotFound)?;
			if code_info.instrumentation_version != INSTRUMENTATION_VERSION {
				log::debug!(target: "runtime::contracts", "do_check_for_reinstrumentation: required");
				let code = PristineCode::<T>::get(code_id).ok_or(Error::<T>::CodeNotFound)?;
				let module = Self::do_load_module(&code)?;
				let instrumented_code = Self::do_instrument_code(module)?;
				InstrumentedCode::<T>::insert(code_id, instrumented_code.clone());
				code_info.instrumentation_version = INSTRUMENTATION_VERSION;
				Ok(Some(instrumented_code.into()))
			} else {
				log::debug!(target: "runtime::contracts", "do_check_for_reinstrumentation: not required");
				Ok(None)
			}
		})
	}

	pub(crate) fn do_load_module(
		code: &ContractCodeOf<T>,
	) -> Result<parity_wasm::elements::Module, Error<T>> {
		parity_wasm::elements::Module::from_bytes(code).map_err(|e| {
			log::debug!(target: "runtime::contracts", "do_load_module: {:#?}", e);
			Error::<T>::CodeDecoding
		})
	}

	/// Extract the current environment from the pallet.
	pub(crate) fn cosmwasm_env(cosmwasm_contract_address: CosmwasmAccount<T>) -> Env {
		Env {
			block: Self::block_env(),
			transaction: frame_system::Pallet::<T>::extrinsic_index()
				.map(|index| TransactionInfo { index }),
			contract: CosmwasmContractInfo {
				address: Addr::unchecked(String::from(cosmwasm_contract_address)),
			},
		}
	}

	pub(crate) fn do_upload(who: &AccountIdOf<T>, code: ContractCodeOf<T>) -> DispatchResult {
		let code_hash = sp_io::hashing::sha2_256(&code);
		ensure!(!CodeHashToId::<T>::contains_key(code_hash), Error::<T>::CodeAlreadyExists);
		let deposit = code.len().saturating_mul(T::CodeStorageByteDeposit::get() as _);
		T::NativeAsset::reserve(who, deposit.saturated_into())
			.map_err(|_| Error::<T>::NotEnoughFundsForUpload)?;
		let module = Self::do_load_module(&code)?;
		let ibc_capable = Self::do_check_ibc_capability(&module);
		let instrumented_code = Self::do_instrument_code(module)?;
		let code_id = CurrentCodeId::<T>::increment()?;
		CodeHashToId::<T>::insert(code_hash, code_id);
		PristineCode::<T>::insert(code_id, code);
		InstrumentedCode::<T>::insert(code_id, instrumented_code);
		CodeIdToInfo::<T>::insert(
			code_id,
			CodeInfoOf::<T> {
				creator: who.clone(),
				pristine_code_hash: code_hash,
				instrumentation_version: INSTRUMENTATION_VERSION,
				ibc_capable,
				refcount: 0,
			},
		);
		Self::deposit_event(Event::<T>::Uploaded { code_hash, code_id });
		Ok(())
	}

	#[allow(clippy::too_many_arguments)]
	fn do_instantiate(
		shared: &mut CosmwasmVMShared,
		who: AccountIdOf<T>,
		code_identifier: CodeIdentifier,
		salt: ContractSaltOf<T>,
		admin: Option<AccountIdOf<T>>,
		label: ContractLabelOf<T>,
		funds: FundsOf<T>,
		message: ContractMessageOf<T>,
	) -> Result<(), CosmwasmVMError<T>> {
		let code_id = match code_identifier {
			CodeIdentifier::CodeId(code_id) => code_id,
			CodeIdentifier::CodeHash(code_hash) =>
				CodeHashToId::<T>::try_get(code_hash).map_err(|_| Error::<T>::CodeNotFound)?,
		};
		setup_instantiate_call(who, code_id, &salt, admin, label)?
			.top_level_call(shared, funds, message)
			.map(|_| ())
	}

	fn do_execute(
		shared: &mut CosmwasmVMShared,
		who: AccountIdOf<T>,
		contract: AccountIdOf<T>,
		funds: FundsOf<T>,
		message: ContractMessageOf<T>,
	) -> Result<(), CosmwasmVMError<T>> {
		setup_execute_call(who, contract)?.top_level_call(shared, funds, message)
	}

	fn do_migrate(
		shared: &mut CosmwasmVMShared,
		who: AccountIdOf<T>,
		contract: AccountIdOf<T>,
		new_code_identifier: CodeIdentifier,
		message: ContractMessageOf<T>,
	) -> Result<(), CosmwasmVMError<T>> {
		let new_code_id = match new_code_identifier {
			CodeIdentifier::CodeId(code_id) => code_id,
			CodeIdentifier::CodeHash(code_hash) =>
				CodeHashToId::<T>::try_get(code_hash).map_err(|_| Error::<T>::CodeNotFound)?,
		};

		setup_migrate_call(shared, who, contract, new_code_id, true)?.top_level_call(
			shared,
			Default::default(),
			message,
		)
	}

	fn do_update_admin(
		shared: &mut CosmwasmVMShared,
		who: AccountIdOf<T>,
		contract: AccountIdOf<T>,
		new_admin: Option<AccountIdOf<T>>,
	) -> Result<(), CosmwasmVMError<T>> {
		Self::sub_level_dispatch(
			shared,
			who.clone(),
			contract.clone(),
			Default::default(),
			|mut vm| {
				cosmwasm_vm::system::update_admin(
					&mut vm,
					&Addr::unchecked(Self::account_to_cosmwasm_addr(who)),
					CosmwasmAccount::new(contract),
					new_admin.map(CosmwasmAccount::new),
				)
				.map_err(Into::into)
			},
		)
	}

	fn block_env() -> BlockInfo {
		BlockInfo {
			height: frame_system::Pallet::<T>::block_number().saturated_into(),
			time: Timestamp::from_seconds(T::UnixTime::now().as_secs()),
			chain_id: T::ChainId::get().into(),
		}
	}

	/// Create a new CosmWasm VM. One instance is created per contract but all of them share the
	/// same [`CosmwasmVMShared<'a, T>`] structure. If the [`contract`] is a PalletHook, the
	/// CosmwasmVM with a [`ContractBackend::Pallet`] is returned instead of
	/// [`ContractBackend::CosmWasm`]
	///
	/// Prior to instantiating the VM. The depth is checked against [`T::MaxFrames`] and the
	/// contract code is loaded from the shared state if cached. If the code is not in cache, we
	/// check whether reinstrumentation is required and cache the code.
	pub(crate) fn cosmwasm_new_vm(
		shared: &mut CosmwasmVMShared,
		sender: AccountIdOf<T>,
		contract: AccountIdOf<T>,
		funds: Vec<Coin>,
	) -> Result<OwnedWasmiVM<DefaultCosmwasmVM<T>>, CosmwasmVMError<T>> {
		shared.depth = shared.depth.checked_add(1).ok_or(Error::<T>::VMDepthOverflow)?;
		ensure!(shared.depth <= T::MAX_FRAMES, Error::<T>::StackOverflow);

		let contract_address: CosmwasmAccount<T> = CosmwasmAccount::new(contract.clone());
		let env = Self::cosmwasm_env(contract_address.clone());
		let cosmwasm_message_info = {
			let cosmwasm_sender_address: CosmwasmAccount<T> = CosmwasmAccount::new(sender);
			MessageInfo { sender: Addr::unchecked(String::from(cosmwasm_sender_address)), funds }
		};

		// If the [`contract`] is actually a pallet that is exposed as a cosmwasm contract,
		// then we use the pallet instead of setting up a ContractBackend::CosmWasm
		if let Some(precompiled_info) = T::PalletHook::info(&contract) {
			let engine = wasmi::Engine::default();
			let store = wasmi::Store::new(
				&engine,
				CosmwasmVM {
					cosmwasm_env: env,
					cosmwasm_message_info,
					contract_address,
					contract_info: precompiled_info.contract,
					shared,
					iterators: Default::default(),
					contract_runtime: ContractBackend::Pallet { call_depth_mut: 0 },
				},
			);

			return Ok(OwnedWasmiVM::new(store))
		}

		// Else, the contract is not a pallet. We continue with the normal wasmi vm creation
		// process:
		let info = Self::contract_info(&contract)?;
		let code = Self::get_code_from_cache(shared, info.code_id)?.clone();

		log::debug!(target: "runtime::contracts", "env  : {:#?}", env);
		log::debug!(target: "runtime::contracts", "info : {:#?}", cosmwasm_message_info);

		let vm = CosmwasmVM {
			cosmwasm_env: env,
			cosmwasm_message_info,
			contract_address,
			contract_info: info,
			shared,
			iterators: Default::default(),
			contract_runtime: ContractBackend::CosmWasm {
				executing_module: None,
				call_depth_mut: 0,
			},
		};

		let wasmi_vm = new_wasmi_vm(code.as_slice(), vm).map_err(|_| Error::<T>::VmCreation)?;
		Ok(wasmi_vm)
	}

	/// Get the code for `code_id` from the `shared_vm`'s code cache. If the code isn't in the cache
	/// yet, then we reinstrument the code if needed and insert it into the cache.
	///
	/// Note that it may look like we could have an invalid cache if reinstrumentation just
	/// happend, but the `shared.cache.code` is only shared for a single tx, so it is
	/// impossible for non-reinstrumented code to be in the `shared.code.cache`
	pub(crate) fn get_code_from_cache(
		shared_vm: &mut CosmwasmVMShared,
		code_id: CosmwasmCodeId,
	) -> Result<&mut Vec<u8>, CosmwasmVMError<T>> {
		log::debug!(target: "runtime::contracts", "Getting code for: {:?} from shared cache", code_id);
		match shared_vm.cache.code.entry(code_id) {
			Entry::Vacant(v) => {
				log::debug!(target: "runtime::contracts", "Code cache miss: {:?}", code_id);
				// Reinstrument the code if needed
				let code = Self::do_check_for_reinstrumentation(code_id)?.map_or_else(
					|| {
						// map_or_else instead of map_or to prevent the code from getting loaded
						Ok(InstrumentedCode::<T>::get(code_id)
							.ok_or(Error::<T>::CodeNotFound)?
							.into_inner())
					},
					Ok::<_, Error<T>>,
				)?;
				Ok(v.insert(code))
			},
			Entry::Occupied(o) => {
				log::debug!(target: "runtime::contracts", "Code cache hit: {:?}", code_id);
				Ok(o.into_mut())
			},
		}
	}

	/// Abstract function to operate on a contract child trie entry.
	pub(crate) fn with_db_entry<R>(
		trie_id: &ContractTrieIdOf<T>,
		key: &[u8],
		f: impl FnOnce(ChildInfo, Vec<u8>) -> R,
	) -> R {
		let child_trie = Self::contract_child_trie(trie_id.as_ref());
		f(child_trie, Blake2_128Concat::hash(key))
	}

	/// Compute the gas required to read the given entry.
	///
	/// Equation: len(entry(trie, key)) x [`T::ContractStorageByteReadPrice`]
	pub(crate) fn do_db_read_gas(trie_id: &ContractTrieIdOf<T>, key: &[u8]) -> u64 {
		Self::with_db_entry(trie_id, key, |child_trie, entry| {
			let bytes_to_read = storage::child::len(&child_trie, &entry).unwrap_or(0);
			u64::from(bytes_to_read).saturating_mul(T::ContractStorageByteReadPrice::get().into())
		})
	}

	/// Read an entry from the executing contract storage, charging the according gas prior to
	/// actually reading the entry.
	pub(crate) fn do_db_read(
		vm: &mut DefaultCosmwasmVM<T>,
		key: &[u8],
	) -> Result<Option<Vec<u8>>, CosmwasmVMError<T>> {
		let price = Self::do_db_read_gas(&vm.contract_info.trie_id, key);
		vm.charge_raw(price)?;
		Ok(Self::with_db_entry(&vm.contract_info.trie_id, key, |child_trie, entry| {
			storage::child::get_raw(&child_trie, &entry)
		}))
	}

	/// Read an entry from an arbitrary contract, charging the according gas prior to actually
	/// reading the entry.
	pub(crate) fn do_db_read_other_contract(
		vm: &mut DefaultCosmwasmVM<T>,
		trie_id: &ContractTrieIdOf<T>,
		key: &[u8],
	) -> Result<Option<Vec<u8>>, CosmwasmVMError<T>> {
		let price = Self::do_db_read_gas(trie_id, key);
		vm.charge_raw(price)?;
		Ok(Self::with_db_entry(trie_id, key, |child_trie, entry| {
			storage::child::get_raw(&child_trie, &entry)
		}))
	}

	/// Compute the gas required to overwrite the given entry.
	///
	/// Equation: len(entry(trie, key)) - len(value)  x [`T::ContractStorageByteWritePrice`]
	/// With minus saturating.
	pub(crate) fn do_db_write_gas(trie_id: &ContractTrieIdOf<T>, key: &[u8], value: &[u8]) -> u64 {
		Self::with_db_entry(trie_id, key, |child_trie, entry| {
			let bytes_to_write = match storage::child::len(&child_trie, &entry) {
				Some(current_len) => (value.len() as u32).saturating_sub(current_len),
				None => value.len() as u32,
			};
			u64::from(bytes_to_write).saturating_mul(T::ContractStorageByteWritePrice::get().into())
		})
	}

	/// Write an entry from the executing contract, charging the according gas prior to actually
	/// writing the entry.
	pub(crate) fn do_db_write(
		vm: &mut DefaultCosmwasmVM<T>,
		key: &[u8],
		value: &[u8],
	) -> Result<(), CosmwasmVMError<T>> {
		let price = Self::do_db_write_gas(&vm.contract_info.trie_id, key, value);
		vm.charge_raw(price)?;
		Self::with_db_entry(&vm.contract_info.trie_id, key, |child_trie, entry| {
			storage::child::put_raw(&child_trie, &entry, value)
		});
		Ok(())
	}

	/// Create an empty iterator.
	pub(crate) fn do_db_scan(vm: &mut DefaultCosmwasmVM<T>) -> Result<u32, CosmwasmVMError<T>> {
		let iterator_id = vm.iterators.len() as u32;
		// let child_info = Self::contract_child_trie(vm.contract_info.trie_id.as_ref());
		vm.iterators.insert(iterator_id, Vec::new());
		Ok(iterator_id)
	}

	/// Return the next (reversed-key, value) pair and save the state. If the next key
	/// is `None`, the iterator is removed from the storage.
	pub(crate) fn do_db_next(
		vm: &mut DefaultCosmwasmVM<T>,
		iterator_id: u32,
	) -> Result<Option<(Vec<u8>, Vec<u8>)>, CosmwasmVMError<T>> {
		// Get the next value from the vm's iterators, based on the iterator id.
		// Error if the iterator with id [`iterator_id`]
		let child_info = Self::contract_child_trie(vm.contract_info.trie_id.as_ref());
		let key_entry = vm.iterators.get_mut(&iterator_id).ok_or(Error::<T>::IteratorNotFound)?;
		match sp_io::default_child_storage::next_key(child_info.storage_key(), key_entry) {
			Some(key) => {
				let reversed_key = Blake2_128Concat::reverse(&key).to_vec();
				*key_entry = key;
				Ok(Some((
					reversed_key.clone(),
					Self::do_db_read(vm, reversed_key.as_slice())?
						.ok_or(Error::<T>::IteratorValueNotFound)?,
				)))
			},
			None => Ok(None),
		}
	}

	/// Remove an entry from the executing contract, no gas is charged for this operation.
	pub(crate) fn do_db_remove(vm: &mut DefaultCosmwasmVM<T>, key: &[u8]) {
		let trie_id = &vm.contract_info.trie_id;
		Self::with_db_entry(trie_id, key, |child_trie, entry| {
			storage::child::kill(&child_trie, &entry)
		})
	}

	pub(crate) fn do_running_contract_meta(
		vm: &mut DefaultCosmwasmVM<T>,
	) -> CosmwasmContractMeta<CosmwasmAccount<T>> {
		CosmwasmContractMeta {
			code_id: vm.contract_info.code_id,
			admin: vm.contract_info.admin.clone().map(CosmwasmAccount::new),
			label: String::from_utf8_lossy(&vm.contract_info.label).into(),
		}
	}

	/// Retrieve an account balance.
	pub(crate) fn do_balance(account: &AccountIdOf<T>, denom: String) -> Result<u128, Error<T>> {
		let asset = Self::cosmwasm_asset_to_native_asset(denom)?;
		Ok(T::Assets::balance(asset, account).into())
	}

	pub(crate) fn do_supply(denom: String) -> Result<u128, Error<T>> {
		let asset = Self::cosmwasm_asset_to_native_asset(denom)?;
		Ok(T::Assets::total_issuance(asset).into())
	}

	/// Execute a transfer of funds between two accounts.
	pub(crate) fn do_transfer(
		from: &AccountIdOf<T>,
		to: &AccountIdOf<T>,
		funds: &[Coin],
		preservation: Preservation,
	) -> Result<(), Error<T>> {
		for Coin { denom, amount } in funds {
			let asset = Self::cosmwasm_asset_to_native_asset(denom.clone())?;
			let amount = amount.u128().saturated_into();
			T::Assets::transfer(asset, from, to, amount, preservation)
				.map_err(|_| Error::<T>::TransferFailed)?;
		}
		Ok(())
	}
	pub(crate) fn do_continue_instantiate(
		vm: &mut DefaultCosmwasmVM<T>,
		CosmwasmContractMeta { code_id, admin, label }: CosmwasmContractMeta<CosmwasmAccount<T>>,
		funds: Vec<Coin>,
		salt: &[u8],
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_std::Event),
	) -> Result<Option<cosmwasm_std::Binary>, CosmwasmVMError<T>> {
		let label = label
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(|_| crate::Error::<T>::LabelTooBig)?;
		setup_instantiate_call(
			vm.contract_address.clone().into_inner(),
			code_id,
			salt,
			admin.map(|admin| admin.into_inner()),
			label,
		)?
		.sub_call(vm.shared, funds, message, event_handler)
	}

	pub(crate) fn do_continue_execute(
		vm: &mut DefaultCosmwasmVM<T>,
		contract: AccountIdOf<T>,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_std::Event),
	) -> Result<Option<cosmwasm_std::Binary>, CosmwasmVMError<T>> {
		setup_execute_call(vm.contract_address.clone().into_inner(), contract)?.sub_call(
			vm.shared,
			funds,
			message,
			event_handler,
		)
	}

	pub(crate) fn do_continue_reply(
		vm: &mut DefaultCosmwasmVM<T>,
		reply: cosmwasm_std::Reply,
		event_handler: &mut dyn FnMut(cosmwasm_std::Event),
	) -> Result<Option<cosmwasm_std::Binary>, CosmwasmVMError<T>> {
		setup_reply_call(
			vm.contract_address.clone().into_inner(),
			vm.contract_address.clone().into_inner(),
		)?
		.sub_call(
			vm.shared,
			Vec::default(),
			&serde_json::to_vec(&reply).map_err(|_| Error::<T>::FailedToSerialize)?,
			event_handler,
		)
	}

	pub(crate) fn do_continue_migrate(
		vm: &mut DefaultCosmwasmVM<T>,
		contract: AccountIdOf<T>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_std::Event),
	) -> Result<Option<cosmwasm_std::Binary>, CosmwasmVMError<T>> {
		let CosmwasmContractMeta { code_id, .. } = Self::do_running_contract_meta(vm);
		setup_migrate_call(
			vm.shared,
			vm.contract_address.clone().into_inner(),
			contract,
			code_id,
			false,
		)?
		.sub_call(vm.shared, Default::default(), message, event_handler)
	}

	pub(crate) fn do_query_contract_info(
		vm: &mut DefaultCosmwasmVM<T>,
		address: AccountIdOf<T>,
	) -> Result<ContractInfoResponse, CosmwasmVMError<T>> {
		// TODO: cache or at least check if its current contract and use `self.contract_info`
		let (contract_info, code_info, pinned) = match T::PalletHook::info(&address) {
			Some(precompiled_info) => (precompiled_info.contract, precompiled_info.code, true),
			None => {
				let contract_info = if &address == vm.contract_address.as_ref() {
					vm.contract_info.clone()
				} else {
					Pallet::<T>::contract_info(&address)?
				};
				let code_info = CodeIdToInfo::<T>::get(contract_info.code_id)
					.ok_or(Error::<T>::CodeNotFound)?;
				let pinned = vm.shared.cache.code.contains_key(&contract_info.code_id);
				(contract_info, code_info, pinned)
			},
		};
		let ibc_port = if code_info.ibc_capable {
			Some(Pallet::<T>::do_compute_ibc_contract_port(address))
		} else {
			None
		};

		let creator = CosmwasmAccount::<T>::new(contract_info.instantiator.clone());
		let mut contract_info_response = ContractInfoResponse::default();
		contract_info_response.code_id = contract_info.code_id;
		contract_info_response.creator = creator.into();
		contract_info_response.admin =
			contract_info.admin.map(|admin| CosmwasmAccount::<T>::new(admin).into());
		contract_info_response.pinned = pinned;
		contract_info_response.ibc_port = ibc_port;
		Ok(contract_info_response)
	}

	pub(crate) fn do_query_code_info(code_id: u64) -> Result<CodeInfoResponse, CosmwasmVMError<T>> {
		let code_info = CodeIdToInfo::<T>::get(code_id).ok_or(Error::<T>::CodeNotFound)?;
		let mut code_info_response = CodeInfoResponse::default();
		code_info_response.code_id = code_id;
		code_info_response.creator = Pallet::<T>::account_to_cosmwasm_addr(code_info.creator);
		code_info_response.checksum = code_info.pristine_code_hash.as_ref().into();
		Ok(code_info_response)
	}

	pub(crate) fn do_continue_query(
		vm: &mut DefaultCosmwasmVM<T>,
		contract: AccountIdOf<T>,
		message: &[u8],
	) -> Result<cosmwasm_vm::executor::QueryResult, CosmwasmVMError<T>> {
		let sender = vm.contract_address.clone().into_inner();
		vm.shared.push_readonly();
		let result = Pallet::<T>::sub_level_dispatch(
			vm.shared,
			sender,
			contract,
			Default::default(),
			|mut vm| match vm.0.as_context().data().contract_runtime {
				ContractBackend::CosmWasm { .. } =>
					cosmwasm_call::<QueryCall, OwnedWasmiVM<DefaultCosmwasmVM<T>>>(&mut vm, message),
				ContractBackend::Pallet { .. } =>
					T::PalletHook::query(&mut vm, message).map(Into::into),
			},
		);
		vm.shared.pop_readonly();
		result
	}

	pub(crate) fn do_query_raw(
		vm: &mut DefaultCosmwasmVM<T>,
		address: AccountIdOf<T>,
		key: &[u8],
	) -> Result<Option<Vec<u8>>, CosmwasmVMError<T>> {
		// TODO(hussein-aitlahcen): allow to raw query precompiled contracts?
		let info = Pallet::<T>::contract_info(&address)?;
		Pallet::<T>::do_db_read_other_contract(vm, &info.trie_id, key)
	}
}
