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
pub type CodeInfoOf<T> = CodeInfo<AccountIdOf<T>>;

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, Debug)]
pub enum EntryPoint {
	Instantiate,
	Execute,
	Migrate,
	Reply,
	IbcChannelOpen,
	IbcChannelConnect,
	IbcChannelClose,
	IbcPacketTimeout,
	IbcPacketAck,
}

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, Debug)]
pub enum CodeIdentifier {
	CodeId(CosmwasmCodeId),
	CodeHash([u8; 32]),
}
/// Pallet contract/code metadata.
#[derive(Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
pub struct PalletContractCodeInfo<AccountId, Label, TrieId> {
	/// Hardcoded code info representing the precompiled code backing the contract.
	pub code: CodeInfo<AccountId>,
	/// Hardcoded contract info representing the precompiled contract.
	pub contract: ContractInfo<AccountId, Label, TrieId>,
}

/// Tracked code metadata.
#[derive(Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
pub struct CodeInfo<AccountId> {
	/// Original owner of the code.
	pub creator: AccountId,
	/// The hash of the pristine code.
	pub pristine_code_hash: [u8; 32],
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
