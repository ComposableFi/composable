#![cfg_attr(not(feature = "std"), no_std)]
use ibc::core::{ics05_port::capabilities::PortCapability, ics24_host::identifier::PortId};
use ibc_primitives::SendPacketData;

pub enum Error {
	SendPacketError,
	BindPortError,
}

pub struct OpenChannelParams {}

pub trait IbcTrait {
	fn send_packet(data: SendPacketData) -> Result<(), Error>;
	fn bind_port(port_id: PortId) -> Result<PortCapability, Error>;
}
