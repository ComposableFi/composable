//! XCVM SDK for CosmWasm
//!
//! # Introduction
//!
//! XCVM uses what we call `late bindings` which gives users ability to,
//! specify addresses that are not known to them yet. For example, an
//! interpreter instance is created upon execution. If so, how will users
//! transfer funds to interpreter without knowing their address? Although
//! we solved this problem in our `pallet-cosmwasm` by deterministically
//! calculating contract addresses prior to execution, we cannot guarantee
//! this on every chain that XCVM is supported. And also, late binding supports
//! some non-address bindings as well. Hence, `late bindings` can be used for
//! situations like this.
//!
//! # Examples
//!
//! We have two types of bindings, `IndexedBinding` and `StaticBinding`.
//! `StaticBinding` is used for the fields with static indices.a
//!
//! Let's assume that we wan't to send `WasmMsg::Execute`:
//! ```json
//! {
//! 	"contract_addr": "",
//! 	"msg": "SOME PAYLOAD",
//! 	"funds": []
//! }
//! ```
//! In this case, we know that `contract_addr` is a static `String`, not
//! a dynamic field like the payload `msg`. Users don't have to know the index
//! of the `contract_addr` in the serialized `msg`. Hence, we use `StaticBinding`
//! for fields like this.
//!
//! But it is totally up to users what to send in `msg` field, therefore they need to
//! provide indices by themselves. But again for simplicity's sake, indices are provided
//! by users relative to the payload, not the whole message.
//!
//! As a complete example, let's say that I want to use `cw20` contract of `PICA` and
//! send the interpreter some coins. First, I need to find the indices in the payload.
//! ```json
//! {"recipient":"","amount":"10000"}
//! ```
//! Index of the value of `recipient` is `13` and the binding that I want to use is
//! `BindingValue::Register(Register::This)`, which is the interpreter.
//!
//! And the contract that I want to use is `cw20` for `PICA`, which is `BindingValue::Asset(1)`.
//! Note that `1` is the identifier of the asset `PICA`. Then, users will call
//! `WasmMsg::Execute` wrapper `wasm_execute` to create the correct payload to pass to
//! use in the `Call` instruction:
//! ```ignore
//! let payload_bindings: OrderedBindings = [(13, BindingValue::Register(Register::This))].into();
//! let cw20_transfer_msg = Cw20ExecuteMsg::Transfer {
//!     // Make sure to leave fields that uses late-bindings empty
//! 	recipient: String::new(),
//!     amount: 10000,
//! };
//!
//! let payload = LateCall::wasm_execute(
//! 	StaticBinding::Some(BindingValue::Asset(1)),
//!     IndexedBinding::Some((payload_bindings, cw20_transfer_msg)),
//!     Vec::new()
//! );
//! ```
//! Note that if you don't need `IndexedBinding` or `StaticBinding`, you can always use `None`
//! variants of both of them.

use super::{BindingValue, Bindings};
use crate::{Bridge, OrderedBindings};
use alloc::{fmt::Debug, string::String, vec, vec::Vec};
use cosmwasm_std::{BankMsg, Coin, CosmosMsg};
use cw_storage_plus::{Key, PrimaryKey};
use serde::{Deserialize, Serialize};

/// Make `Bridge` usable in `cw-storage-plus`
impl<'a> PrimaryKey<'a> for Bridge {
	type Prefix = u8;
	type SubPrefix = ();
	type Suffix = u8;
	type SuperSuffix = (u8, u8);

