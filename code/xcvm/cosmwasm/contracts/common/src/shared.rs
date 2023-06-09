use cosmwasm_std::{from_binary, to_binary, Binary, StdResult};
use cw_xc_utils::DefaultXCVMProgram;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use xc_core::{Displayed, Funds, InterpreterOrigin, NetworkId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BridgeMsg {
	pub interpreter_origin: InterpreterOrigin,
	pub network_id: NetworkId,
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
