#![feature(generic_associated_types)]
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

pub mod instrument;
pub mod runtimes;
pub mod types;
pub mod weights;

#[allow(clippy::too_many_arguments)]
#[frame_support::pallet]
pub mod pallet {
	use crate::{
		instrument::gas_and_stack_instrumentation,
		runtimes::{
			abstraction::{CosmwasmAccount, Gas, VMPallet},
			wasmi::{
				CodeValidation, CosmwasmVM, CosmwasmVMCache, CosmwasmVMError, CosmwasmVMShared,
				ExportRequirement, InitialStorageMutability, ValidationError,
			},
		},
		types::{CodeInfo, ContractInfo},
		weights::WeightInfo,
	};
	use alloc::{
		collections::{btree_map::Entry, BTreeMap},
		string::String,
	};
	use composable_support::abstractions::{
		nonce::Nonce,
		utils::{
			increment::{Increment, SafeIncrement},
			start_at::ZeroInit,
		},
	};
	use core::fmt::Debug;
	use cosmwasm_minimal_std::{
		Addr, Attribute as CosmwasmEventAttribute, Binary as CosmwasmBinary, BlockInfo, Coin,
		ContractInfo as CosmwasmContractInfo, Env, Event as CosmwasmEvent, MessageInfo, Timestamp,
		TransactionInfo,
	};
	use cosmwasm_vm::{
		executor::{
			AllocateInput, AsFunctionName, DeallocateInput, ExecuteInput, InstantiateInput,
			MigrateInput, QueryInput, ReplyInput,
		},
		memory::PointerOf,
		system::{cosmwasm_system_entrypoint, CosmwasmCodeId},
		vm::VmMessageCustomOf,
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
	use sp_core::crypto::UncheckedFrom;
	use sp_runtime::traits::{Convert, Hash, MaybeDisplay, SaturatedConversion};
	use sp_std::vec::Vec;
	use wasm_instrument::gas_metering::ConstantCostRules;

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
	pub(crate) type AccountIdOf<T> = <T as Config>::AccountId;
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
	pub(crate) type CodeInfoOf<T> = CodeInfo<AccountIdOf<T>>;

	#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, Debug)]
	pub enum EntryPoint {
		Instantiate,
		Execute,
		Migrate,
		Reply,
		Sudo,
		Query,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Uploaded { code_hash: CodeHashOf<T>, code_id: CosmwasmCodeId },
		Instantiated { contract: AccountIdOf<T>, info: ContractInfoOf<T> },
		Executed { contract: AccountIdOf<T>, entrypoint: EntryPoint, data: Option<Vec<u8>> },
		ExecutionFailed { contract: AccountIdOf<T>, entrypoint: EntryPoint, error: Vec<u8> },
		Emitted { contract: AccountIdOf<T>, ty: Vec<u8>, attributes: Vec<(Vec<u8>, Vec<u8>)> },
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
		ChargeGas,
		RefundGas,
		LabelTooBig,
		UnknownDenom,
		StackOverflow,
		NotEnoughFundsForUpload,
		ContractNonceOverflow,
		NonceOverflow,
		RefcountOverflow,
		VMDepthOverflow,
		SignatureVerificationError,
		IteratorIdOverflow,
		IteratorNotFound,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config<AccountId = AccountIdOf<Self>> + Debug {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type AccountId: Parameter
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

		/// Price of writting a byte in the storage.
		#[pallet::constant]
		type ContractStorageByteWritePrice: Get<u32>;

		/// Price of extracting a byte from the storage.
		#[pallet::constant]
		type ContractStorageByteReadPrice: Get<u32>;

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
		StorageMap<_, Twox64Concat, CosmwasmCodeId, CodeInfo<AccountIdOf<T>>>;

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

	/// A mapping between a contract and it's nonce.
	/// The nonce is a monotonic counter incremented when the contract instantiate another contract.
	#[allow(clippy::disallowed_types)]
	#[pallet::storage]
	pub(crate) type ContractNonce<T: Config> =
		StorageMap<_, Identity, AccountIdOf<T>, u64, ValueQuery>;

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
		#[pallet::weight(T::WeightInfo::upload(code.len()))]
		pub fn upload(origin: OriginFor<T>, code: ContractCodeOf<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Self::do_upload(&who, code)?;
			Ok(().into())
		}

		/// Instantiate a previously uploaded code resulting in a new contract being generated.
		///
		/// * Emits an `Instantiated` event on success.
		/// * Emits an `Executed` event.
		/// * Possibily emit `Emitted` events.
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
		#[pallet::weight(T::WeightInfo::instantiate(funds.len()).saturating_add(*gas))]
		pub fn instantiate(
			origin: OriginFor<T>,
			code_id: CosmwasmCodeId,
			salt: ContractSaltOf<T>,
			admin: Option<AccountIdOf<T>>,
			label: ContractLabelOf<T>,
			funds: FundsOf<T>,
			gas: u64,
			message: ContractMessageOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let mut shared = Self::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
			let outcome = Self::do_extrinsic_instantiate(
				&mut shared,
				who,
				code_id,
				&salt,
				admin,
				label,
				funds,
				message,
			);
			log::debug!(target: "runtime::contracts", "Instantiate Result: {:?}", outcome);
			Self::refund_gas(outcome, gas, shared.gas.remaining())
		}

		/// Execute a previously instantiated contract.
		///
		/// * Emits an `Executed` event.
		/// * Possibily emit `Emitted` events.
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
		#[pallet::weight(T::WeightInfo::execute(funds.len()).saturating_add(*gas))]
		pub fn execute(
			origin: OriginFor<T>,
			contract: AccountIdOf<T>,
			funds: FundsOf<T>,
			gas: u64,
			message: ContractMessageOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let mut shared = Self::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
			let outcome = Self::do_extrinsic_execute(&mut shared, who, contract, funds, message);
			Self::refund_gas(outcome, gas, shared.gas.remaining())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Deterministic contract address computation, equivalent to solidity CREATE2.
		pub(crate) fn derive_contract_address(
			instantiator: &AccountIdOf<T>,
			code_id: CosmwasmCodeId,
			salt: &[u8],
		) -> AccountIdOf<T> {
			let data: Vec<_> = instantiator
				.as_ref()
				.iter()
				.chain(&code_id.to_le_bytes())
				.chain(salt)
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

		/// Setup pallet state for a new contract.
		/// This is called prior to calling the `instantiate` export of the contract.
		pub(crate) fn do_instantiate_phase1(
			instantiator: AccountIdOf<T>,
			code_id: CosmwasmCodeId,
			salt: &[u8],
			admin: Option<AccountIdOf<T>>,
			label: ContractLabelOf<T>,
		) -> Result<(AccountIdOf<T>, ContractInfoOf<T>), Error<T>> {
			let contract = Self::derive_contract_address(&instantiator, code_id, salt);
			ensure!(
				!ContractToInfo::<T>::contains_key(&contract),
				Error::<T>::ContractAlreadyExists
			);
			let nonce = CurrentNonce::<T>::increment().map_err(|_| Error::<T>::NonceOverflow)?;
			let trie_id = Self::derive_contract_trie_id(&contract, nonce);
			let contract_info =
				ContractInfoOf::<T> { instantiator, code_id, trie_id, admin, label };
			ContractToInfo::<T>::insert(&contract, &contract_info);
			CodeIdToInfo::<T>::try_mutate(code_id, |entry| -> Result<(), Error<T>> {
				let code_info = entry.as_mut().ok_or(Error::<T>::CodeNotFound)?;
				code_info.refcount =
					code_info.refcount.checked_add(1).ok_or(Error::<T>::RefcountOverflow)?;
				Ok(())
			})?;
			Self::deposit_event(Event::<T>::Instantiated {
				contract: contract.clone(),
				info: contract_info.clone(),
			});
			Ok((contract, contract_info))
		}

		/// Create the shared VM state. Including readonly stack, VM depth, gas metering limits and
		/// code cache.
		///
		/// This state is shared accross all VMs (all contracts loaded within a single call) and is
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

		pub(crate) fn do_extrinsic_instantiate(
			shared: &mut CosmwasmVMShared,
			instantiator: AccountIdOf<T>,
			code_id: CosmwasmCodeId,
			salt: &[u8],
			admin: Option<AccountIdOf<T>>,
			label: ContractLabelOf<T>,
			funds: FundsOf<T>,
			message: ContractMessageOf<T>,
		) -> Result<(), CosmwasmVMError<T>> {
			let (contract, info) =
				Self::do_instantiate_phase1(instantiator.clone(), code_id, salt, admin, label)?;
			Self::do_extrinsic_dispatch(
				shared,
				EntryPoint::Instantiate,
				instantiator,
				contract,
				info,
				funds,
				|vm| cosmwasm_system_entrypoint::<InstantiateInput, _>(vm, &message),
			)
		}

		pub(crate) fn do_extrinsic_execute(
			shared: &mut CosmwasmVMShared,
			sender: AccountIdOf<T>,
			contract: AccountIdOf<T>,
			funds: FundsOf<T>,
			message: ContractMessageOf<T>,
		) -> Result<(), CosmwasmVMError<T>> {
			let info = Self::contract_info(&contract)?;
			Self::do_extrinsic_dispatch(
				shared,
				EntryPoint::Execute,
				sender,
				contract,
				info,
				funds,
				|vm| cosmwasm_system_entrypoint::<ExecuteInput, _>(vm, &message),
			)
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
					for CosmwasmEvent { ty, attributes } in events {
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
			Self::do_transfer(&sender, &contract, &funds, false)?;
			let mut vm = Self::cosmwasm_new_vm(shared, sender, contract, info, funds)?;
			call(&mut vm)
		}

		/// Refund the remaining gas regardless of a contract outcome.
		pub(crate) fn refund_gas(
			outcome: Result<(), CosmwasmVMError<T>>,
			initial_gas: u64,
			remaining_gas: u64,
		) -> DispatchResultWithPostInfo {
			let post_info = PostDispatchInfo {
				actual_weight: Some(initial_gas.saturating_sub(remaining_gas)),
				pays_fee: Pays::Yes,
			};
			match outcome {
				Ok(()) => Ok(post_info),
				Err(e) => {
					let e = match e {
						CosmwasmVMError::Pallet(e) => e,
						_ => Error::<T>::ContractTrapped,
					};
					Err(DispatchErrorWithPostInfo { error: e.into(), post_info })
				},
			}
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

		/// Compute the next contract nonce and return it.
		/// This nonce is used in contract instantiation (contract -> contract).
		/// Consequently, we track a nonce for each instantiated contracts.
		pub(crate) fn next_contract_nonce(contract: &AccountIdOf<T>) -> Result<u64, Error<T>> {
			ContractNonce::<T>::try_mutate(contract, |nonce| -> Result<u64, Error<T>> {
				*nonce = nonce.checked_add(1).ok_or(Error::<T>::ContractNonceOverflow)?;
				Ok(*nonce)
			})
		}

		/// Current instrumentation version
		const INSTRUMENTATION_VERSION: u16 = 1;

		/// V1 exports, verified w.r.t https://github.com/CosmWasm/cosmwasm/#exports
		/// The format is (required, function_name, function_signature)
		const V1_EXPORTS: &'static [(
			ExportRequirement,
			&'static str,
			&'static [parity_wasm::elements::ValueType],
		)] = &[
			// We support v1+
			(ExportRequirement::Mandatory, "interface_version_8", &[]),
			// Memory related exports.
			(
				ExportRequirement::Mandatory,
				AllocateInput::<PointerOf<CosmwasmVM<T>>>::NAME,
				&[parity_wasm::elements::ValueType::I32],
			),
			(
				ExportRequirement::Mandatory,
				DeallocateInput::<PointerOf<CosmwasmVM<T>>>::NAME,
				&[parity_wasm::elements::ValueType::I32],
			),
			// Contract execution exports.
			(
				ExportRequirement::Mandatory,
				InstantiateInput::<VmMessageCustomOf<CosmwasmVM<T>>>::NAME,
				// extern "C" fn instantiate(env_ptr: u32, info_ptr: u32, msg_ptr: u32) -> u32;
				&[
					parity_wasm::elements::ValueType::I32,
					parity_wasm::elements::ValueType::I32,
					parity_wasm::elements::ValueType::I32,
				],
			),
			(
				ExportRequirement::Mandatory,
				ExecuteInput::<VmMessageCustomOf<CosmwasmVM<T>>>::NAME,
				// extern "C" fn execute(env_ptr: u32, info_ptr: u32, msg_ptr: u32) -> u32;
				&[
					parity_wasm::elements::ValueType::I32,
					parity_wasm::elements::ValueType::I32,
					parity_wasm::elements::ValueType::I32,
				],
			),
			(
				ExportRequirement::Mandatory,
				QueryInput::NAME,
				// extern "C" fn query(env_ptr: u32, msg_ptr: u32) -> u32;
				&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
			),
			(
				ExportRequirement::Optional,
				MigrateInput::<VmMessageCustomOf<CosmwasmVM<T>>>::NAME,
				// extern "C" fn migrate(env_ptr: u32, msg_ptr: u32) -> u32;
				&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
			),
			(
				ExportRequirement::Optional,
				ReplyInput::<VmMessageCustomOf<CosmwasmVM<T>>>::NAME,
				// extern "C" fn reply(env_ptr: u32, msg_ptr: u32) -> u32;
				&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
			),
		];

		/// Default module from where a CosmWasm import functions.
		const ENV_MODULE: &'static str = "env";

		/// Arbitrary function name for gas instrumentation.
		/// A contract is not allowed to import this function from the above [`ENV_MODULE`].
		const ENV_GAS: &'static str = "gas";

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
					.validate_exports(Self::V1_EXPORTS)?
					// env.gas is banned as injected by instrumentation
					.validate_imports(&[(Self::ENV_MODULE, Self::ENV_GAS)])?;
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
				Self::ENV_MODULE,
				T::CodeStackLimit::get(),
				// TODO(hussein-aitlahcen): this constant cost rules can't be used in production
				// and must be benchmarked we can reuse contracts pallet cost rules for now as
				// well.
				&ConstantCostRules::new(0, 0),
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
		/// If the instrumentation is outdated, re-instrument the pristive code.
		pub(crate) fn do_check_for_reinstrumentation(
			code_id: CosmwasmCodeId,
		) -> Result<(), Error<T>> {
			CodeIdToInfo::<T>::try_mutate(code_id, |entry| {
				let code_info = entry.as_mut().ok_or(Error::<T>::CodeNotFound)?;
				if code_info.instrumentation_version != Self::INSTRUMENTATION_VERSION {
					log::debug!(target: "runtime::contracts", "do_check_for_reinstrumentation: required");
					let code = PristineCode::<T>::get(code_id).ok_or(Error::<T>::CodeNotFound)?;
					let module = Self::do_load_module(&code)?;
					let instrumented_code = Self::do_instrument_code(module)?;
					InstrumentedCode::<T>::insert(&code_id, instrumented_code);
					code_info.instrumentation_version = Self::INSTRUMENTATION_VERSION;
				} else {
					log::debug!(target: "runtime::contracts", "do_check_for_reinstrumentation: not required");
				}
				Ok(())
			})
		}

		pub(crate) fn do_load_module(
			code: &ContractCodeOf<T>,
		) -> Result<parity_wasm::elements::Module, Error<T>> {
			parity_wasm::elements::Module::from_bytes(&code).map_err(|e| {
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
			let instrumented_code = Self::do_instrument_code(module)?;
			let code_id = CurrentCodeId::<T>::increment()?;
			PristineCode::<T>::insert(code_id, code);
			InstrumentedCode::<T>::insert(code_id, instrumented_code);
			CodeIdToInfo::<T>::insert(
				code_id,
				CodeInfoOf::<T> {
					creator: who.clone(),
					instrumentation_version: Self::INSTRUMENTATION_VERSION,
					refcount: 0,
				},
			);
			Self::deposit_event(Event::<T>::Uploaded { code_hash, code_id });
			Ok(())
		}

		/// Extract the current environment from the pallet.
		pub(crate) fn cosmwasm_env(cosmwasm_contract_address: CosmwasmAccount<T>) -> Env {
			Env {
				block: BlockInfo {
					height: frame_system::Pallet::<T>::block_number().saturated_into(),
					time: Timestamp(T::UnixTime::now().as_secs().into()),
					chain_id: T::ChainId::get().into(),
				},
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
			Coin { denom, amount: amount.into() }
		}

		/// Try to convert from a CosmWasm denom to a native [`AdsetIdOf<T>`].
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
					.flat_map(|(_, modules)| modules.into_iter().map(|(_, function)| function))
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
		/// writting the entry.
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
				let amount = (*amount).saturated_into();
				T::Assets::transfer(asset, from, to, amount, keep_alive)
					.map_err(|_| Error::<T>::TransferFailed)?;
			}
			Ok(())
		}
	}

	impl<T: Config> VMPallet for T {
		type VmError = CosmwasmVMError<T>;
	}
}
