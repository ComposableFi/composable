use codec::{Decode, Encode, MaxEncodedLen};
use cosmwasm_vm::system::CosmwasmCodeId;
use scale_info::TypeInfo;

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
	/// The contract trie ID.
	pub trie_id: TrieId,
	/// Account that created this instance.
	pub instantiator: AccountId,
	/// Current admin of the contract instance.
	/// If the value is [`None`], the contract cannot be migrated.
	pub admin: Option<AccountId>,
	/// Contract label defined by the instantiator.w
	pub label: Label,
}
