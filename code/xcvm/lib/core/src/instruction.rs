use crate::{AssetId, Balance, BridgeSecurity, Program};
use alloc::{
	borrow::Cow,
	collections::{BTreeMap, VecDeque},
	vec::Vec,
};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingValue {
	Register(Register),
	/// Asset's address
	Asset(AssetId),
	AssetAmount(AssetId, Balance),
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
	Spawn {
		network: Network,
		bridge_security: BridgeSecurity,
		salt: Vec<u8>,
		assets: Assets,
		program: Program<VecDeque<Self>>,
	},
	/// Query the state of a contract
	#[serde(rename_all = "snake_case")]
	Query { network: Network, salt: Vec<u8> },
}

/// Error types for late binding operation
pub enum LateBindingError<E> {
	/// Provided late-binding is invalid
	InvalidBinding,
	/// Generic app-specific error
	App(E),
}

/// Apply late bindings to the given payload.
///
/// * `payload`: Payload that is suitable for late-binding operation. Note that this API has no
///   assumption on the payload format or structure at all. It will only put the binding values in
///   the corresponding indices. If the payload were to be JSON, and `to` key supposed to have
///   late-binding, the payload would probably look similar to this:
/// ```{ "from": "address", "to": "" }```
/// * `bindings`: **SORTED** and **UNIQUE** (in-terms of index) binding index-value pairs.
/// * `formatted_payload`: Output payload. This should have enough size to contain the final data.
/// * `binding_data`: Callback function that gives the binding data corresponding to a binding value.
pub fn apply_bindings<'a, F, E>(
	payload: Vec<u8>,
	bindings: Bindings,
	formatted_payload: &mut Vec<u8>,
	binding_data: F,
) -> Result<(), LateBindingError<E>>
where
	F: Fn(BindingValue) -> Result<Cow<'a, [u8]>, E>,
{
	// Current index of the unformatted call
	let mut original_index: usize = 0;
	// This stores the amount of shifting we caused because of the data insertion. For example,
	// inserting a contract address "addr1234" causes 8 chars of shift. Which means index 'X' in
	// the unformatted call, will be equal to 'X + 8' in the output call.
	let mut offset: usize = 0;
	for (binding_index, binding) in bindings {
		let binding_index = binding_index as usize;
		// Current index of the output call
		let shifted_index = original_index + offset;

		// Check for overflow
		// * No need to check if `shifted_index` > `binding_index + offset` because `original_index
		//   > binding_index` already guarantees that
		// * No need to check if `shifted_index < formatted_call.len()` because initial allocation
		//   of `formatted_call` guarantees that even the max length can fit in.
		// * No need to check if `original_index < encoded_call.len()` because `original_index` is
		//   already less or equals to `binding_index` and we check if `binding_index` is in-bounds.
		if original_index > binding_index || binding_index + 1 >= payload.len() {
			return Err(LateBindingError::InvalidBinding)
		}

		// Copy everything until the index of where binding happens from original call
		// to formatted call. Eg.
		// Formatted call: `{ "hello": "" }`
		// Output call supposed to be: `{ "hello": "contract_addr" }`
		// In the first iteration, this will copy `{ "hello": "` to the formatted call.
		// SAFETY:
		//     - Two slices are in the same size for sure because `shifted_index` is
		//		 `original_index + offset` and `binding_index + offset - (shifted_index)`
		//       equals to `binding_index - original_index`.
		//     - Index accesses should not fail because we check if all indices are inbounds and
		//       also if `shifted` and `original` indices are greater than `binding_index`
		formatted_payload[shifted_index..=binding_index + offset]
			.copy_from_slice(&payload[original_index..=binding_index]);

		let data: Cow<[u8]> = binding_data(binding).map_err(LateBindingError::App)?;

		formatted_payload[binding_index + offset + 1..=binding_index + offset + data.len()]
			.copy_from_slice(&data);
		offset += data.len();
		original_index = binding_index + 1;
	}
	// Copy the rest of the data to the output data
	if original_index < payload.len() {
		formatted_payload[original_index + offset..payload.len() + offset]
			.copy_from_slice(&payload[original_index..]);
	}
	// Get rid of the final 0's.
	formatted_payload.truncate(payload.len() + offset);

	Ok(())
}
