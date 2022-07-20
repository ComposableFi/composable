use codec::{Decode, Encode, MaxEncodedLen};
use cosmwasm_vm::system::CosmwasmCodeId;
use scale_info::TypeInfo;

/// Tracked code metadatas.
#[derive(Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
pub struct CodeInfo<AccountId> {
	/// Original owner of the code.
	pub creator: AccountId,
	/// Version of the instrumentation applied to the code.
	pub instrumentation_version: u16,
	/// Number of contract referencing this code.
	pub refcount: u32,
}

/// Contract metadatas.
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
	/// Contract label defined by the instantiator.
	pub label: Label,
}
