use codec::{Encode, Decode, MaxEncodedLen};
use scale_info::TypeInfo;

/// Tracked code metadatas.
#[derive(Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
pub struct CodeInfo<AccountId> {
  /// Original owner of the code.
  pub creator: AccountId,
}

/// Contract metadatas.
#[derive(Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
pub struct ContractInfo<AccountId, Label> {
  /// Account that created this instance.
  pub instantiator: AccountId,
  /// Current admin of the contract instance.
  /// If the value is [`None`], the contract cannot be migrated.
  pub admin: Option<AccountId>,
  /// Contract label defined by the instantiator.
  pub label: Label,
}
