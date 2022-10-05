use super::{BindingValue, Bindings};
use alloc::{fmt::Debug, string::String, vec, vec::Vec};
use cosmwasm_std::CosmosMsg;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LateCall {
	pub bindings: Bindings,
	pub encoded_call: Vec<u8>,
}

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteMsg<T>
where
	T: Serialize + Clone + Debug,
{
	contract_addr: String,
	/// Serialized ExecuteMsg
	msg: T,
}

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TypedCosmosMsg<T>
where
	T: Serialize + Clone + Debug,
{
	/// Dispatches a call to another contract at a known address (with known ABI).
	///
	/// This is translated to a [MsgExecuteContract](https://github.com/CosmWasm/wasmd/blob/v0.14.0/x/wasm/internal/types/tx.proto#L68-L78).
	/// `sender` is automatically filled with the current contract's address.
	Execute(ExecuteMsg<T>),
}

/// Bindings for the generic `msg` that are done with indices.
///
/// Eg. Let's say we want to do late binding in the `to` field in the
/// following payload and want to put the interpreter's address:
/// `{"from":"helloworld","to":""}`
/// Then the binding is `(26, BindingValue::This)`
pub enum IndexedBinding<T> {
	None(T),
	Some((Bindings, T)),
}

/// Static bindings that are used for typed and fixed types.
/// Eg. `contract_addr` in `ExecuteMsg`
pub enum StaticBinding<T> {
	None(T),
	Some(BindingValue),
}

#[inline]
fn find_key_offset(key: &str, data: &str) -> Option<u32> {
	data.find(key).map(|index| index as u32 + key.len() as u32 + 1)
}

impl LateCall {
	pub fn new(bindings: Bindings, encoded_call: Vec<u8>) -> Self {
		LateCall { bindings, encoded_call }
	}
}

impl LateCall {
	pub fn execute<T: Serialize + Clone + Debug>(
		contract_addr: StaticBinding<String>,
		msg: IndexedBinding<T>,
	) -> Result<Self, ()> {
		// TODO: Check uniqueness
		let execute_msg = ExecuteMsg::<T> {
			contract_addr: match &contract_addr {
				StaticBinding::Some(_) => Default::default(),
				StaticBinding::None(data) => data.clone(),
			},
			msg: match &msg {
				IndexedBinding::Some((_, data)) => data.clone(),
				IndexedBinding::None(data) => data.clone(),
			},
		};

		let serialized_data =
			serde_json::to_string(&TypedCosmosMsg::Execute(execute_msg.clone())).map_err(|_| ())?;

		let mut total_bindings = Bindings::new();

		if let StaticBinding::Some(binding) = contract_addr {
			let offset = find_key_offset("\"contract_addr\"", &serialized_data).ok_or(())?;
			total_bindings.push((offset, binding));
		}

		if let IndexedBinding::Some((bindings, _)) = msg {
			let offset = find_key_offset("\"msg\"", &serialized_data).ok_or(())?;
			for binding in bindings {
				total_bindings.push((offset + binding.0, binding.1));
			}
		}

		Ok(LateCall::new(total_bindings, serialized_data.into()))
	}
}

impl<T> TryInto<CosmosMsg> for TypedCosmosMsg<T>
where
	T: Serialize + Clone + Debug,
{
	type Error = serde_json::Error;
	fn try_into(self) -> Result<CosmosMsg, Self::Error> {
		Ok(match self {
			TypedCosmosMsg::Execute(ExecuteMsg { contract_addr, msg }) =>
				CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
					contract_addr,
					msg: cosmwasm_std::Binary(serde_json::to_vec(&msg)?),
					funds: Vec::new(),
				}),
		})
	}
}

#[test]
fn test_late_binding() {
	#[derive(Debug, Clone, Serialize, Deserialize, Default)]
	struct TestMsg {
		part1: String,
		part2: String,
		part3: String,
	}

	// {"part1":"","part2":"hello","part3":""}

	let test_msg =
		TestMsg { part1: String::new(), part2: String::from("hello"), part3: String::new() };
	let msg = LateCall::execute(
		StaticBinding::Some(BindingValue::Ip),
		IndexedBinding::Some((
			vec![(9, BindingValue::This), (36, BindingValue::Relayer)],
			test_msg.clone(),
		)),
	)
	.unwrap();

	assert_eq!(
		msg.bindings,
		vec![(28, BindingValue::Ip), (46, BindingValue::This), (73, BindingValue::Relayer)]
	);

	assert_eq!(
		msg.encoded_call,
		serde_json::to_vec(&TypedCosmosMsg::Execute(ExecuteMsg {
			contract_addr: String::new(),
			msg: test_msg
		}))
		.unwrap()
	);
}