	fn key(&self) -> Vec<Key> {
		vec![Key::Val16([self.security as u8, self.protocol as u8])]
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LateCall {
	pub bindings: Bindings,
	pub encoded_call: Vec<u8>,
}

/// Flat version of `CosmosMsg` that stores typed and unmodified
/// payloads.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FlatCosmosMsg<T>
where
	T: Serialize + Clone + Debug,
{
	// Flat inner type is not necessary since `BankMsg` doesn't have a payload
	Bank(BankMsg),
	Wasm(FlatWasmMsg<T>),
}

/// The message types of the wasm module.
///
/// See https://github.com/CosmWasm/wasmd/blob/v0.14.0/x/wasm/internal/types/tx.proto
#[non_exhaustive]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FlatWasmMsg<T>
where
	T: Serialize + Clone + Debug,
{
	/// Dispatches a call to another contract at a known address (with known ABI).
	///
	/// This is translated to a [MsgExecuteContract](https://github.com/CosmWasm/wasmd/blob/v0.14.0/x/wasm/internal/types/tx.proto#L68-L78).
	/// `sender` is automatically filled with the current contract's address.
	Execute {
		contract_addr: String,
		/// msg is the json-encoded ExecuteMsg struct (as raw Binary)
		msg: T,
		funds: Vec<Coin>,
	},
	/// Instantiates a new contracts from previously uploaded Wasm code.
	///
	/// This is translated to a [MsgInstantiateContract](https://github.com/CosmWasm/wasmd/blob/v0.16.0-alpha1/x/wasm/internal/types/tx.proto#L47-L61).
	/// `sender` is automatically filled with the current contract's address.
	Instantiate {
		admin: Option<String>,
		code_id: u64,
		/// msg is the JSON-encoded InstantiateMsg struct (as raw Binary)
		msg: T,
		funds: Vec<Coin>,
		/// A human-readable label for the contract
		label: String,
	},
	/// Migrates a given contracts to use new wasm code. Passes a MigrateMsg to allow us to
	/// customize behavior.
	///
	/// Only the contract admin (as defined in wasmd), if any, is able to make this call.
	///
	/// This is translated to a [MsgMigrateContract](https://github.com/CosmWasm/wasmd/blob/v0.14.0/x/wasm/internal/types/tx.proto#L86-L96).
	/// `sender` is automatically filled with the current contract's address.
	Migrate {
		contract_addr: String,
		/// the code_id of the new logic to place in the given contract
		new_code_id: u64,
		/// msg is the json-encoded MigrateMsg struct that will be passed to the new code
		msg: T,
	},
	/// Sets a new admin (for migrate) on the given contract.
	/// Fails if this contract is not currently admin of the target contract.
	UpdateAdmin { contract_addr: String, admin: String },
	/// Clears the admin on the given contract, so no more migration possible.
	/// Fails if this contract is not currently admin of the target contract.
	ClearAdmin { contract_addr: String },
}

/// Bindings for the generic `msg` that are done with indices.
///
/// Eg. Let's say we want to do late binding in the `to` field in the
/// following payload and want to put the interpreter's address:
/// `{"from":"helloworld","to":""}`
/// Then the binding is `(26, BindingValue::Register(Register::This))`
pub enum IndexedBinding<T> {
	None(T),
	Some((OrderedBindings, T)),
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
	/// Wrapper for `CosmosMsg::Bank(BankMsg::Send)`
	pub fn bank_send(to_address: StaticBinding<String>, amount: Vec<Coin>) -> Result<Self, ()> {
		let send_msg = BankMsg::Send {
			to_address: match &to_address {
				StaticBinding::Some(_) => Default::default(),
				StaticBinding::None(data) => data.clone(),
			},
			amount,
		};

		let serialized_data =
			serde_json::to_string(&FlatCosmosMsg::<()>::Bank(send_msg.clone())).map_err(|_| ())?;

		let mut total_bindings = Bindings::new();

		if let StaticBinding::Some(binding) = to_address {
			let offset = find_key_offset("\"to_address\"", &serialized_data).ok_or(())?;
			total_bindings.push((offset, binding));
		}

		Ok(LateCall::new(total_bindings, serialized_data.into()))
	}

	/// Wrapper for `CosmosMsg::Bank(BankMsg::Burn)`
	pub fn bank_burn(amount: Vec<Coin>) -> Result<Self, ()> {
		Ok(LateCall::new(
			Bindings::new(),
			serde_json::to_vec(&FlatCosmosMsg::<()>::Bank(BankMsg::Burn { amount }))
				.map_err(|_| ())?,
		))
	}

	/// Wrapper for `CosmosMsg::Wasm(WasmMsg::Execute)`
	pub fn wasm_execute<T: Serialize + Clone + Debug>(
		contract_addr: StaticBinding<String>,
		msg: IndexedBinding<T>,
		funds: Vec<Coin>,
	) -> Result<Self, ()> {
		let execute_msg = FlatWasmMsg::<T>::Execute {
			contract_addr: match &contract_addr {
				StaticBinding::Some(_) => Default::default(),
				StaticBinding::None(data) => data.clone(),
			},
			msg: match &msg {
				IndexedBinding::Some((_, data)) => data.clone(),
				IndexedBinding::None(data) => data.clone(),
			},
			funds,
		};

		let serialized_data =
			serde_json::to_string(&FlatCosmosMsg::Wasm(execute_msg.clone())).map_err(|_| ())?;

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

	/// Wrapper for `CosmosMsg::Wasm(WasmMsg::Instantiate)`
	pub fn wasm_instantiate<T: Serialize + Clone + Debug>(
		admin: Option<StaticBinding<String>>,
		code_id: u64,
		msg: IndexedBinding<T>,
		funds: Vec<Coin>,
		label: String,
	) -> Result<Self, ()> {
		let instantiate_msg = FlatWasmMsg::<T>::Instantiate {
			admin: match &admin {
				Some(StaticBinding::Some(_)) => Some(Default::default()),
				Some(StaticBinding::None(data)) => Some(data.clone()),
				_ => None,
			},
			code_id,
			msg: match &msg {
				IndexedBinding::Some((_, data)) => data.clone(),
				IndexedBinding::None(data) => data.clone(),
			},
			funds,
			label,
		};

		let serialized_data =
			serde_json::to_string(&FlatCosmosMsg::Wasm(instantiate_msg.clone())).map_err(|_| ())?;

		let mut total_bindings = Bindings::new();

		if let Some(StaticBinding::Some(binding)) = admin {
			let offset = find_key_offset("\"admin\"", &serialized_data).ok_or(())?;
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

	/// Wrapper for `CosmosMsg::Wasm(WasmMsg::Migrate)`
	pub fn wasm_migrate<T: Serialize + Clone + Debug>(
		// Note that we don't use `StaticBinding` here because users can't migrate
		// any of the binding contracts.
		contract_addr: String,
		new_code_id: u64,
		msg: IndexedBinding<T>,
	) -> Result<Self, ()> {
		let migrate_msg = FlatWasmMsg::<T>::Migrate {
			contract_addr,
			new_code_id,
			msg: match &msg {
				IndexedBinding::Some((_, data)) => data.clone(),
				IndexedBinding::None(data) => data.clone(),
			},
		};

		let serialized_data =
			serde_json::to_string(&FlatCosmosMsg::Wasm(migrate_msg.clone())).map_err(|_| ())?;

		let mut total_bindings = Bindings::new();

		if let IndexedBinding::Some((bindings, _)) = msg {
			let offset = find_key_offset("\"msg\"", &serialized_data).ok_or(())?;
			for binding in bindings {
				total_bindings.push((offset + binding.0, binding.1));
			}
		}

		Ok(LateCall::new(total_bindings, serialized_data.into()))
	}

	/// Wrapper for `CosmosMsg::Wasm(WasmMsg::UpdateAdmin)`
	pub fn wasm_update_admin(
		// Note that we don't use `StaticBinding` here because users can't update
		// any of the binding contracts.
		contract_addr: String,
		admin: StaticBinding<String>,
	) -> Result<Self, ()> {
		let migrate_msg = FlatWasmMsg::<()>::UpdateAdmin {
			contract_addr,
			admin: match &admin {
				StaticBinding::Some(_) => Default::default(),
				StaticBinding::None(data) => data.clone(),
			},
		};

		let serialized_data =
			serde_json::to_string(&FlatCosmosMsg::Wasm(migrate_msg.clone())).map_err(|_| ())?;

		let mut total_bindings = Bindings::new();

		if let StaticBinding::Some(binding) = admin {
			let offset = find_key_offset("\"admin\"", &serialized_data).ok_or(())?;
			total_bindings.push((offset, binding));
		}

		Ok(LateCall::new(total_bindings, serialized_data.into()))
	}

	/// Wrapper for `CosmosMsg::Wasm(WasmMsg::ClearAdmin)`
	pub fn wasm_clear_admin(
		// Note that we don't use `StaticBinding` here because users can't clear
		// any of the binding contracts.
		contract_addr: String,
	) -> Result<Self, ()> {
		let clear_admin_msg = FlatWasmMsg::<()>::ClearAdmin { contract_addr };

		let serialized_data =
			serde_json::to_string(&FlatCosmosMsg::Wasm(clear_admin_msg)).map_err(|_| ())?;

		Ok(LateCall::new(Bindings::new(), serialized_data.into()))
	}
}

impl<T> TryInto<CosmosMsg> for FlatCosmosMsg<T>
where
	T: Serialize + Clone + Debug,
{
	type Error = serde_json::Error;
	fn try_into(self) -> Result<CosmosMsg, Self::Error> {
		let m = match self {
			FlatCosmosMsg::Wasm(FlatWasmMsg::Execute { contract_addr, msg, funds }) =>
				CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
					contract_addr,
					msg: cosmwasm_std::Binary(serde_json::to_vec(&msg)?),
					funds,
				}),
			FlatCosmosMsg::Wasm(FlatWasmMsg::Instantiate { admin, code_id, msg, funds, label }) =>
				CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Instantiate {
					admin,
					code_id,
					msg: cosmwasm_std::Binary(serde_json::to_vec(&msg)?),
					funds,
					label,
				}),
			FlatCosmosMsg::Wasm(FlatWasmMsg::Migrate { contract_addr, new_code_id, msg }) =>
				CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Migrate {
					contract_addr,
					new_code_id,
					msg: cosmwasm_std::Binary(serde_json::to_vec(&msg)?),
				}),
			FlatCosmosMsg::Wasm(FlatWasmMsg::UpdateAdmin { contract_addr, admin }) =>
				CosmosMsg::Wasm(cosmwasm_std::WasmMsg::UpdateAdmin { contract_addr, admin }),
			FlatCosmosMsg::Wasm(FlatWasmMsg::ClearAdmin { contract_addr }) =>
				CosmosMsg::Wasm(cosmwasm_std::WasmMsg::ClearAdmin { contract_addr }),
			FlatCosmosMsg::Bank(msg) => CosmosMsg::Bank(msg),
		};
		Ok(m)
	}
}

// #[cfg(test)]
/*
mod tests {
	use super::*;
	use alloc::vec;

	#[derive(Debug, Clone, Serialize, Deserialize, Default)]
	struct TestMsg {
		part1: String,
		part2: String,
		part3: String,
	}

	fn create_dummy_data() -> (TestMsg, IndexedBinding<TestMsg>) {
		// {"part1":"","part2":"hello","part3":""}
		let test_msg =
			TestMsg { part1: String::new(), part2: String::from("hello"), part3: String::new() };

		(
			test_msg.clone(),
			IndexedBinding::Some((
				[
					(9, BindingValue::Register(Register::This)),
					(36, BindingValue::Register(Register::Relayer)),
				]
				.into(),
				test_msg,
			)),
		)
	}

	#[test]
	fn test_execute() {
		let (test_msg, payload_bindings) = create_dummy_data();
		let msg = LateCall::wasm_execute(
			StaticBinding::Some(BindingValue::Register(Register::Ip)),
			payload_bindings,
			Vec::new(),
		)
		.unwrap();

		assert_eq!(
			msg.bindings,
			vec![
				(36, BindingValue::Register(Register::Ip)),
				(54, BindingValue::Register(Register::This)),
				(81, BindingValue::Register(Register::Relayer))
			]
		);

		assert_eq!(
			msg.encoded_call,
			serde_json::to_vec(&FlatCosmosMsg::Wasm(FlatWasmMsg::Execute {
				contract_addr: String::new(),
				msg: test_msg,
				funds: Vec::new()
			}))
			.unwrap()
		);
	}

	#[test]
	fn test_instantiate() {
		let (test_msg, payload_bindings) = create_dummy_data();
		let msg = LateCall::wasm_instantiate(
			Some(StaticBinding::Some(BindingValue::Asset(1))),
			1,
			payload_bindings,
			Vec::new(),
			"cool label".into(),
		)
		.unwrap();

		assert_eq!(
			msg.bindings,
			vec![
				(32, BindingValue::Asset(1)),
				(62, BindingValue::Register(Register::This)),
				(89, BindingValue::Register(Register::Relayer)),
			]
		);

		assert_eq!(
			msg.encoded_call,
			serde_json::to_vec(&FlatCosmosMsg::Wasm(FlatWasmMsg::Instantiate {
				admin: Some(Default::default()),
				code_id: 1,
				msg: test_msg,
				funds: Vec::new(),
				label: "cool label".into()
			}))
			.unwrap()
		);
	}

	#[test]
	fn test_migrate() {
		let (test_msg, payload_bindings) = create_dummy_data();
		let msg = LateCall::wasm_migrate("migrate_addr".into(), 2, payload_bindings).unwrap();

		assert_eq!(msg.bindings, vec![(82, BindingValue::This), (109, BindingValue::Relayer)]);

		assert_eq!(
			msg.encoded_call,
			serde_json::to_vec(&FlatCosmosMsg::Wasm(FlatWasmMsg::Migrate {
				contract_addr: "migrate_addr".into(),
				new_code_id: 2,
				msg: test_msg
			}))
			.unwrap()
		);
	}

	#[test]
	fn test_update_admin() {
		let msg = LateCall::wasm_update_admin(
			"contract_addr".into(),
			StaticBinding::Some(BindingValue::This),
		)
		.unwrap();

		assert_eq!(msg.bindings, vec![(65, BindingValue::This)]);

		assert_eq!(
			msg.encoded_call,
			serde_json::to_vec(&FlatCosmosMsg::<()>::Wasm(FlatWasmMsg::UpdateAdmin {
				contract_addr: "contract_addr".into(),
				admin: Default::default()
			}))
			.unwrap()
		);
	}

	#[test]
	fn test_clear_admin() {
		let msg = LateCall::wasm_clear_admin("contract_addr".into()).unwrap();

		assert!(msg.bindings.is_empty());

		assert_eq!(
			msg.encoded_call,
			serde_json::to_vec(&FlatCosmosMsg::<()>::Wasm(FlatWasmMsg::ClearAdmin {
				contract_addr: "contract_addr".into(),
			}))
			.unwrap()
		);
	}

	#[test]
	fn test_bank_send() {
		let msg = LateCall::bank_send(StaticBinding::Some(BindingValue::This), Default::default())
			.unwrap();

		assert_eq!(msg.bindings, vec![(30, BindingValue::This)]);

		assert_eq!(
			msg.encoded_call,
			serde_json::to_vec(&FlatCosmosMsg::<()>::Bank(BankMsg::Send {
				to_address: Default::default(),
				amount: Default::default()
			}))
			.unwrap()
		);
	}

	#[test]
	fn test_bank_burn() {
		let msg = LateCall::bank_burn(Default::default()).unwrap();

		assert!(msg.bindings.is_empty());

		assert_eq!(
			msg.encoded_call,
			serde_json::to_vec(&FlatCosmosMsg::<()>::Bank(BankMsg::Burn {
				amount: Default::default()
			}))
			.unwrap()
		);
	}
}
*/
