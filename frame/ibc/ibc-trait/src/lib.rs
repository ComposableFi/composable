#![cfg_attr(not(feature = "std"), no_std)]
use codec::{alloc::string::String, Decode, Encode};
use frame_support::{weights::Weight, RuntimeDebug};
use ibc::core::{
	ics02_client::client_type::ClientType,
	ics04_channel::{
		channel::{ChannelEnd, Order},
		msgs::acknowledgement::Acknowledgement,
		packet::Packet,
	},
	ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
};
use ibc_primitives::SendPacketData;
use scale_info::TypeInfo;
use sp_std::{prelude::*, str::FromStr};

#[derive(RuntimeDebug)]
/// Error definition for module
pub enum Error {
	/// Failed to register a new packet
	SendPacketError,
	/// Failed to bind port
	BindPortError,
	/// Failed to intialize a new channel
	ChannelInitError,
	/// Failed to decode a value
	DecodingError,
	/// Failed to decode commitment prefix
	ErrorDecodingPrefix,
	/// Some other error
	Other,
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
/// Captures all parameters needed to initialize a channel
pub struct OpenChannelParams {
	pub order: u8,
	pub connection_id: Vec<u8>,
	pub counterparty_port_id: Vec<u8>,
	/// UTF8 string bytes
	pub version: Vec<u8>,
}

impl TryFrom<&OpenChannelParams> for Order {
	type Error = Error;

	fn try_from(value: &OpenChannelParams) -> Result<Self, Self::Error> {
		match value.order {
			0 => Ok(Order::None),
			1 => Ok(Order::Unordered),
			2 => Ok(Order::Ordered),
			_ => Err(Error::Other),
		}
	}
}

/// Captures the functions modules can use to interact with the ibc pallet
/// Currently allows modules to register packets and crreate channels
pub trait IbcTrait {
	/// Register a packet to be sent
	fn send_packet(data: SendPacketData) -> Result<(), Error>;
	/// Allows a module to open a channel
	fn open_channel(port_id: PortId, channel_end: ChannelEnd) -> Result<ChannelId, Error>;
}

/// Callback Weight
/// This trait must be implemented by module callback handlers to be able to estimate the weight
/// of the callback function.
pub trait CallbackWeight {
	/// Returns the callback weight for the channel open init ibc message
	fn on_chan_open_init(&self) -> Weight;

	/// Returns the callback weight for the channel open try ibc message
	fn on_chan_open_try(&self) -> Weight;

	/// Returns the callback weight for the channel open acknowledgement ibc message
	fn on_chan_open_ack(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight;

	/// Returns the callback weight for the channel open comfirm ibc message
	fn on_chan_open_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight;

	/// Returns the callback weight for the channel close init ibc message
	fn on_chan_close_init(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight;

	/// Returns the callback weight for the channel close confirm ibc message
	fn on_chan_close_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight;

	/// Returns the callback weight for the receive packet ibc message
	fn on_recv_packet(&self, _packet: &Packet) -> Weight;

	/// Returns the callback weight for the packet acknowledgement ibc message
	fn on_acknowledgement_packet(
		&self,
		_packet: &Packet,
		_acknowledgement: &Acknowledgement,
	) -> Weight;

	/// Returns the callback weight for the packet timeout ibc message
	fn on_timeout_packet(&self, packet: &Packet) -> Weight;
}

impl CallbackWeight for () {
	fn on_chan_open_init(&self) -> Weight {
		Weight::MAX
	}

	fn on_chan_open_try(&self) -> Weight {
		Weight::MAX
	}

	fn on_chan_open_ack(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		Weight::MAX
	}

	fn on_chan_open_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		Weight::MAX
	}

	fn on_chan_close_init(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		Weight::MAX
	}

	fn on_chan_close_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		Weight::MAX
	}

	fn on_recv_packet(&self, _packet: &Packet) -> Weight {
		Weight::MAX
	}

	fn on_acknowledgement_packet(
		&self,
		_packet: &Packet,
		_acknowledgement: &Acknowledgement,
	) -> Weight {
		Weight::MAX
	}

	fn on_timeout_packet(&self, _packet: &Packet) -> Weight {
		Weight::MAX
	}
}

/// Get port_id from raw bytes
pub fn port_id_from_bytes(port: Vec<u8>) -> Result<PortId, Error> {
	PortId::from_str(&String::from_utf8(port).map_err(|_| Error::DecodingError)?)
		.map_err(|_| Error::DecodingError)
}

/// Get channel_id from raw bytes
pub fn channel_id_from_bytes(channel: Vec<u8>) -> Result<ChannelId, Error> {
	ChannelId::from_str(&String::from_utf8(channel).map_err(|_| Error::DecodingError)?)
		.map_err(|_| Error::DecodingError)
}

/// Get connection_id from raw bytes
pub fn connection_id_from_bytes(connection: Vec<u8>) -> Result<ConnectionId, Error> {
	ConnectionId::from_str(&String::from_utf8(connection).map_err(|_| Error::DecodingError)?)
		.map_err(|_| Error::DecodingError)
}

/// Get client_id from raw bytes
pub fn client_id_from_bytes(client_id: Vec<u8>) -> Result<ClientId, Error> {
	ClientId::from_str(&String::from_utf8(client_id).map_err(|_| Error::DecodingError)?)
		.map_err(|_| Error::DecodingError)
}

/// Get client_type from raw bytes
pub fn client_type_from_bytes(client_type: Vec<u8>) -> Result<ClientType, Error> {
	ClientType::from_str(&String::from_utf8(client_type).map_err(|_| Error::DecodingError)?)
		.map_err(|_| Error::DecodingError)
}

/// Get trie key by applying the commitment prefix to the path and scale encoding the result
pub fn apply_prefix_and_encode(prefix: &[u8], path: Vec<String>) -> Result<Vec<u8>, Error> {
	let mut key_path = vec![prefix];
	let path = path.iter().map(|val| val.as_bytes()).collect::<Vec<_>>();
	key_path.extend_from_slice(&path);
	Ok(key_path.encode())
}
