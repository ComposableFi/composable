use crate::{prelude::*, AssetId, Displayed};
use cosmwasm_std::{from_binary, to_binary, Binary, CanonicalAddr, StdResult};
use serde::{de::DeserializeOwned, Serialize};

pub type Salt = Vec<u8>;
pub type XcFunds = Vec<(AssetId, Displayed<u128>)>;
pub type XcInstruction = crate::Instruction<Vec<u8>, CanonicalAddr, crate::Funds>;
pub type XcPacket = crate::Packet<XcProgram>;
pub type XcProgram = crate::Program<VecDeque<XcInstruction>>;

pub fn encode_base64<T: Serialize>(x: &T) -> StdResult<String> {
	Ok(to_binary(x)?.to_base64())
}

pub fn decode_base64<S: AsRef<str>, T: DeserializeOwned>(encoded: S) -> StdResult<T> {
	from_binary::<T>(&Binary::from_base64(encoded.as_ref())?)
}
