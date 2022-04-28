#![cfg_attr(not(feature = "std"), no_std)]
use codec::{alloc::string::String, Decode, Encode};
use frame_support::{weights::Weight, RuntimeDebug};
use ibc::core::{
	ics02_client::client_type::ClientType,
	ics04_channel::{channel::ChannelEnd, msgs::acknowledgement::Acknowledgement, packet::Packet},
	ics05_port::capabilities::PortCapability,
	ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
};
use ibc_primitives::SendPacketData;
use scale_info::TypeInfo;
use sp_std::{prelude::*, str::FromStr};

#[derive(RuntimeDebug)]
pub enum Error {
	SendPacketError,
	BindPortError,
	ChannelInitError,
	DecodingError,
	InvalidCapability,
	ErrorDecodingPrefix,
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct OpenChannelParams {
	pub state: u8,
	pub order: u8,
	pub connection_id: Vec<u8>,
	pub counterparty_port_id: Vec<u8>,
	/// UTF8 string bytes
	pub version: Vec<u8>,
}

pub trait IbcTrait {
	fn send_packet(data: SendPacketData) -> Result<(), Error>;
	fn bind_port(port_id: PortId) -> Result<PortCapability, Error>;
	fn open_channel(
		port_id: PortId,
		capability: PortCapability,
		channel_end: ChannelEnd,
	) -> Result<ChannelId, Error>;
}

pub trait CallbackWeight {
	fn on_chan_open_init(&self) -> Weight;

	fn on_chan_open_try(&self) -> Weight;

	fn on_chan_open_ack(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight;

	fn on_chan_open_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight;

	fn on_chan_close_init(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight;

	fn on_chan_close_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight;

	fn on_recv_packet(&self, _packet: &Packet) -> Weight;

	fn on_acknowledgement_packet(
		&self,
		_packet: &Packet,
		_acknowledgement: &Acknowledgement,
	) -> Weight;

	fn on_timeout_packet(&self, packet: &Packet) -> Weight;
}

pub fn port_id_from_bytes(port: Vec<u8>) -> Result<PortId, Error> {
	PortId::from_str(&String::from_utf8(port).map_err(|_| Error::DecodingError)?)
		.map_err(|_| Error::DecodingError)
}

pub fn channel_id_from_bytes(channel: Vec<u8>) -> Result<ChannelId, Error> {
	ChannelId::from_str(&String::from_utf8(channel).map_err(|_| Error::DecodingError)?)
		.map_err(|_| Error::DecodingError)
}

pub fn connection_id_from_bytes(connection: Vec<u8>) -> Result<ConnectionId, Error> {
	ConnectionId::from_str(&String::from_utf8(connection).map_err(|_| Error::DecodingError)?)
		.map_err(|_| Error::DecodingError)
}

pub fn client_id_from_bytes(client_id: Vec<u8>) -> Result<ClientId, Error> {
	ClientId::from_str(&String::from_utf8(client_id).map_err(|_| Error::DecodingError)?)
		.map_err(|_| Error::DecodingError)
}

pub fn client_type_from_bytes(client_type: Vec<u8>) -> Result<ClientType, Error> {
	ClientType::from_str(&String::from_utf8(client_type).map_err(|_| Error::DecodingError)?)
		.map_err(|_| Error::DecodingError)
}

pub fn apply_prefix_and_encode(prefix: &[u8], path: Vec<String>) -> Result<Vec<u8>, Error> {
	let prefix = String::from_utf8(prefix.to_vec()).map_err(|_| Error::ErrorDecodingPrefix)?;
	let mut key_path = vec![prefix.as_bytes()];
	let path = path.iter().map(|val| val.as_bytes()).collect::<Vec<_>>();
	key_path.extend_from_slice(&path);
	Ok(key_path.encode())
}
