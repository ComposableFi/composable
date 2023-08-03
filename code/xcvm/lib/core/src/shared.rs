use crate::prelude::*;
use cosmwasm_std::{from_binary, to_binary, Binary, CanonicalAddr, StdResult};
use serde::{de::DeserializeOwned, Serialize};

pub type DefaultXCVMInstruction = crate::Instruction<Vec<u8>, CanonicalAddr, crate::Funds>;
pub type DefaultXCVMProgram = crate::Program<VecDeque<DefaultXCVMInstruction>>;
pub type XcPacket = crate::Packet<DefaultXCVMProgram>;
pub type Salt = Vec<u8>;

pub fn encode_base64<T: Serialize>(x: &T) -> StdResult<String> {
	Ok(to_binary(x)?.to_base64())
}

pub fn decode_base64<S: AsRef<str>, T: DeserializeOwned>(encoded: S) -> StdResult<T> {
	from_binary::<T>(&Binary::from_base64(encoded.as_ref())?)
}
