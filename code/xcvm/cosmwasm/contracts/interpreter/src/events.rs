use cosmwasm_std::{Addr, Event};
use serde::{Deserialize, Serialize};
use xc_core::{service::dex::ExchangeId, shared, InterpreterOrigin, NetworkId, UserId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.exchanged")]
pub struct CvmInterpreterExchanged {
	pub exchange_id: ExchangeId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.execution.started")]
pub struct CvmInterpreterExecutionStarted {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.instantiated")]
pub struct CvmInterpreterInstantiated {
	pub interpreter_origin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.transferred")]
pub struct CvmInterpreterTransferred {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.owner.added")]
pub struct CvmInterpreterOwnerAdded {
	pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.execution.failed")]
pub struct CvmInterpreterExchangeFailed {
	pub reason: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.instruction.spawned")]
pub struct CvmInterpreterInstructionSpawned {
	pub origin_network_id: NetworkId,
	pub origin_user_id: UserId,
}

// beneath is something to be generate by macro

impl CvmInterpreterInstructionSpawned {
	pub fn new(
		origin_network_id: NetworkId,
		origin_user_id: UserId,
		network_id: NetworkId,
	) -> Event {
		Event::new("cvm.interpreter.instruction.spawned")
			.add_attribute(
				"origin_network_id",
				serde_json_wasm::to_string(&interpreter_origin.user_origin.network_id)
					.expect("network id is controlled by us and it is always serde"),
			)
			.add_attribute(
				"origin_user_id",
				serde_json_wasm::to_string(&interpreter_origin.user_origin.user_id)
					.expect("user id is controlled by us and it is always serde"),
			)
			.add_attribute(
				"network_id",
				serde_json_wasm::to_string(&network_id)
					.expect("network id is controlled by us and it is always serde"),
			)
	}
}

impl CvmInterpreterExchangeFailed {
	pub fn new(reason: String) -> Event {
		Event::new("cvm.interpreter.exchange.failed").add_attribute("reason", reason)
	}
}

impl CvmInterpreterOwnerAdded {
	pub fn new(owners: Vec<Addr>) -> Event {
		let mut e = Event::new("cvm.interpreter.owner.added");
		for owner in owners {
			e = e.add_attribute("owner", owner.to_string())
		}
		e
	}
}

impl CvmInterpreterExecutionStarted {
	pub fn new() -> Event {
		Event::new("cvm.interpreter.execution.started")
	}
}

impl CvmInterpreterTransferred {
	pub fn new() -> Event {
		Event::new("cvm.interpreter.transferred")
	}
}

impl CvmInterpreterInstantiated {
	pub fn new(interpreter_origin: &InterpreterOrigin) -> Event {
		Event::new("cvm.interpreter.instantiated").add_attribute(
			"interpreter_origin",
			shared::encode_base64(interpreter_origin).expect("origin is managed by"),
		)
	}
}

impl CvmInterpreterExchanged {
	pub fn new(exchange_id: ExchangeId) -> Event {
		Event::new("cvm.interpreter.exchanged")
			.add_attribute("exchange_id", exchange_id.to_string())
	}
}
