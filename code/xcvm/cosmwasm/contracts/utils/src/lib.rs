use cosmwasm_std::{from_binary, to_binary, Binary, StdResult};
use xcvm_core::NetworkId;

pub type UserId = Vec<u8>;

pub fn encode_origin_data(network_id: NetworkId, user_id: &UserId) -> StdResult<String> {
	Ok(to_binary(&(network_id.0, user_id))?.to_base64())
}

pub fn decode_origin_data<S: AsRef<str>>(encoded: S) -> StdResult<(u8, UserId)> {
	from_binary::<(u8, UserId)>(&Binary::from_base64(encoded.as_ref())?)
}
