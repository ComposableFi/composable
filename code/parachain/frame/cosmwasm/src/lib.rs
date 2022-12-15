#![recursion_limit = "256"]
#![feature(sync_unsafe_cell)]
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

pub use pallet::*;

pub mod ibc;
pub mod instrument;
pub mod runtimes;
pub mod types;
pub mod version;
pub mod weights;
pub use crate::ibc::NoRelayer;
mod entrypoint;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[allow(clippy::too_many_arguments)]
#[frame_support::pallet]
pub mod pallet {
	const SUBSTRATE_ECDSA_SIGNATURE_LEN: usize = 65;
	use crate::{
		entrypoint::*,
		instrument::{gas_and_stack_instrumentation, CostRules, INSTRUMENTATION_VERSION},
		runtimes::{
			abstraction::{CanonicalCosmwasmAccount, CosmwasmAccount, Gas, VMPallet},
			wasmi::{
				CodeValidation, CosmwasmVM, CosmwasmVMCache, CosmwasmVMError, CosmwasmVMShared,
				InitialStorageMutability, ValidationError,
			},
		},
		types::{CodeInfo, ContractInfo},
		version::Version,
		weights::WeightInfo,
	};
	use alloc::{
		collections::{btree_map::Entry, BTreeMap},
		format,
		string::String,
		vec,
	};
	use composable_support::abstractions::{
		nonce::Nonce,
		utils::{
			increment::{Increment, SafeIncrement},
			start_at::ZeroInit,
		},
	};
	use core::fmt::Debug;
	use cosmwasm_vm::{
		cosmwasm_std::{
			Addr, Attribute as CosmwasmEventAttribute, Binary as CosmwasmBinary, BlockInfo, Coin,
			ContractInfo as CosmwasmContractInfo, ContractInfoResponse, Env,
			Event as CosmwasmEvent, MessageInfo, Timestamp, TransactionInfo,
		},
		executor::{
			cosmwasm_call, ExecuteCall, InstantiateCall, MigrateCall, QueryCall, QueryResponse,
			ReplyCall,
		},
		system::{cosmwasm_system_query, CosmwasmCodeId, CosmwasmContractMeta},
	};
	use cosmwasm_vm_wasmi::{host_functions, new_wasmi_vm, WasmiImportResolver, WasmiVM};
	use frame_support::{
		dispatch::{DispatchErrorWithPostInfo, DispatchResultWithPostInfo, PostDispatchInfo},
		pallet_prelude::*,
		storage::{child::ChildInfo, ChildTriePrefixIterator},
		traits::{
			fungibles::{Inspect as FungiblesInspect, Transfer as FungiblesTransfer},
			tokens::{AssetId, Balance},
			Get, ReservableCurrency, UnixTime,
		},
		transactional, BoundedBTreeMap, PalletId, StorageHasher, Twox64Concat,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_core::{crypto::UncheckedFrom, ecdsa, ed25519};
	use sp_runtime::traits::{Convert, Hash, MaybeDisplay, SaturatedConversion};
	use sp_std::vec::Vec;

	pub(crate) type KeepAlive = bool;
	pub(crate) type FundsOf<T> =
		BoundedBTreeMap<AssetIdOf<T>, (BalanceOf<T>, KeepAlive), MaxFundsAssetOf<T>>;
	pub(crate) type ContractSaltOf<T> = BoundedVec<u8, MaxInstantiateSaltSizeOf<T>>;
	pub(crate) type ContractMessageOf<T> = BoundedVec<u8, MaxMessageSizeOf<T>>;
	pub(crate) type ContractCodeOf<T> = BoundedVec<u8, MaxCodeSizeOf<T>>;
	pub(crate) type ContractInstrumentedCodeOf<T> = BoundedVec<u8, MaxInstrumentedCodeSizeOf<T>>;
	pub(crate) type ContractTrieIdOf<T> = BoundedVec<u8, MaxContractTrieIdSizeOf<T>>;
	pub(crate) type ContractLabelOf<T> = BoundedVec<u8, MaxContractLabelSizeOf<T>>;
	pub(crate) type CodeHashOf<T> = <T as frame_system::Config>::Hash;
	pub(crate) type AccountIdOf<T> = <T as Config>::AccountIdExtended;
	pub(crate) type MaxCodeSizeOf<T> = <T as Config>::MaxCodeSize;
	pub(crate) type MaxInstrumentedCodeSizeOf<T> = <T as Config>::MaxInstrumentedCodeSize;
	pub(crate) type MaxMessageSizeOf<T> = <T as Config>::MaxMessageSize;
	pub(crate) type MaxContractLabelSizeOf<T> = <T as Config>::MaxContractLabelSize;
	pub(crate) type MaxContractTrieIdSizeOf<T> = <T as Config>::MaxContractTrieIdSize;
	pub(crate) type MaxInstantiateSaltSizeOf<T> = <T as Config>::MaxInstantiateSaltSize;
	pub(crate) type MaxFundsAssetOf<T> = <T as Config>::MaxFundsAssets;
	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub(crate) type ContractInfoOf<T> =
		ContractInfo<AccountIdOf<T>, ContractLabelOf<T>, ContractTrieIdOf<T>>;
	pub(crate) type CodeInfoOf<T> = CodeInfo<AccountIdOf<T>, CodeHashOf<T>>;

	#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, Debug)]
	pub enum EntryPoint {
		Instantiate,
		Execute,
		Migrate,
		Reply,
		Sudo,
		Query,
	}

	#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, Debug)]
	#[scale_info(skip_type_params(T))]
	pub enum CodeIdentifier<T: Config> {
		CodeId(CosmwasmCodeId),
		CodeHash(CodeHashOf<T>),
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Uploaded { code_hash: CodeHashOf<T>, code_id: CosmwasmCodeId },
		Instantiated { contract: AccountIdOf<T>, info: ContractInfoOf<T> },
		Executed { contract: AccountIdOf<T>, entrypoint: EntryPoint, data: Option<Vec<u8>> },
		ExecutionFailed { contract: AccountIdOf<T>, entrypoint: EntryPoint, error: Vec<u8> },
		Emitted { contract: AccountIdOf<T>, ty: Vec<u8>, attributes: Vec<(Vec<u8>, Vec<u8>)> },
		Migrated { contract: AccountIdOf<T>, to: CosmwasmCodeId },
		AdminUpdated { contract: AccountIdOf<T>, new_admin: AccountIdOf<T> },
		AdminCleared { contract: AccountIdOf<T> },
		IbcChannelOpen { contract: AccountIdOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		Instrumentation,
		VmCreation,
		ContractTrapped,
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
		NotAuthorized,
		Unsupported,
		Ibc,
		FailedToSerialize,
		OutOfGas,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config<AccountId = AccountIdOf<Self>> + Debug {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

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

		/// Max number of frames a contract is able to push, a.k.a recursive calls.
		#[pallet::constant]
		type MaxFrames: Get<u32>;

		/// Max accepted code size.
		#[pallet::constant]
		type MaxCodeSize: Get<u32>;

		/// Max code size after gas instrumentation.
		#[pallet::constant]
		type MaxInstrumentedCodeSize: Get<u32>;

		/// Max message size.
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
			+ Convert<String, Result<AccountIdOf<Self>, ()>>;

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
			> + FungiblesTransfer<
				AccountIdOf<Self>,
				Balance = BalanceOf<Self>,
				AssetId = AssetIdOf<Self>,
			>;

		/// Source of time.
		type UnixTime: UnixTime;

		/// Weight implementation.
		type WeightInfo: WeightInfo;

		/// account which execute relayer calls IBC exported entry points
		type IbcRelayerAccount: Get<AccountIdOf<Self>>;

		type IbcRelayer: ibc_primitives::IbcHandler<AccountIdOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
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
	pub(crate) type CodeHashToId<T: Config> =
		StorageMap<_, Identity, CodeHashOf<T>, CosmwasmCodeId>;

	/// This is a **monotonic** counter incremented on contract instantiation.
	/// The purpose of this nonce is just to make sure that contract trie are unique.
	#[allow(clippy::disallowed_types)]
	#[pallet::storage]
	pub(crate) type CurrentNonce<T: Config> =
		StorageValue<_, u64, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	/// A mapping between a contract and it's metadata.
	#[pallet::storage]
	pub(crate) type ContractToInfo<T: Config> =
		StorageMap<_, Identity, AccountIdOf<T>, ContractInfoOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Upload a CosmWasm contract.
		/// The function will ensure that the wasm module is well formed and that it fits the
		/// according limits. The module exports are going to be checked against the expected
		/// CosmWasm export signatures.
		///
		/// * Emits an `Uploaded` event on success.
		///
		/// Arguments
		///
		/// - `origin` the original dispatching the extrinsic.
		/// - `code` the actual wasm code.
		#[transactional]
		#[pallet::weight(T::WeightInfo::upload(code.len() as u32))]
		pub fn upload(origin: OriginFor<T>, code: ContractCodeOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_upload(&who, code)
		}

		/// Instantiate a previously uploaded code resulting in a new contract being generated.
		///
		/// * Emits an `Instantiated` event on success.
		/// * Emits an `Executed` event.
		/// * Possibly emit `Emitted` events.
		///
		/// Arguments
		///
		/// * `origin` the origin dispatching the extrinsic.
		/// * `code_id` the unique code id generated when the code has been uploaded via [`upload`].
		/// * `salt` the salt, usually used to instantiate the same contract multiple times.
		/// * `funds` the assets transferred to the contract prior to calling it's `instantiate`
		///   export.
		/// * `gas` the maximum gas to use, the remaining is refunded at the end of the transaction.
		#[transactional]
		#[pallet::weight(T::WeightInfo::instantiate(funds.len() as u32).saturating_add(*gas))]
		pub fn instantiate(
			origin: OriginFor<T>,
			code_identifier: CodeIdentifier<T>,
			salt: ContractSaltOf<T>,
			admin: Option<AccountIdOf<T>>,
			label: ContractLabelOf<T>,
			funds: FundsOf<T>,
			gas: u64,
			message: ContractMessageOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let mut shared = Self::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
			let initial_gas = T::WeightInfo::instantiate(funds.len() as u32).saturating_add(gas);
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
		/// Arguments
		///
		/// * `origin` the origin dispatching the extrinsic.
		/// * `code_id` the unique code id generated when the code has been uploaded via [`upload`].
		/// * `salt` the salt, usually used to instantiate the same contract multiple times.
		/// * `funds` the assets transferred to the contract prior to calling it's `instantiate`
		///   export.
		/// * `gas` the maximum gas to use, the remaining is refunded at the end of the transaction.
		#[transactional]
		#[pallet::weight(T::WeightInfo::execute(funds.len() as u32).saturating_add(*gas))]
		pub fn execute(
			origin: OriginFor<T>,
			contract: AccountIdOf<T>,
			funds: FundsOf<T>,
			gas: u64,
			message: ContractMessageOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let mut shared = Self::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
			let initial_gas = T::WeightInfo::execute(funds.len() as u32).saturating_add(gas);
			let outcome = Self::do_execute(&mut shared, who, contract, funds, message);
			Self::refund_gas(outcome, initial_gas, shared.gas.remaining())
		}

		/// Migrate a previously instantiated contract.
		///
		/// * Emits a `Migrated` event on success.
		/// * Emits an `Executed` event.
		/// * Possibly emit `Emitted` events.
		///
		/// Arguments
		///
		/// * `origin` the origin dispatching the extrinsic.
		/// * `contract` the address of the contract that we want to migrate
		/// * `new_code_identifier` the code identifier that we want to switch to.
		/// * `gas` the maximum gas to use, the remaining is refunded at the end of the transaction.
		/// * `message` MigrateMsg, that will be passed to the contract.
		#[transactional]
		#[pallet::weight(T::WeightInfo::migrate().saturating_add(*gas))]
		pub fn migrate(
			origin: OriginFor<T>,
			contract: AccountIdOf<T>,
			new_code_identifier: CodeIdentifier<T>,
			gas: u64,
			message: ContractMessageOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let mut shared = Self::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
			let initial_gas = T::WeightInfo::migrate().saturating_add(gas);
			let outcome =
				Self::do_migrate(&mut shared, who, contract, new_code_identifier, message);
			Self::refund_gas(outcome, initial_gas, shared.gas.remaining())
		}

		/// Update the admin of a contract.
		///
		/// * Emits a `AdminUpdated` event on success.
		///
		/// Arguments
		///
		/// * `origin` the origin dispatching the extrinsic.
		/// * `contract` the address of the contract that we want to migrate.
		/// * `new_admin` new admin of the contract that we want to update to.
		/// * `gas` the maximum gas to use, the remaining is refunded at the end of the transaction.
		#[transactional]
		#[pallet::weight(T::WeightInfo::update_admin().saturating_add(*gas))]
		pub fn update_admin(
			origin: OriginFor<T>,
			contract: AccountIdOf<T>,
			new_admin: AccountIdOf<T>,
			gas: u64,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let mut shared = Self::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
			let initial_gas = T::WeightInfo::update_admin().saturating_add(gas);
			let outcome =
				Self::do_update_admin(&mut shared, who, contract.clone(), Some(new_admin.clone()));
			Self::deposit_event(Event::<T>::AdminUpdated { contract, new_admin });
			Self::refund_gas(outcome, initial_gas, shared.gas.remaining())
		}

		/// Clear the admin of a contract.
		///
		/// * Emits a `AdminCleared` event on success.
		///
		/// Arguments
		///
		/// * `origin` the origin dispatching the extrinsic.
		/// * `contract` the address of the contract that we want to migrate.
		/// * `gas` the maximum gas to use, the remaining is refunded at the end of the transaction.
		#[transactional]
		#[pallet::weight(T::WeightInfo::clear_admin().saturating_add(*gas))]
		pub fn clear_admin(
			origin: OriginFor<T>,
			contract: AccountIdOf<T>,
			gas: u64,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let mut shared = Self::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
			let initial_gas = T::WeightInfo::clear_admin().saturating_add(gas);
			let outcome = Self::do_update_admin(&mut shared, who, contract.clone(), None);
			Self::deposit_event(Event::<T>::AdminCleared { contract });
			Self::refund_gas(outcome, initial_gas, shared.gas.remaining())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Deterministic contract address computation, similar to https://eips.ethereum.org/EIPS/eip-1014.
		pub(crate) fn derive_contract_address(
			instantiator: &AccountIdOf<T>,
			salt: &[u8],
			code_hash: CodeHashOf<T>,
			message: &[u8],
		) -> AccountIdOf<T> {
			let data: Vec<_> = instantiator
				.as_ref()
				.iter()
				.chain(salt)
				.chain(code_hash.as_ref())
				.chain(T::Hashing::hash(message).as_ref())
				.cloned()
				.collect();
			UncheckedFrom::unchecked_from(T::Hashing::hash(&data))
		}

		/// Deterministic contract trie id generation.
		pub(crate) fn derive_contract_trie_id(
			contract: &AccountIdOf<T>,
			nonce: u64,
		) -> ContractTrieIdOf<T> {
			let data: Vec<_> =
				contract.as_ref().iter().chain(&nonce.to_le_bytes()).cloned().collect();
			T::Hashing::hash(&data).as_ref().to_vec().try_into().expect(
				"hashing len implementation must always be <= defined max contract trie id size; QED;",
			)
		}

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
				gas: Gas::new(T::MaxFrames::get(), gas),
				cache: CosmwasmVMCache { code: Default::default() },
			}
		}

		/// Wrapper around [`Pallet::<T>::cosmwasm_call`] for extrinsics.
		/// It's purpose is converting the input and emit events based on the result.
		pub(crate) fn do_extrinsic_dispatch<F>(
			shared: &mut CosmwasmVMShared,
			entrypoint: EntryPoint,
			sender: AccountIdOf<T>,
			contract: AccountIdOf<T>,
			info: ContractInfoOf<T>,
			funds: FundsOf<T>,
			call: F,
		) -> Result<(), CosmwasmVMError<T>>
		where
			F: for<'x> FnOnce(
				&'x mut WasmiVM<CosmwasmVM<'x, T>>,
			) -> Result<
				(Option<CosmwasmBinary>, Vec<CosmwasmEvent>),
				CosmwasmVMError<T>,
			>,
		{
			let cosmwasm_funds = funds
				.into_iter()
				.map(|(asset, (amount, _))| Self::native_asset_to_cosmwasm_asset(asset, amount))
				.collect::<Vec<_>>();
			Self::cosmwasm_call(shared, sender, contract.clone(), info, cosmwasm_funds, call).map(
				|(data, events)| {
					for CosmwasmEvent { ty, attributes, .. } in events {
						Self::deposit_event(Event::<T>::Emitted {
							contract: contract.clone(),
							ty: ty.into(),
							attributes: attributes
								.into_iter()
								.map(|CosmwasmEventAttribute { key, value }| {
									(key.into(), value.into())
								})
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
		pub(crate) fn cosmwasm_call<F, R>(
			shared: &mut CosmwasmVMShared,
			sender: AccountIdOf<T>,
			contract: AccountIdOf<T>,
			info: ContractInfoOf<T>,
			funds: Vec<Coin>,
			call: F,
		) -> Result<R, CosmwasmVMError<T>>
		where
			F: for<'x> FnOnce(&'x mut WasmiVM<CosmwasmVM<'x, T>>) -> Result<R, CosmwasmVMError<T>>,
		{
			let mut vm = Self::cosmwasm_new_vm(shared, sender, contract, info, funds)?;
			call(&mut vm)
		}

		/// Refund the remaining gas regardless of a contract outcome.
		pub(crate) fn refund_gas(
			outcome: Result<(), CosmwasmVMError<T>>,
			initial_gas: u64,
			remaining_gas: u64,
		) -> DispatchResultWithPostInfo {
			log::info!(target: "runtime::contracts", "outcome: {:?}", outcome);
			let post_info = PostDispatchInfo {
				actual_weight: Some(initial_gas.saturating_sub(remaining_gas)),
				pays_fee: Pays::Yes,
			};
			match outcome {
				Ok(()) => Ok(post_info),
				Err(e) => {
					let e = match e {
						CosmwasmVMError::Pallet(e) => e,
						CosmwasmVMError::OutOfGas => Error::<T>::OutOfGas,
						_ => Error::<T>::ContractTrapped,
					};
					Err(DispatchErrorWithPostInfo { error: e.into(), post_info })
				},
			}
		}

		/// Set the contract info and update the state accordingly.
		///
		/// This function will update the state if the `code_id` is changing:
		/// 1. Refcount of the new `code_id` is incremented.
		/// 2. Refcount of the old `code_id` is decremented.
		/// 3. Delete every entry related to old `code_id` if
		///    the refcount is 0. And unreserve the bonded funds.
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
				CodeIdToInfo::<T>::try_mutate_exists(
					info.code_id,
					|entry| -> Result<(), Error<T>> {
						// Decrement the refcount
						let code_info = entry.as_mut().ok_or(Error::<T>::CodeNotFound)?;
						code_info.refcount = code_info
							.refcount
							.checked_sub(1)
							.ok_or(Error::<T>::RefcountOverflow)?;
						if code_info.refcount == 0 {
							// Unreserve the bonded funds for this code
							let code = PristineCode::<T>::try_get(info.code_id)
								.map_err(|_| Error::<T>::CodeNotFound)?;
							let deposit =
								code.len().saturating_mul(T::CodeStorageByteDeposit::get() as _);
							let _ = T::NativeAsset::unreserve(
								&code_info.creator,
								deposit.saturated_into(),
							);
							let code_hash = T::Hashing::hash(&code);
							PristineCode::<T>::remove(info.code_id);
							InstrumentedCode::<T>::remove(info.code_id);
							CodeHashToId::<T>::remove(code_hash);
							// Code is unused after this point, so it can be removed
							*entry = None;
						}
						Ok(())
					},
				)?;
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

		/// Handy wrapper to return contract info.
		pub(crate) fn contract_info(
			contract: &AccountIdOf<T>,
		) -> Result<ContractInfoOf<T>, Error<T>> {
			ContractToInfo::<T>::get(contract).ok_or(Error::<T>::ContractNotFound)
		}

		/// Handy wrapper to update contract info.
		pub(crate) fn set_contract_info(contract: &AccountIdOf<T>, info: ContractInfoOf<T>) {
			ContractToInfo::<T>::insert(contract, info)
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
		pub(crate) fn do_validate_code(
			module: &parity_wasm::elements::Module,
		) -> Result<(), Error<T>> {
			let validation: Result<(), ValidationError> = (|| {
				let _ = CodeValidation::new(module)
					.validate_base()?
					.validate_memory_limit()?
					.validate_table_size_limit(T::CodeTableSizeLimit::get())?
					.validate_global_variable_limit(T::CodeGlobalVariableLimit::get())?
					.validate_parameter_limit(T::CodeParameterLimit::get())?
					.validate_br_table_size_limit(T::CodeBranchTableSizeLimit::get())?
					.validate_no_floating_types()?
					.validate_exports(Version::<T>::EXPORTS)?
					// env.gas is banned as injected by instrumentation
					.validate_imports(&[(Version::<T>::ENV_MODULE, Version::<T>::ENV_GAS)])?;
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
				Version::<T>::ENV_MODULE,
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
		) -> Result<(), Error<T>> {
			CodeIdToInfo::<T>::try_mutate(code_id, |entry| {
				let code_info = entry.as_mut().ok_or(Error::<T>::CodeNotFound)?;
				if code_info.instrumentation_version != INSTRUMENTATION_VERSION {
					log::debug!(target: "runtime::contracts", "do_check_for_reinstrumentation: required");
					let code = PristineCode::<T>::get(code_id).ok_or(Error::<T>::CodeNotFound)?;
					let module = Self::do_load_module(&code)?;
					let instrumented_code = Self::do_instrument_code(module)?;
					InstrumentedCode::<T>::insert(code_id, instrumented_code);
					code_info.instrumentation_version = INSTRUMENTATION_VERSION;
				} else {
					log::debug!(target: "runtime::contracts", "do_check_for_reinstrumentation: not required");
				}
				Ok(())
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

		pub(crate) fn do_upload(who: &AccountIdOf<T>, code: ContractCodeOf<T>) -> DispatchResult {
			let code_hash = T::Hashing::hash(&code);
			ensure!(!CodeHashToId::<T>::contains_key(code_hash), Error::<T>::CodeAlreadyExists);
			let deposit = code.len().saturating_mul(T::CodeStorageByteDeposit::get() as _);
			// TODO: release this when the code is destroyed, a.k.a. refcount => 0 after a contract
			// migration for instance.
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

		fn do_instantiate(
			shared: &mut CosmwasmVMShared,
			who: AccountIdOf<T>,
			code_identifier: CodeIdentifier<T>,
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
			EntryPointCaller::<InstantiateCall>::setup(who, code_id, &salt, admin, label, &message)?
				.call(shared, funds, message)
				.map(|_| ())
		}

		fn do_execute(
			shared: &mut CosmwasmVMShared,
			who: AccountIdOf<T>,
			contract: AccountIdOf<T>,
			funds: FundsOf<T>,
			message: ContractMessageOf<T>,
		) -> Result<(), CosmwasmVMError<T>> {
			EntryPointCaller::<ExecuteCall>::setup(who, contract)?.call(shared, funds, message)
		}

		fn do_migrate(
			shared: &mut CosmwasmVMShared,
			who: AccountIdOf<T>,
			contract: AccountIdOf<T>,
			new_code_identifier: CodeIdentifier<T>,
			message: ContractMessageOf<T>,
		) -> Result<(), CosmwasmVMError<T>> {
			let new_code_id = match new_code_identifier {
				CodeIdentifier::CodeId(code_id) => code_id,
				CodeIdentifier::CodeHash(code_hash) =>
					CodeHashToId::<T>::try_get(code_hash).map_err(|_| Error::<T>::CodeNotFound)?,
			};

			EntryPointCaller::<MigrateCall>::setup(shared, who, contract, new_code_id)?.call(
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
			let info = Self::contract_info(&contract)?;
			let outcome = Self::cosmwasm_call(
				shared,
				who.clone(),
				contract.clone(),
				info,
				Default::default(),
				|vm| {
					cosmwasm_vm::system::update_admin(
						vm,
						&Addr::unchecked(Self::account_to_cosmwasm_addr(who)),
						CosmwasmAccount::new(contract),
						new_admin.map(CosmwasmAccount::new),
					)
					.map_err(Into::into)
				},
			);
			outcome
		}

		fn block_env() -> BlockInfo {
			BlockInfo {
				height: frame_system::Pallet::<T>::block_number().saturated_into(),
				time: Timestamp::from_seconds(T::UnixTime::now().as_secs()),
				chain_id: T::ChainId::get().into(),
			}
		}

		/// Extract the current environment from the pallet.
		pub(crate) fn cosmwasm_env(cosmwasm_contract_address: CosmwasmAccount<T>) -> Env {
			Env {
				block: Self::block_env(),
				transaction: frame_system::Pallet::<T>::extrinsic_index()
					.map(|index| TransactionInfo { index }),
				contract: CosmwasmContractInfo {
					address: Addr::unchecked(Into::<String>::into(cosmwasm_contract_address)),
				},
			}
		}

		/// Try to convert from a CosmWasm address to a native AccountId.
		pub(crate) fn cosmwasm_addr_to_account(
			cosmwasm_addr: String,
		) -> Result<AccountIdOf<T>, <T as VMPallet>::VmError> {
			T::AccountToAddr::convert(cosmwasm_addr)
				.map_err(|()| CosmwasmVMError::AccountConversionFailure)
		}

		/// Convert from a native account to a CosmWasm address.
		pub(crate) fn account_to_cosmwasm_addr(account: AccountIdOf<T>) -> String {
			T::AccountToAddr::convert(account)
		}

		/// Convert a native asset and amount into a CosmWasm [`Coin`].
		pub(crate) fn native_asset_to_cosmwasm_asset(
			asset: AssetIdOf<T>,
			amount: BalanceOf<T>,
		) -> Coin {
			let denom = T::AssetToDenom::convert(asset);
			Coin { denom, amount: amount.into().into() }
		}

		/// Try to convert from a CosmWasm denom to a native [`AssetIdOf<T>`].
		pub(crate) fn cosmwasm_asset_to_native_asset(
			denom: String,
		) -> Result<AssetIdOf<T>, Error<T>> {
			T::AssetToDenom::convert(denom).map_err(|_| Error::<T>::UnknownDenom)
		}

		/// Create a new CosmWasm VM. One instance is created per contract but all of them share the
		/// same [`CosmwasmVMShared<'a, T>`] structure.
		///
		/// Prior to instantiating the VM. The depth is checked against [`T::MaxFrames`] and the
		/// contract code is loaded from the shared state if cached. If the code is not in cache, we
		/// check whether reinstrumentation is required and cache the code.
		pub(crate) fn cosmwasm_new_vm<'a>(
			shared: &'a mut CosmwasmVMShared,
			sender: AccountIdOf<T>,
			contract: AccountIdOf<T>,
			info: ContractInfoOf<T>,
			funds: Vec<Coin>,
		) -> Result<WasmiVM<CosmwasmVM<'a, T>>, CosmwasmVMError<T>> {
			shared.depth = shared.depth.checked_add(1).ok_or(Error::<T>::VMDepthOverflow)?;
			ensure!(shared.depth <= T::MaxFrames::get(), Error::<T>::StackOverflow);
			let code = match shared.cache.code.entry(info.code_id) {
				Entry::Vacant(v) => {
					log::debug!(target: "runtime::contracts", "Code cache miss: {:?}", info.code_id);
					Self::do_check_for_reinstrumentation(info.code_id)?;
					let code = InstrumentedCode::<T>::get(info.code_id)
						.ok_or(Error::<T>::CodeNotFound)?
						.into_inner();
					v.insert(code)
				},
				Entry::Occupied(o) => {
					log::debug!(target: "runtime::contracts", "Code cache hit: {:?}", info.code_id);
					o.into_mut()
				},
			};
			let host_functions_definitions =
				WasmiImportResolver(host_functions::definitions::<CosmwasmVM<'a, T>>());
			let module = new_wasmi_vm(&host_functions_definitions, code)
				.map_err(|_| Error::<T>::VmCreation)?;
			let cosmwasm_sender_address: CosmwasmAccount<T> = CosmwasmAccount::new(sender);
			let cosmwasm_contract_address: CosmwasmAccount<T> = CosmwasmAccount::new(contract);
			let env = Self::cosmwasm_env(cosmwasm_contract_address.clone());
			let message_info = MessageInfo {
				sender: Addr::unchecked(Into::<String>::into(cosmwasm_sender_address)),
				funds,
			};
			log::debug!(target: "runtime::contracts", "cosmwasm_new_vm: {:#?}", env);
			log::debug!(target: "runtime::contracts", "cosmwasm_new_vm: {:#?}", message_info);
			Ok(WasmiVM(CosmwasmVM::<'a, T> {
				host_functions_by_name: host_functions_definitions.0.clone(),
				host_functions_by_index: host_functions_definitions
					.0
					.into_iter()
					.flat_map(|(_, modules)| modules.into_values())
					.collect(),
				executing_module: module,
				cosmwasm_env: env,
				cosmwasm_message_info: message_info,
				contract_address: cosmwasm_contract_address,
				contract_info: info,
				shared,
				iterators: BTreeMap::new(),
			}))
		}

		/// Build a [`ChildInfo`] out of a contract trie id.
		pub(crate) fn contract_child_trie(trie_id: &[u8]) -> ChildInfo {
			ChildInfo::new_default(trie_id)
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
				u64::from(bytes_to_read)
					.saturating_mul(T::ContractStorageByteReadPrice::get().into())
			})
		}

		/// Read an entry from the executing contract storage, charging the according gas prior to
		/// actually reading the entry.
		pub(crate) fn do_db_read<'a>(
			vm: &'a mut CosmwasmVM<T>,
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
		pub(crate) fn do_db_read_other_contract<'a>(
			vm: &'a mut CosmwasmVM<T>,
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
		pub(crate) fn do_db_write_gas(
			trie_id: &ContractTrieIdOf<T>,
			key: &[u8],
			value: &[u8],
		) -> u64 {
			Self::with_db_entry(trie_id, key, |child_trie, entry| {
				let bytes_to_write = match storage::child::len(&child_trie, &entry) {
					Some(current_len) => current_len.saturating_sub(value.len() as _),
					None => value.len() as u32,
				};
				u64::from(bytes_to_write)
					.saturating_mul(T::ContractStorageByteWritePrice::get().into())
			})
		}

		/// Write an entry from the executing contract, charging the according gas prior to actually
		/// writing the entry.
		pub(crate) fn do_db_write<'a>(
			vm: &'a mut CosmwasmVM<T>,
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
		pub(crate) fn do_db_scan(vm: &mut CosmwasmVM<T>) -> Result<u32, CosmwasmVMError<T>> {
			let iterator_id = vm.iterators.len() as u32;
			let child_info = Self::contract_child_trie(vm.contract_info.trie_id.as_ref());
			vm.iterators.insert(
				iterator_id,
				ChildTriePrefixIterator::<_>::with_prefix_over_key::<Blake2_128Concat>(
					&child_info,
					&[],
				),
			);
			Ok(iterator_id)
		}

		/// Return the next (reversed-key, value) pair and save the state. If the next key
		/// is `None`, the iterator is removed from the storage.
		pub(crate) fn do_db_next(
			vm: &mut CosmwasmVM<T>,
			iterator_id: u32,
		) -> Result<Option<(Vec<u8>, Vec<u8>)>, CosmwasmVMError<T>> {
			let iterator =
				vm.iterators.get_mut(&iterator_id).ok_or(Error::<T>::IteratorNotFound)?;
			match iterator.next() {
				Some((key, value)) => {
					let price = Self::do_db_read_gas(&vm.contract_info.trie_id, &key);
					vm.charge_raw(price)?;
					Ok(Some((key, value)))
				},
				None => Ok(None),
			}
		}

		/// Remove an entry from the executing contract, no gas is charged for this operation.
		pub(crate) fn do_db_remove<'a>(vm: &'a mut CosmwasmVM<T>, key: &[u8]) {
			let trie_id = &vm.contract_info.trie_id;
			Self::with_db_entry(trie_id, key, |child_trie, entry| {
				storage::child::kill(&child_trie, &entry)
			})
		}

		pub(crate) fn do_running_contract_meta(
			vm: &mut CosmwasmVM<T>,
		) -> CosmwasmContractMeta<CosmwasmAccount<T>> {
			CosmwasmContractMeta {
				code_id: vm.contract_info.code_id,
				admin: vm.contract_info.admin.clone().map(CosmwasmAccount::new),
				label: String::from_utf8_lossy(&vm.contract_info.label).into(),
			}
		}

		pub(crate) fn do_contract_meta(
			address: AccountIdOf<T>,
		) -> Result<CosmwasmContractMeta<CosmwasmAccount<T>>, CosmwasmVMError<T>> {
			let info = Pallet::<T>::contract_info(&address)?;
			Ok(CosmwasmContractMeta {
				code_id: info.code_id,
				admin: info.admin.clone().map(CosmwasmAccount::new),
				label: String::from_utf8_lossy(&info.label).into(),
			})
		}

		/// Validate a string address
		pub(crate) fn do_addr_validate(
			address: String,
		) -> Result<AccountIdOf<T>, CosmwasmVMError<T>> {
			Pallet::<T>::cosmwasm_addr_to_account(address)
		}

		/// Canonicalize a human readable address
		pub(crate) fn do_addr_canonicalize(
			address: String,
		) -> Result<AccountIdOf<T>, CosmwasmVMError<T>> {
			Pallet::<T>::cosmwasm_addr_to_account(address)
		}

		/// Humanize a canonical address
		pub(crate) fn do_addr_humanize(
			address: &CanonicalCosmwasmAccount<T>,
		) -> CosmwasmAccount<T> {
			address.0.clone()
		}

		/// Retrieve an account balance.
		pub(crate) fn do_balance(
			account: &AccountIdOf<T>,
			denom: String,
		) -> Result<u128, Error<T>> {
			let asset = Self::cosmwasm_asset_to_native_asset(denom)?;
			Ok(T::Assets::balance(asset, account).into())
		}

		/// Execute a transfer of funds between two accounts.
		pub(crate) fn do_transfer(
			from: &AccountIdOf<T>,
			to: &AccountIdOf<T>,
			funds: &[Coin],
			keep_alive: bool,
		) -> Result<(), Error<T>> {
			// Move funds to contract.
			for Coin { denom, amount } in funds {
				let asset = Self::cosmwasm_asset_to_native_asset(denom.clone())?;
				let amount = amount.u128().saturated_into();
				T::Assets::transfer(asset, from, to, amount, keep_alive)
					.map_err(|_| Error::<T>::TransferFailed)?;
			}
			Ok(())
		}

		pub(crate) fn do_secp256k1_recover_pubkey(
			message_hash: &[u8],
			signature: &[u8],
			recovery_param: u8,
		) -> Result<Vec<u8>, ()> {
			// `recovery_param` must be 0 or 1. Other values are not supported from CosmWasm.
			if recovery_param >= 2 {
				return Err(())
			}

			if signature.len() != SUBSTRATE_ECDSA_SIGNATURE_LEN - 1 {
				return Err(())
			}

			// Try into a [u8; 32]
			let message_hash = message_hash.try_into().map_err(|_| ())?;

			let signature = {
				// Since we fill `signature_inner` with `recovery_param`, when 64 bytes are written
				// the final byte will be the `recovery_param`.
				let mut signature_inner = [recovery_param; SUBSTRATE_ECDSA_SIGNATURE_LEN];
				signature_inner[..SUBSTRATE_ECDSA_SIGNATURE_LEN - 1].copy_from_slice(signature);
				signature_inner
			};

			sp_io::crypto::secp256k1_ecdsa_recover(&signature, &message_hash)
				.map(|without_tag| {
					let mut with_tag = vec![0x04_u8];
					with_tag.extend_from_slice(&without_tag[..]);
					with_tag
				})
				.map_err(|_| ())
		}

		pub(crate) fn do_secp256k1_verify(
			message_hash: &[u8],
			signature: &[u8],
			public_key: &[u8],
		) -> bool {
			let message_hash = match message_hash.try_into() {
				Ok(message_hash) => message_hash,
				Err(_) => return false,
			};

			// We are expecting 64 bytes long public keys but the substrate function use an
			// additional byte for recovery id. So we insert a dummy byte.
			let signature = {
				let mut signature_inner = [0_u8; SUBSTRATE_ECDSA_SIGNATURE_LEN];
				signature_inner[..SUBSTRATE_ECDSA_SIGNATURE_LEN - 1].copy_from_slice(signature);
				ecdsa::Signature(signature_inner)
			};

			let public_key = match libsecp256k1::PublicKey::parse_slice(public_key, None) {
				Ok(public_key) => ecdsa::Public::from_raw(public_key.serialize_compressed()),
				Err(_) => return false,
			};

			sp_io::crypto::ecdsa_verify_prehashed(&signature, &message_hash, &public_key)
		}

		pub(crate) fn do_ed25519_batch_verify(
			messages: &[&[u8]],
			signatures: &[&[u8]],
			public_keys: &[&[u8]],
		) -> bool {
			let mut messages = messages.to_vec();
			let mut public_keys = public_keys.to_vec();

			if messages.len() == signatures.len() && messages.len() == public_keys.len() {
				// Nothing needs to be done
			} else if messages.len() == 1 && signatures.len() == public_keys.len() {
				// There can be a single message signed with different signature-public key pairs
				messages = messages.repeat(signatures.len());
			} else if public_keys.len() == 1 && messages.len() == signatures.len() {
				// Single entity(with a public key) might wanna verify different messages
				public_keys = public_keys.repeat(signatures.len());
			} else {
				// Any other case is wrong
				return false
			}

			// Each batch verification process is started with `start_batch_verify` and ended with
			// `finish_batch_verify`. When it is started, it needs to be properly finished. But this
			// means `finish_batch_verify` will verify the previously pushed verification tasks. We
			// converted all the public keys and signatures in-front not to unnecessarily verify
			// previously pushed signatures. (Note that there is no function to ditch the batch
			// verification early without doing any verification)
			let mut verify_items = Vec::with_capacity(messages.len());
			for ((message, signature), public_key) in
				messages.iter().zip(signatures.iter()).zip(public_keys.iter())
			{
				match ((*signature).try_into(), (*public_key).try_into()) {
					(Ok(signature), Ok(public_key)) =>
						verify_items.push((signature, message, public_key)),
					_ => return false,
				}
			}

			sp_io::crypto::start_batch_verify();

			for (signature, message, public_key) in verify_items {
				// This is very unlikely to fail. Because this only fails if the verification task
				// cannot be spawned internally. Note that the actual verification is only done when
				// `finish_batch_verify` is called.
				if !sp_io::crypto::ed25519_batch_verify(&signature, message, &public_key) {
					let _ = sp_io::crypto::finish_batch_verify();
					return false
				}
			}

			sp_io::crypto::finish_batch_verify()
		}

		pub(crate) fn do_ed25519_verify(
			message: &[u8],
			signature: &[u8],
			public_key: &[u8],
		) -> bool {
			let signature: ed25519::Signature = match signature.try_into() {
				Ok(signature) => signature,
				Err(_) => return false,
			};

			let public_key: ed25519::Public = match public_key.try_into() {
				Ok(public_key) => public_key,
				Err(_) => return false,
			};

			sp_io::crypto::ed25519_verify(&signature, message, &public_key)
		}

		pub(crate) fn do_continue_instantiate<'a>(
			vm: &'a mut CosmwasmVM<T>,
			CosmwasmContractMeta { code_id, admin, label }: CosmwasmContractMeta<
				CosmwasmAccount<T>,
			>,
			funds: Vec<Coin>,
			message: &[u8],
			event_handler: &mut dyn FnMut(cosmwasm_vm::cosmwasm_std::Event),
		) -> Result<Option<cosmwasm_vm::cosmwasm_std::Binary>, CosmwasmVMError<T>> {
			EntryPointCaller::<InstantiateCall>::setup(
				vm.contract_address.clone().into_inner(),
				code_id,
				&[],
				admin.map(|admin| admin.into_inner()),
				label
					.as_bytes()
					.to_vec()
					.try_into()
					.map_err(|_| crate::Error::<T>::LabelTooBig)?,
				message,
			)?
			.continue_run(vm.shared, funds, message, event_handler)
		}

		pub(crate) fn do_continue_execute<'a>(
			vm: &'a mut CosmwasmVM<T>,
			contract: AccountIdOf<T>,
			funds: Vec<Coin>,
			message: &[u8],
			event_handler: &mut dyn FnMut(cosmwasm_vm::cosmwasm_std::Event),
		) -> Result<Option<cosmwasm_vm::cosmwasm_std::Binary>, CosmwasmVMError<T>> {
			EntryPointCaller::<ExecuteCall>::setup(
				vm.contract_address.clone().into_inner(),
				contract,
			)?
			.continue_run(vm.shared, funds, message, event_handler)
		}

		pub(crate) fn do_continue_reply<'a>(
			vm: &'a mut CosmwasmVM<T>,
			reply: cosmwasm_vm::cosmwasm_std::Reply,
			event_handler: &mut dyn FnMut(cosmwasm_vm::cosmwasm_std::Event),
		) -> Result<Option<cosmwasm_vm::cosmwasm_std::Binary>, CosmwasmVMError<T>> {
			EntryPointCaller::<ReplyCall>::setup(
				vm.contract_address.clone().into_inner(),
				vm.contract_address.clone().into_inner(),
			)?
			.continue_run(
				vm.shared,
				Vec::default(),
				&serde_json::to_vec(&reply).map_err(|_| Error::<T>::FailedToSerialize)?,
				event_handler,
			)
		}

		pub(crate) fn do_continue_migrate<'a>(
			vm: &'a mut CosmwasmVM<T>,
			contract: AccountIdOf<T>,
			message: &[u8],
			event_handler: &mut dyn FnMut(cosmwasm_vm::cosmwasm_std::Event),
		) -> Result<Option<cosmwasm_vm::cosmwasm_std::Binary>, CosmwasmVMError<T>> {
			let CosmwasmContractMeta { code_id, .. } = Self::do_running_contract_meta(vm);
			EntryPointCaller::<MigrateCall>::setup(
				vm.shared,
				vm.contract_address.clone().into_inner(),
				contract,
				code_id,
			)?
			.continue_run(vm.shared, Default::default(), message, event_handler)
		}

		pub(crate) fn do_query_info(
			vm: &mut CosmwasmVM<T>,
			address: AccountIdOf<T>,
		) -> Result<ContractInfoResponse, CosmwasmVMError<T>> {
			// TODO: cache or at least check if its current contract and use `self.contract_info`
			let info = Pallet::<T>::contract_info(&address)?;
			let code_info = CodeIdToInfo::<T>::get(info.code_id).ok_or(Error::<T>::CodeNotFound)?;
			let ibc_port = if code_info.ibc_capable {
				Some(Pallet::<T>::do_compute_ibc_contract_port(address))
			} else {
				None
			};
			let pinned = vm.shared.cache.code.contains_key(&info.code_id);
			let creator = CosmwasmAccount::<T>::new(info.instantiator.clone());
			let mut contract_info_response = ContractInfoResponse::new(info.code_id, creator);
			contract_info_response.admin =
				info.admin.map(|admin| CosmwasmAccount::<T>::new(admin).into());
			contract_info_response.pinned = pinned;
			contract_info_response.ibc_port = ibc_port;

			Ok(contract_info_response)
		}

		pub(crate) fn do_continue_query<'a>(
			vm: &'a mut CosmwasmVM<T>,
			contract: AccountIdOf<T>,
			message: &[u8],
		) -> Result<cosmwasm_vm::executor::QueryResult, CosmwasmVMError<T>> {
			log::debug!(target: "runtime::contracts", "query_continuation");
			let sender = vm.contract_address.clone().into_inner();
			let info = Pallet::<T>::contract_info(&contract)?;
			vm.shared.push_readonly();
			let result = Pallet::<T>::cosmwasm_call(
				vm.shared,
				sender,
				contract,
				info,
				Default::default(),
				|vm| cosmwasm_call::<QueryCall, WasmiVM<CosmwasmVM<T>>>(vm, message),
			);
			vm.shared.pop_readonly();
			result
		}

		pub(crate) fn do_query_raw<'a>(
			vm: &'a mut CosmwasmVM<T>,
			address: AccountIdOf<T>,
			key: &[u8],
		) -> Result<Option<Vec<u8>>, CosmwasmVMError<T>> {
			let info = Pallet::<T>::contract_info(&address)?;
			Pallet::<T>::do_db_read_other_contract(vm, &info.trie_id, key)
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
		let info = Pallet::<T>::contract_info(&contract)?;
		let query_request = serde_json::from_slice(&query_request)
			.map_err(|e| CosmwasmVMError::Rpc(format!("{}", e)))?;
		Pallet::<T>::cosmwasm_call(
			&mut shared,
			contract.clone(),
			contract,
			info,
			Default::default(),
			|vm| {
				cosmwasm_system_query(vm, query_request)?
					.into_result()
					.map_err(|e| CosmwasmVMError::Rpc(format!("{:?}", e)))?
					.into_result()
					.map_err(|e| CosmwasmVMError::Rpc(e))
			},
		)
	}

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
			.map_err(|_| CosmwasmVMError::Rpc(String::from("'salt' is too large")))?;
		let label: ContractLabelOf<T> = label
			.try_into()
			.map_err(|_| CosmwasmVMError::Rpc(String::from("'label' is too large")))?;
		let funds: FundsOf<T> = funds
			.try_into()
			.map_err(|_| CosmwasmVMError::Rpc(String::from("'funds' is too large")))?;
		let message: ContractMessageOf<T> = message
			.try_into()
			.map_err(|_| CosmwasmVMError::Rpc(String::from("'message' is too large")))?;
		let mut shared = Pallet::<T>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
		EntryPointCaller::<InstantiateCall>::setup(
			instantiator,
			code_id,
			&salt,
			admin,
			label,
			&message,
		)?
		.call(&mut shared, funds, message)
	}

	pub fn execute<T: Config>(
		executor: AccountIdOf<T>,
		contract: AccountIdOf<T>,
		funds: FundsOf<T>,
		gas: u64,
		message: ContractMessageOf<T>,
	) -> Result<(), CosmwasmVMError<T>> {
		let mut shared = Pallet::<T>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
		EntryPointCaller::<ExecuteCall>::setup(executor, contract)?.call(
			&mut shared,
			funds,
			message,
		)
	}

	impl<T: Config> VMPallet for T {
		type VmError = CosmwasmVMError<T>;
	}
}
