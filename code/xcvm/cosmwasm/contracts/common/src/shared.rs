use cosmwasm_std::{from_binary, to_binary, Binary, StdResult};
use serde::{de::DeserializeOwned, Serialize};

pub fn encode_base64<T: Serialize>(x: &T) -> StdResult<String> {
	Ok(to_binary(x)?.to_base64())
}

pub fn decode_base64<S: AsRef<str>, T: DeserializeOwned>(encoded: S) -> StdResult<T> {
	let x = from_binary::<T>(&Binary::from_base64(encoded.as_ref())?)?;
	Ok(x)
}
