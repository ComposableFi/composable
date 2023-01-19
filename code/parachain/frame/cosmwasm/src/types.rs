use crate::{runtimes::vm::CosmwasmVM, Config};
use codec::{Decode, Encode, MaxEncodedLen};
use cosmwasm_vm::system::CosmwasmCodeId;
use frame_support::{BoundedBTreeMap, BoundedVec};
use scale_info::TypeInfo;

/// Aliases to simplify implementation level.
pub type DefaultCosmwasmVM<'a, T> = CosmwasmVM<'a, T>;
pub type KeepAlive = bool;
pub type FundsOf<T> = BoundedBTreeMap<AssetIdOf<T>, (BalanceOf<T>, KeepAlive), MaxFundsAssetOf<T>>;
pub type ContractSaltOf<T> = BoundedVec<u8, MaxInstantiateSaltSizeOf<T>>;
pub type ContractMessageOf<T> = BoundedVec<u8, MaxMessageSizeOf<T>>;
pub type ContractCodeOf<T> = BoundedVec<u8, MaxCodeSizeOf<T>>;
pub type ContractInstrumentedCodeOf<T> = BoundedVec<u8, MaxInstrumentedCodeSizeOf<T>>;
pub type ContractTrieIdOf<T> = BoundedVec<u8, MaxContractTrieIdSizeOf<T>>;
pub type ContractLabelOf<T> = BoundedVec<u8, MaxContractLabelSizeOf<T>>;
pub type CodeHashOf<T> = <T as frame_system::Config>::Hash;
pub type AccountIdOf<T> = <T as Config>::AccountIdExtended;
pub type MaxCodeSizeOf<T> = <T as Config>::MaxCodeSize;
pub type MaxInstrumentedCodeSizeOf<T> = <T as Config>::MaxInstrumentedCodeSize;
pub type MaxMessageSizeOf<T> = <T as Config>::MaxMessageSize;
pub type MaxContractLabelSizeOf<T> = <T as Config>::MaxContractLabelSize;
pub type MaxContractTrieIdSizeOf<T> = <T as Config>::MaxContractTrieIdSize;
pub type MaxInstantiateSaltSizeOf<T> = <T as Config>::MaxInstantiateSaltSize;
pub type MaxFundsAssetOf<T> = <T as Config>::MaxFundsAssets;
pub type AssetIdOf<T> = <T as Config>::AssetId;
pub type BalanceOf<T> = <T as Config>::Balance;
pub type ContractInfoOf<T> = ContractInfo<AccountIdOf<T>, ContractLabelOf<T>, ContractTrieIdOf<T>>;
pub type CodeInfoOf<T> = CodeInfo<AccountIdOf<T>, CodeHashOf<T>>;

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, Debug)]
pub enum EntryPoint {
	Instantiate,
	Execute,
	Migrate,
	Reply,
	// TODO(hussein-aitlahcen): do we want to support cosmwasm sudo?
}

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, Debug)]
#[scale_info(skip_type_params(T))]
pub enum CodeIdentifier<T: Config> {
	CodeId(CosmwasmCodeId),
	CodeHash(CodeHashOf<T>),
}
/// Pallet contract/code metadata.
#[derive(Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
pub struct PalletContractCodeInfo<AccountId, Hash, Label, TrieId> {
	/// Hardcoded code info representing the precompiled code backing the contract.
	pub code: CodeInfo<AccountId, Hash>,
	/// Hardcoded contract info representing the precompiled contract.
	pub contract: ContractInfo<AccountId, Label, TrieId>,
}

/// Tracked code metadata.
#[derive(Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
pub struct CodeInfo<AccountId, Hash> {
	/// Original owner of the code.
	pub creator: AccountId,
	/// The hash of the pristine code.
	pub pristine_code_hash: Hash,
	/// Version of the instrumentation applied to the code.
	pub instrumentation_version: u16,
	/// Number of contract referencing this code.
	pub refcount: u32,
	/// Wether the contract export IBC functions and is consequently able to be called back by IBC
	/// operations.
	pub ibc_capable: bool,
}

/// Contract metadata.
#[derive(Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
pub struct ContractInfo<AccountId, Label, TrieId> {
	/// The code this contract is baked by.
	pub code_id: CosmwasmCodeId,
	/// The contract trie ID (The unique storage prefix).
	pub trie_id: TrieId,
	/// Account that created this instance.
	pub instantiator: AccountId,
	/// Current admin of the contract instance.
	/// If the value is [`None`], the contract cannot be migrated.
	pub admin: Option<AccountId>,
	/// Contract label defined by the instantiator.
	pub label: Label,
}
