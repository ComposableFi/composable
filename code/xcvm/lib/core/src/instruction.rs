use crate::Program;
use alloc::{collections::VecDeque, vec::Vec};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

/// Base XCVM instructions.
/// This set will remain as small as possible, expressiveness must come on `top` of the base
/// instructions.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Instruction<Network, Payload, Account, Assets> {
	/// Transfer some [`Assets`] from the current program to the [`to`] account.
	#[serde(rename_all = "snake_case")]
	Transfer { to: Account, assets: Assets },
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
	Call { encoded: Payload },
	/// Spawn a sub-program on the target `network`.
	///
	/// The program will be spawned with the desired [`Assets`].
	/// The salt is used to track the program when events are dispatched in the network.
	#[serde(rename_all = "snake_case")]
	Spawn { network: Network, salt: Vec<u8>, assets: Assets, program: Program<VecDeque<Self>> },
}
