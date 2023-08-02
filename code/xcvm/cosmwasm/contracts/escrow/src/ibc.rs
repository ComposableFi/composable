use crate::error::{ContractError, Result};
use cosmwasm_std::{Binary, IbcMsg, IbcTimeout, IbcTimeoutBlock};
use parity_scale_codec::{Decode, Encode};

pub(crate) fn make_message(packet: &crate::msg::accounts::Packet) -> IbcMsg {
	IbcMsg::from(IbcMsg::SendPacket {
		channel_id: String::from("XXX"),
		data: cosmwasm_std::Binary::from(packet.encode()),
		// TODO: should be a parameter or configuration
		timeout: IbcTimeout::with_block(IbcTimeoutBlock { revision: 0, height: 10000 }),
	})
}

pub(crate) fn decode<T: Decode>(data: Binary) -> Result<T> {
	T::decode(&mut data.as_slice()).map_err(|_| ContractError::InvalidIbcPacket)
}
