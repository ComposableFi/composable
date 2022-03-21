use super::*;
use ibc::{
	core::{
		ics04_channel::packet::{Packet, Sequence},
		ics05_port::capabilities::Capability,
		ics24_host::identifier::{ChannelId, PortId},
	},
	timestamp::Timestamp,
};
use scale_info::prelude::string::ToString;

pub struct SendPacketData {
	pub data: Vec<u8>,
	pub timeout_height_offset: u64,
	/// This value should be represent nano seconds
	pub timeout_timestamp_offset: u64,
	pub capability: Capability,
	pub port_id: Vec<u8>,
	pub channel_id: Vec<u8>,
	pub dest_port_id: Vec<u8>,
	pub dest_channel_id: Vec<u8>,
}
#[derive(codec::Encode, codec::Decode, Clone)]
pub struct OffchainPacketType {
	sequence: u64,
	source_port: Vec<u8>,
	source_channel: Vec<u8>,
	destination_port: Vec<u8>,
	destination_channel: Vec<u8>,
	data: Vec<u8>,
	timeout_height: (u64, u64),
	timeout_timestamp: u64,
}

impl From<OffchainPacketType> for Packet {
	fn from(packet: OffchainPacketType) -> Self {
		Self {
			sequence: Sequence::from(packet.sequence),
			source_port: PortId::from_str(
				&String::from_utf8(packet.source_port).unwrap_or_default(),
			)
			.unwrap_or_default(),
			source_channel: ChannelId::from_str(
				&String::from_utf8(packet.source_channel).unwrap_or_default(),
			)
			.unwrap_or_default(),
			destination_port: PortId::from_str(
				&String::from_utf8(packet.destination_port).unwrap_or_default(),
			)
			.unwrap_or_default(),
			destination_channel: ChannelId::from_str(
				&String::from_utf8(packet.destination_channel).unwrap_or_default(),
			)
			.unwrap_or_default(),
			data: packet.data,
			timeout_height: ibc::Height::new(packet.timeout_height.0, packet.timeout_height.1),
			timeout_timestamp: Timestamp::from_nanoseconds(packet.timeout_timestamp)
				.unwrap_or_default(),
		}
	}
}

impl From<Packet> for OffchainPacketType {
	fn from(packet: Packet) -> Self {
		Self {
			sequence: packet.sequence.into(),
			source_port: packet.source_port.to_string().as_bytes().to_vec(),
			source_channel: packet.source_channel.to_string().as_bytes().to_vec(),
			destination_port: packet.destination_port.to_string().as_bytes().to_vec(),
			destination_channel: packet.destination_channel.to_string().as_bytes().to_vec(),
			data: packet.data,
			timeout_height: (
				packet.timeout_height.revision_number,
				packet.timeout_height.revision_height,
			),
			timeout_timestamp: packet.timeout_timestamp.nanoseconds(),
		}
	}
}

pub trait SendPacketTrait<T: Config> {
	fn send_packet(data: SendPacketData) -> Result<(), Error<T>>;
}
