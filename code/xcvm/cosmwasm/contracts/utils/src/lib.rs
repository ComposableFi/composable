use cosmwasm_std::{from_binary, to_binary, Binary, CanonicalAddr, StdResult};
use std::collections::VecDeque;
use xcvm_core::{Funds, NetworkId, UserId, UserOrigin};

pub type DefaultXCVMInstruction = xcvm_core::Instruction<NetworkId, Vec<u8>, CanonicalAddr, Funds>;
pub type DefaultXCVMProgram = xcvm_core::Program<VecDeque<DefaultXCVMInstruction>>;
pub type DefaultXCVMPacket = xcvm_core::Packet<DefaultXCVMProgram>;

pub fn encode_origin_data(user_origin: UserOrigin) -> StdResult<String> {
	Ok(to_binary(&(user_origin.network_id, user_origin.user_id))?.to_base64())
}

pub fn decode_origin_data<S: AsRef<str>>(encoded: S) -> StdResult<UserOrigin> {
	let (network_id, user_id) =
		from_binary::<(NetworkId, UserId)>(&Binary::from_base64(encoded.as_ref())?)?;
	Ok(UserOrigin { network_id, user_id })
}
