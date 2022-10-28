use crate::{AssetId, Program};
use alloc::{
	collections::{BTreeMap, VecDeque},
	vec::Vec,
};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingValue {
	Register(Register),
	/// Asset's address
	Asset(AssetId),
}

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Register {
	/// Instruction pointer
	Ip,
	/// Relayer's address
	Relayer,
	/// Interpreter's address
	This,
	/// Result of the last executed instruction
	Result,
}

/// Bindings: (Index, Binding)
pub type Bindings = Vec<(u32, BindingValue)>;

/// Ordered Bindings: (Index, Binding)
pub type OrderedBindings = BTreeMap<u32, BindingValue>;

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Destination<Account> {
	Account(Account),
	Relayer,
}

/// Base XCVM instructions.
/// This set will remain as small as possible, expressiveness must come on `top` of the base
/// instructions.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Instruction<Network, Payload, Account, Assets> {
	/// Transfer some [`Assets`] from the current program to the [`to`] account.
	#[serde(rename_all = "snake_case")]
	Transfer { to: Destination<Account>, assets: Assets },
	/// Arbitrary payload representing a raw call inside the current [`Network`].
	///
	/// On picasso, this will be a SCALE encoded dispatch call.
	/// On ethereum, an ethereum ABI encoded call.
	/// On cosmos, a raw json WasmMsg call.
	///
	/// Depending on the network, the payload might be more structured than the base call.
	/// For most of the network in fact, we need to provide the target address along the payload,
	/// which can be encoded inside this single payload.
	#[serde(rename_all = "snake_case")]
	Call { bindings: Bindings, encoded: Payload },
	/// Spawn a sub-program on the target `network`.
	///
	/// The program will be spawned with the desired [`Assets`].
	/// The salt is used to track the program when events are dispatched in the network.
	#[serde(rename_all = "snake_case")]
	Spawn { network: Network, salt: Vec<u8>, assets: Assets, program: Program<VecDeque<Self>> },
	/// Query the state of a contract
	#[serde(rename_all = "snake_case")]
	Query { network: Network, salt: Vec<u8> },
}
