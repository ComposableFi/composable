use cosmwasm_std::{from_binary, to_binary, Binary, StdResult};
use xcvm_core::{NetworkId, UserId, UserOrigin};

pub fn encode_origin_data(user_origin: UserOrigin) -> StdResult<String> {
	Ok(to_binary(&(user_origin.network_id, user_origin.user_id))?.to_base64())
}

pub fn decode_origin_data<S: AsRef<str>>(encoded: S) -> StdResult<UserOrigin> {
	let (network_id, user_id) =
		from_binary::<(NetworkId, UserId)>(&Binary::from_base64(encoded.as_ref())?)?;
	Ok(UserOrigin { network_id, user_id })
}
