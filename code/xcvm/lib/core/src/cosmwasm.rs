use crate::OrderedBindings;

use super::{BindingValue, Bindings};
use alloc::{fmt::Debug, string::String, vec::Vec};
use cosmwasm_std::{BankMsg, Coin, CosmosMsg};
use serde::{Deserialize, Serialize};

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
		/// A human-readbale label for the contract
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
/// Then the binding is `(26, BindingValue::This)`
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
	pub fn bank_send<T: Serialize + Clone + Debug>(
		to_address: StaticBinding<String>,
		amount: Vec<Coin>,
	) -> Result<Self, ()> {
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
	pub fn wasm_update_admin<T: Serialize + Clone + Debug>(
		// Note that we don't use `StaticBinding` here because users can't update
		// any of the binding contracts.
		contract_addr: String,
		admin: StaticBinding<String>,
	) -> Result<Self, ()> {
		let migrate_msg = FlatWasmMsg::<T>::UpdateAdmin {
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
	pub fn wasm_clear_admin<T: Serialize + Clone + Debug>(
		// Note that we don't use `StaticBinding` here because users can't clear
		// any of the binding contracts.
		contract_addr: String,
	) -> Result<Self, ()> {
		let clear_admin_msg = FlatWasmMsg::<T>::ClearAdmin { contract_addr };

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

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::vec;

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
		let msg = LateCall::wasm_execute(
			StaticBinding::Some(BindingValue::Ip),
			IndexedBinding::Some((
				[(9, BindingValue::This), (36, BindingValue::Relayer)].into(),
				test_msg.clone(),
			)),
			Vec::new(),
		)
		.unwrap();

		assert_eq!(
			msg.bindings,
			vec![(36, BindingValue::Ip), (54, BindingValue::This), (81, BindingValue::Relayer)]
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
}
