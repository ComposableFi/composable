use cosmwasm_std::{from_binary, to_binary, Addr, Binary, Deps, Event, StdError, StdResult};
use cw_xcvm_utils::DefaultXCVMProgram;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use xcvm_core::{BridgeSecurity, Displayed, Funds, InterpreterOrigin, NetworkId};

pub const EVENT_INSTANTIATE: &str = "instantiate";
pub const EVENT_ATTR_CONTRACT_ADDRESS: &str = "_contract_address";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BridgeMsg {
	pub interpreter_origin: InterpreterOrigin,
	pub network_id: NetworkId,
	pub security: BridgeSecurity,
	pub salt: Vec<u8>,
	pub program: DefaultXCVMProgram,
	pub assets: Funds<Displayed<u128>>,
}

pub fn encode_base64<T: Serialize>(x: &T) -> StdResult<String> {
	Ok(to_binary(x)?.to_base64())
}

pub fn decode_base64<S: AsRef<str>, T: DeserializeOwned>(encoded: S) -> StdResult<T> {
	let x = from_binary::<T>(&Binary::from_base64(encoded.as_ref())?)?;
	Ok(x)
}

pub fn parse_instantiated_contract_address(deps: Deps, events: &[Event]) -> StdResult<Addr> {
	let instantiate_event = events
		.iter()
		.find(|event| event.ty == "instantiate")
		.ok_or_else(|| StdError::not_found("instantiate event not found"))?;
	deps.api.addr_validate(
		&instantiate_event
			.attributes
			.iter()
			.find(|attr| &attr.key == "_contract_address")
			.ok_or_else(|| StdError::not_found("_contract_address attribute not found"))?
			.value,
	)
}
