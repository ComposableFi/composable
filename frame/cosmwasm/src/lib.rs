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

pub mod instrument;
pub mod runtimes;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		instrument::{gas_and_stack_instrumentation, InstrumentationError},
		runtimes::{
			abstraction::{CosmwasmAccount, VMPallet},
			wasmi::{
				CodeValidation, CosmwasmVM, CosmwasmVMError, ExportRequirement, ValidationError,
			},
		},
		types::{CodeInfo, ContractInfo},
	};
	use alloc::string::String;
	use composable_support::abstractions::{
		nonce::Nonce,
		utils::{
			increment::{Increment, SafeIncrement},
			start_at::ZeroInit,
		},
	};
	use core::{fmt::Debug, num::NonZeroU32};
	use cosmwasm_minimal_std::{
		Addr, Binary as CosmwasmBinary, BlockInfo, ContractInfo as CosmwasmContractInfo, Empty,
		Env, Event as CosmwasmEvent, MessageInfo, Timestamp, TransactionInfo,
	};
	use cosmwasm_vm::{
		executor::{cosmwasm_call, ExecuteInput, InstantiateInput},
		system::{cosmwasm_system_entrypoint, CosmwasmCallVM, CosmwasmCodeId},
		vm::{VMBase, VmErrorOf},
	};
	use cosmwasm_vm_wasmi::{host_functions, new_wasmi_vm, WasmiImportResolver, WasmiVM};
	use frame_support::{
		pallet_prelude::*,
		storage::child::ChildInfo,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::{AssetId, Balance},
			Get, UnixTime,
		},
		BoundedBTreeMap,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_core::crypto::UncheckedFrom;
	use sp_io::hashing::blake2_256;
	use sp_runtime::traits::{Convert, Hash, MaybeDisplay, SaturatedConversion};
	use sp_std::vec::Vec;
	use wasm_instrument::gas_metering::ConstantCostRules;

	pub(crate) type FundsOf<T> = BoundedBTreeMap<AssetIdOf<T>, BalanceOf<T>, MaxFundsAssetOf<T>>;
	pub(crate) type ContractSaltOf<T> = BoundedVec<u8, MaxInstantiateSaltSizeOf<T>>;
	pub(crate) type ContractMessageOf<T> = BoundedVec<u8, MaxMessageSizeOf<T>>;
	pub(crate) type ContractCodeOf<T> = BoundedVec<u8, MaxCodeSizeOf<T>>;
	pub(crate) type ContractInstrumentedCodeOf<T> = BoundedVec<u8, MaxInstrumentedCodeSizeOf<T>>;
	pub(crate) type ContractTrieIdOf<T> = BoundedVec<u8, MaxContractTrieIdSizeOf<T>>;
	pub(crate) type ContractLabelOf<T> = BoundedVec<u8, MaxContractLabelSizeOf<T>>;
	pub(crate) type CodeHashOf<T> = <T as frame_system::Config>::Hash;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
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
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Uploaded { code_hash: CodeHashOf<T>, code_id: CosmwasmCodeId },
		Instantiated { contract: AccountIdOf<T>, info: ContractInfoOf<T> },
		Executed { contract: AccountIdOf<T>, entrypoint: EntryPoint, data: Option<Vec<u8>> },
		Emitted { contract: AccountIdOf<T>, events: Vec<u8> },
	}

	#[pallet::error]
	pub enum Error<T> {
		Instrumentation,
		VmCreation,
		Instantiation,
		Execution,
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
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Current chain ID. Provided to the contract via the [`Env`].
		#[pallet::constant]
		type ChainId: Get<String>;

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

		/// A way to convert from our native account to cosmwasm `Addr`.
		type AccountToAddr: Convert<AccountIdOf<Self>, String>
			+ Convert<String, Result<AccountIdOf<Self>, ()>>;

		/// Type of an account balance.
		type Balance: Balance + From<u128>;

		/// Type of a tradable asset id.
		///
		/// The [`Ord`] constraint is required for [`BoundedBTreeMap`].
		type AssetId: AssetId + Ord;

		/// A way to convert from our native currency to cosmwasm `Denom`.
		type AssetToDenom: Convert<AssetIdOf<Self>, String>
			+ Convert<String, Result<AssetIdOf<Self>, ()>>;

		/// Interface from which we are going to execute assets operations.
		type Assets: Inspect<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ Transfer<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ Mutate<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>;

		/// Source of time.
		type UnixTime: UnixTime;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// A mapping from an original code hash to the original code, untouched by instrumentation.
	#[pallet::storage]
	pub(crate) type PristineCode<T: Config> =
		StorageMap<_, Twox64Concat, CosmwasmCodeId, BoundedVec<u8, MaxCodeSizeOf<T>>>;

	/// A mapping between an original code hash and instrumented wasm code, ready for execution.
	#[pallet::storage]
	pub(crate) type InstrumentedCode<T: Config> =
		StorageMap<_, Twox64Concat, CosmwasmCodeId, ContractInstrumentedCodeOf<T>>;

	/// Momotonic counter incremented on code creation.
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
	#[pallet::storage]
	pub(crate) type CurrentNonce<T: Config> =
		StorageValue<_, u64, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	/// A mapping between a contract and it's metadata.
	#[pallet::storage]
	pub(crate) type ContractToInfo<T: Config> =
		StorageMap<_, Identity, AccountIdOf<T>, ContractInfoOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		AccountIdOf<T>: UncheckedFrom<T::Hash> + AsRef<[u8]>,
	{
		/// Upload a CosmWasm contract.
		/// The function will ensure that the wasm module is well formed and that it fits the according limits.
		/// The module exports are going to be checked against the expected CosmWasm export signatures.
		///
		/// * Emits an `Uploaded` event on success.
		///
		/// Arguments
		///
		/// - `origin` the original dispatching the extrinsic.
		/// - `code` the actual wasm code.
		#[pallet::weight(0)]
		pub fn upload(origin: OriginFor<T>, code: ContractCodeOf<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Self::do_upload(&who, code)?;
			Ok(().into())
		}

		/// Instantiate a previously uploaded code resulting in a new contract being generated.
		///
		/// * Emits an `Instantiated` event on success.
		/// * Possibily emit a `ContractData` event.
		/// * Possibily emit a `ContractEvent` event.
		///
		/// Arguments
		///
		/// * `origin` the origin dispatching the extrinsic.
		/// * `code_id` the unique code id generated when the code has been uploaded via [`upload`].
		/// * `salt` the salt, usually used to instantiate the same contract multiple times.
		/// * `funds` the assets transferred to the contract prior to calling it's `instantiate` export.
		#[pallet::weight(0)]
		pub fn instantiate(
			origin: OriginFor<T>,
			code_id: CosmwasmCodeId,
			salt: ContractSaltOf<T>,
			admin: Option<AccountIdOf<T>>,
			label: ContractLabelOf<T>,
			funds: FundsOf<T>,
			message: ContractMessageOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Self::do_instantiate(who, code_id, salt, admin, label, funds, message)?;
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn execute(
			origin: OriginFor<T>,
			contract: AccountIdOf<T>,
			funds: FundsOf<T>,
			message: ContractMessageOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Self::do_execute(who, contract, funds, message)?;
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T>
	where
		AccountIdOf<T>: UncheckedFrom<T::Hash> + AsRef<[u8]>,
	{
		/// Deterministic contract address computation, equivalent to solidity CREATE2.
		fn derive_contract_address(
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
		fn derive_contract_trie_id(contract: &AccountIdOf<T>, nonce: u64) -> ContractTrieIdOf<T> {
			let data: Vec<_> =
				contract.as_ref().iter().chain(&nonce.to_le_bytes()).cloned().collect();
			T::Hashing::hash(&data).as_ref().to_vec().try_into().expect(
				"hashing len implementation must always be <= defined max contract trie id size; QED;",
			)
		}

		fn do_instantiate(
			instantiator: AccountIdOf<T>,
			code_id: CosmwasmCodeId,
			salt: ContractSaltOf<T>,
			admin: Option<AccountIdOf<T>>,
			label: ContractLabelOf<T>,
			funds: FundsOf<T>,
			message: ContractMessageOf<T>,
		) -> DispatchResult {
			let contract = Self::derive_contract_address(&instantiator, code_id, &salt);
			ensure!(
				!ContractToInfo::<T>::contains_key(&contract),
				Error::<T>::ContractAlreadyExists
			);
			let nonce = CurrentNonce::<T>::increment()?;
			let trie_id = Self::derive_contract_trie_id(&contract, nonce);
			let contract_info = ContractInfoOf::<T> {
				instantiator: instantiator.clone(),
				code_id,
				trie_id,
				admin,
				label,
			};
			ContractToInfo::<T>::insert(&contract, &contract_info);
			Self::deposit_event(Event::<T>::Instantiated {
				contract: contract.clone(),
				info: contract_info.clone(),
			});
			Self::cosmwasm_run::<InstantiateInput<Empty>>(
				EntryPoint::Instantiate,
				instantiator,
				contract,
				contract_info,
				message,
			)
			.map_err(|_| Error::<T>::Instantiation)?;
			Ok(())
		}

		fn do_execute(
			sender: AccountIdOf<T>,
			contract: AccountIdOf<T>,
			funds: FundsOf<T>,
			message: ContractMessageOf<T>,
		) -> DispatchResult {
			let contract_info =
				ContractToInfo::<T>::get(&contract).ok_or(Error::<T>::ContractNotFound)?;
			Self::cosmwasm_run::<ExecuteInput<Empty>>(
				EntryPoint::Execute,
				sender,
				contract,
				contract_info,
				message,
			)
			.map_err(|_| Error::<T>::Execution)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// Current instrumentation version
		const INSTRUMENTATION_VERSION: u16 = 1;

		//
		// Exports are verified w.r.t https://github.com/CosmWasm/cosmwasm/#exports
		//
		const V1_EXPORTS: &'static [(
			ExportRequirement,
			&'static str,
			&'static [parity_wasm::elements::ValueType],
		)] = &[
			// We support v1+
			(ExportRequirement::Mandatory, "interface_version_8", &[]),
			// Memory related exports.
			(ExportRequirement::Mandatory, "allocate", &[parity_wasm::elements::ValueType::I32]),
			(ExportRequirement::Mandatory, "deallocate", &[parity_wasm::elements::ValueType::I32]),
			// Contract execution exports.
			(
				ExportRequirement::Mandatory,
				"instantiate",
				// extern "C" fn instantiate(env_ptr: u32, info_ptr: u32, msg_ptr: u32) -> u32;
				&[
					parity_wasm::elements::ValueType::I32,
					parity_wasm::elements::ValueType::I32,
					parity_wasm::elements::ValueType::I32,
				],
			),
			(
				ExportRequirement::Mandatory,
				"execute",
				// extern "C" fn execute(env_ptr: u32, info_ptr: u32, msg_ptr: u32) -> u32;
				&[
					parity_wasm::elements::ValueType::I32,
					parity_wasm::elements::ValueType::I32,
					parity_wasm::elements::ValueType::I32,
				],
			),
			(
				ExportRequirement::Mandatory,
				"query",
				// extern "C" fn query(env_ptr: u32, msg_ptr: u32) -> u32;
				&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
			),
			(
				ExportRequirement::Optional,
				"migrate",
				// extern "C" fn migrate(env_ptr: u32, info_ptr: u32, msg_ptr: u32) -> u32;
				&[
					parity_wasm::elements::ValueType::I32,
					parity_wasm::elements::ValueType::I32,
					parity_wasm::elements::ValueType::I32,
				],
			),
			(
				ExportRequirement::Optional,
				"reply",
				// extern "C" fn reply(env_ptr: u32, msg_ptr: u32) -> u32;
				&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
			),
		];
		const ENV_MODULE: &'static str = "env";
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
				let _ = CodeValidation::new(&module)
					.validate_base()?
					.validate_memory_limit()?
					.validate_table_size_limit(T::CodeTableSizeLimit::get())?
					.validate_global_variable_limit(T::CodeGlobalVariableLimit::get())?
					.validate_parameter_limit(T::CodeParameterLimit::get())?
					.validate_br_table_size_limit(T::CodeBranchTableSizeLimit::get())?
					.validate_no_floating_types()?
					.validate_exports(Self::V1_EXPORTS)?
					// env.gas is banned as injected by instrumentation
					.validate_imports(Self::ENV_MODULE, &[(Self::ENV_MODULE, Self::ENV_GAS)])?;
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
			let instrumented_module = gas_and_stack_instrumentation(
				module,
				Self::ENV_MODULE,
				T::CodeStackLimit::get(),
				// TODO(hussein-aitlahcen): this constant cost rules can't be used in production and must be benchmarked
				// we can reuse contracts pallet cost rules for now as well.
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
					let code = PristineCode::<T>::get(code_id).ok_or(Error::<T>::CodeNotFound)?;
					let module = Self::do_load_module(&code)?;
					Self::do_validate_code(&module)?;
					let instrumented_code = Self::do_instrument_code(module)?;
					InstrumentedCode::<T>::insert(&code_id, instrumented_code);
					code_info.instrumentation_version = Self::INSTRUMENTATION_VERSION;
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
			let module = Self::do_load_module(&code)?;
			Self::do_validate_code(&module)?;
			let instrumented_code = Self::do_instrument_code(module)?;
			let code_id = CurrentCodeId::<T>::increment()?;
			PristineCode::<T>::insert(code_id, code);
			InstrumentedCode::<T>::insert(code_id, instrumented_code);
			CodeIdToInfo::<T>::insert(
				code_id,
				CodeInfoOf::<T> {
					creator: who.clone(),
					instrumentation_version: Self::INSTRUMENTATION_VERSION,
					refcount: 1,
				},
			);
			Self::deposit_event(Event::<T>::Uploaded { code_hash, code_id });
			Ok(())
		}

		/// Extract the current environment from the pallet.
		pub(crate) fn cosmwasm_env(
			cosmwasm_contract_address: CosmwasmAccount<T, AccountIdOf<T>>,
		) -> Env {
			Env {
				block: BlockInfo {
					height: frame_system::Pallet::<T>::block_number().saturated_into(),
					time: Timestamp(T::UnixTime::now().as_secs().into()),
					chain_id: T::ChainId::get(),
				},
				transaction: frame_system::Pallet::<T>::extrinsic_index()
					.map(|index| TransactionInfo { index }),
				contract: CosmwasmContractInfo {
					address: Addr::unchecked(Into::<String>::into(
						cosmwasm_contract_address.clone(),
					)),
				},
			}
		}

		pub(crate) fn cosmwasm_new_vm(
			sender: AccountIdOf<T>,
			contract: AccountIdOf<T>,
			info: ContractInfoOf<T>,
		) -> Result<WasmiVM<CosmwasmVM<T>>, <WasmiVM<CosmwasmVM<T>> as VMBase>::Error> {
			let code = InstrumentedCode::<T>::get(info.code_id).ok_or(Error::<T>::CodeNotFound)?;
			let host_functions_definitions = WasmiImportResolver(host_functions::definitions());
			let module = new_wasmi_vm(&host_functions_definitions, &code)
				.map_err(|_| Error::<T>::VmCreation)?;
			let cosmwasm_sender_address: CosmwasmAccount<T, AccountIdOf<T>> =
				CosmwasmAccount::new(sender);
			let cosmwasm_contract_address: CosmwasmAccount<T, AccountIdOf<T>> =
				CosmwasmAccount::new(contract);
			Ok(WasmiVM(CosmwasmVM::<T> {
				host_functions: host_functions_definitions
					.0
					.clone()
					.into_iter()
					.flat_map(|(_, modules)| modules.into_iter().map(|(_, function)| function))
					.collect(),
				executing_module: module,
				cosmwasm_env: Self::cosmwasm_env(cosmwasm_contract_address.clone()),
				cosmwasm_message_info: MessageInfo {
					sender: Addr::unchecked(Into::<String>::into(cosmwasm_sender_address)),
					funds: Default::default(),
				},
				contract_address: cosmwasm_contract_address,
				contract_info: info,
				_marker: PhantomData,
			}))
		}

		pub(crate) fn cosmwasm_run<I>(
			entrypoint: EntryPoint,
			sender: AccountIdOf<T>,
			contract: AccountIdOf<T>,
			info: ContractInfoOf<T>,
			message: ContractMessageOf<T>,
		) -> Result<(Option<CosmwasmBinary>, Vec<CosmwasmEvent>), VmErrorOf<WasmiVM<CosmwasmVM<T>>>>
		where
			WasmiVM<CosmwasmVM<T>>: CosmwasmCallVM<I>,
			VmErrorOf<WasmiVM<CosmwasmVM<T>>>: From<Error<T>>,
		{
			Self::do_check_for_reinstrumentation(info.code_id)?;
			let mut vm = Self::cosmwasm_new_vm(sender, contract.clone(), info)?;
			cosmwasm_system_entrypoint::<I, WasmiVM<CosmwasmVM<T>>>(&mut vm, &message)
				.map(|(data, events)| {
					Self::deposit_event(Event::<T>::Emitted {
						contract: contract.clone(),
						events: serde_json::to_vec(&events).unwrap(),
					});
					Self::deposit_event(Event::<T>::Executed {
						contract,
						entrypoint,
						data: data.clone().map(Into::into),
					});
					(data, events)
				})
				.map_err(|e| {
					log::debug!(target: "runtime::contracts", "cosmwasm_run: {:#?}", e);
					e
				})
		}

		/// Build a [`ChildInfo`] out of a contract trie id.
		pub(crate) fn contract_child_trie(trie_id: &[u8]) -> ChildInfo {
			ChildInfo::new_default(trie_id)
		}

		/// Abstract function to operate on a contract child trie entry.
		pub(crate) fn with_db_entry<R>(
			trie_id: &ContractTrieIdOf<T>,
			key: Vec<u8>,
			f: impl FnOnce(ChildInfo, [u8; 32]) -> R,
		) -> Result<R, Error<T>> {
			let child_trie = Self::contract_child_trie(trie_id.as_ref());
			Ok(f(child_trie, blake2_256(&key)))
		}

		pub(crate) fn do_db_read(
			trie_id: &ContractTrieIdOf<T>,
			key: Vec<u8>,
		) -> Result<Option<Vec<u8>>, Error<T>> {
			Self::with_db_entry(trie_id, key, |child_trie, entry| {
				storage::child::get_raw(&child_trie, &entry)
			})
		}

		pub(crate) fn do_db_write(
			trie_id: &ContractTrieIdOf<T>,
			key: Vec<u8>,
			value: Vec<u8>,
		) -> Result<(), Error<T>> {
			Self::with_db_entry(trie_id, key, |child_trie, entry| {
				storage::child::put_raw(&child_trie, &entry, &value)
			})
		}

		pub(crate) fn do_db_remove(
			trie_id: &ContractTrieIdOf<T>,
			key: Vec<u8>,
		) -> Result<(), Error<T>> {
			Self::with_db_entry(trie_id, key, |child_trie, entry| {
				storage::child::kill(&child_trie, &entry)
			})
		}

		pub(crate) fn cosmwasm_addr_to_account(
			cosmwasm_addr: String,
		) -> Result<AccountIdOf<T>, <T as VMPallet>::VmError> {
			T::AccountToAddr::convert(cosmwasm_addr)
				.map_err(|()| CosmwasmVMError::AccountConversionFailure)
		}

		pub(crate) fn account_to_cosmwasm_addr(account: AccountIdOf<T>) -> String {
			T::AccountToAddr::convert(account)
		}
	}

	impl<T: Config> VMPallet for T {
		type VmError = VmErrorOf<CosmwasmVM<T>>;
	}
}
