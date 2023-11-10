use crate::{service::dex::ExchangeId, Amount, AssetId, Program};
use alloc::{
	borrow::Cow,
	collections::{BTreeMap, VecDeque},
	vec::Vec,
};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingValue {
	Register(Register),
	/// Asset's address
	Asset(AssetId),
	AssetAmount(AssetId, Amount),
}

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Register {
	/// Instruction pointer
	Ip,
	/// Tip's address
	Tip,
	/// Interpreter's address
	This,
	/// Result of the last executed instruction.
	/// If not empty, program did not executed to the end.
	Result,
	/// Refers to amount transferred via Spawn or originating call
	Carry(AssetId),
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
	Tip,
}

/// Base XCVM instructions.
/// This set will remain as small as possible, expressiveness must come on `top` of the base
/// instructions.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Instruction<Payload, Account, Assets> {
	/// Transfer some [`Assets`] from the current program to the [`to`] account.
	Transfer {
		to: Destination<Account>,
		assets: Assets,
	},
	/// Arbitrary payload representing a raw call inside the current network.
	///
	/// On picasso, this will be a SCALE encoded dispatch call.
	/// On ethereum, an ethereum ABI encoded call.
	/// On cosmos, a raw json WasmMsg call.
	///
	/// Depending on the network, the payload might be more structured than the base call.
	/// For most of the network in fact, we need to provide the target address along the payload,
	/// which can be encoded inside this single payload.
	Call {
		bindings: Bindings,
		encoded: Payload,
	},
	/// Spawn a sub-program on the target `network`.
	///
	/// The program will be spawned with the desired [`Assets`].
	/// The salt is used to track the program when events are dispatched in the network.
	Spawn {
		network_id: crate::network::NetworkId,
		/// If JSON, than hex encoded non prefixed lower case string.
		/// Different salt allows to split funds into different virtual wallets
		/// So same salt shares assets on set of derived accounts on chains program executes.
		#[serde(serialize_with = "hex::serialize", deserialize_with = "hex::deserialize")]
		#[cfg_attr(feature = "std", schemars(schema_with = "String::json_schema"))]
		#[serde(skip_serializing_if = "Vec::is_empty", default)]
		salt: Vec<u8>,
		assets: Assets,
		program: Program<VecDeque<Self>>,
	},
	Exchange {
		exchange_id: ExchangeId,
		give: Assets,
		want: Assets,
	},
}

/// Error types for late binding operation
#[derive(Clone, Debug, PartialEq)]
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
///   late-binding, the payload would probably look similar to this: `{ "from": "address", "to": ""
///   }`
/// * `bindings`: **Sorted** by index binding index-value pairs.
/// * `binding_data`: Callback function that gives the binding data corresponding to a binding
///   value.
pub fn apply_bindings<'a, E>(
	payload: Vec<u8>,
	bindings: &[(u32, BindingValue)],
	binding_data: impl Fn(&BindingValue) -> Result<Cow<'a, [u8]>, E>,
) -> Result<Vec<u8>, LateBindingError<E>> {
	if bindings.is_empty() {
		return Ok(payload)
	}

	// Estimate the maximum length of the payload.  It’s ok if we don’t get this
	// right.  If our estimate is too large we’re just waste some bytes; if it’s
	// too small we’ll need to reallocate.  We could go through the bindings an
	// calculate their lengths but for now we’re assuming this estimate is
	// enough.
	let this_len = binding_data(&BindingValue::Register(Register::This))
		.map_err(LateBindingError::App)?
		.len();
	let capacity = bindings.len() * this_len + payload.len();
	let mut output = Vec::with_capacity(capacity);

	let mut start = 0;
	for (binding_index, binding) in bindings {
		let binding_index =
			usize::try_from(*binding_index).map_err(|_| LateBindingError::InvalidBinding)?;

		#[allow(clippy::comparison_chain)]
		if binding_index < start {
			// The bindings weren’t ordered by index.
			return Err(LateBindingError::InvalidBinding)
		} else if binding_index > start {
			// Copy literal part from the template payload.
			let literal =
				payload.get(start..binding_index).ok_or(LateBindingError::InvalidBinding)?;
			output.extend_from_slice(literal);
			start = binding_index;
		}

		// Resolve the binding and insert it next.
		let data: Cow<[u8]> = binding_data(binding).map_err(LateBindingError::App)?;
		output.extend_from_slice(&data);
	}

	// Copy remaining part of the template.
	let literal = payload.get(start..).ok_or(LateBindingError::InvalidBinding)?;
	output.extend_from_slice(literal);

	Ok(output)
}

#[cfg(test)]
mod tests {
	use super::*;

	const FOO: BindingValue = BindingValue::Register(Register::This);
	const BAR: BindingValue = BindingValue::Register(Register::Tip);
	const ERR: BindingValue = BindingValue::Register(Register::Ip);

	fn resolver<'a>(binding: &BindingValue) -> Result<Cow<'a, [u8]>, ()> {
		if binding == &FOO {
			Ok(Cow::Borrowed("foo".as_bytes()))
		} else if binding == &BAR {
			Ok(Cow::Owned("bar".as_bytes().to_vec()))
		} else {
			Err(())
		}
	}

	fn apply(
		template: &str,
		bindings: &[(u32, BindingValue)],
	) -> Result<String, LateBindingError<()>> {
		let template = template.as_bytes().to_vec();
		apply_bindings(template, bindings, resolver)
			.map(|payload| String::from_utf8(payload).unwrap())
	}

	#[track_caller]
	fn check_ok(want: &str, template: &str, bindings: &[(u32, BindingValue)]) {
		let got = apply(template, bindings);
		assert_eq!(Ok(want), got.as_deref())
	}

	#[track_caller]
	fn check_err(want: LateBindingError<()>, template: &str, bindings: &[(u32, BindingValue)]) {
		assert_eq!(Err(want), apply(template, bindings))
	}

	#[test]
	fn test_apply_bindings_success() {
		check_ok("", "", &[]);
		check_ok("foo", "", &[(0, FOO.clone())]);
		check_ok("<foo>", "<>", &[(1, FOO.clone())]);
		check_ok("<foobar>", "<>", &[(1, FOO.clone()), (1, BAR.clone())]);
	}

	#[test]
	fn test_apply_bindings_failure() {
		// Index beyond template’s length
		check_err(LateBindingError::InvalidBinding, "", &[(1, FOO.clone())]);
		check_err(LateBindingError::InvalidBinding, "", &[(0, FOO.clone()), (1, BAR.clone())]);
		// Failure in resolution.
		check_err(LateBindingError::App(()), "", &[(0, ERR.clone())]);
		// Bindings not in sorted order.
		check_err(LateBindingError::InvalidBinding, "<>", &[(1, FOO.clone()), (0, BAR.clone())]);
	}
}
