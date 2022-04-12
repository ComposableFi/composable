#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use ibc::core::{ics05_port::capabilities::PortCapability, ics24_host::identifier::PortId};
use ibc_primitives::SendPacketData;
use scale_info::TypeInfo;
use sp_std::prelude::*;

pub enum Error {
	SendPacketError,
	BindPortError,
}

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct OpenChannelParams {
	state: u8,
	order: u8,
	connection_id: Vec<u8>,
	counterparty_port_id: Vec<u8>,
	version: Vec<u8>,
}

pub trait IbcTrait {
	fn send_packet(data: SendPacketData) -> Result<(), Error>;
	fn bind_port(port_id: PortId) -> Result<PortCapability, Error>;
}
