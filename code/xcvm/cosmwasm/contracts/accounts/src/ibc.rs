use crate::error::{ContractError, Result};
use cosmwasm_std::Binary;
use parity_scale_codec::Decode;

pub(crate) fn decode<T: Decode>(data: Binary) -> Result<T> {
	T::decode(&mut data.as_slice()).map_err(|_| ContractError::InvalidPacket)
}
