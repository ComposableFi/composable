//! Relayer events.
use ibc::{core::ics04_channel::packet::Packet, events::IbcEvent as RawIbcEvent};
use ibc_runtime_api::IbcRuntimeApi;
use pallet_ibc::events::IbcEvent;
use sc_client_api::HeaderBackend;
use sp_api::{ApiRef, BlockId, BlockT, ProvideRuntimeApi};

/// Filter out none relayer events and modify
/// Fetch actual packet and acknowledgements from off chain storage and modify packets
pub fn filter_map_pallet_event<C, Block>(
	at: &BlockId<Block>,
	api: &ApiRef<<C as ProvideRuntimeApi<Block>>::Api>,
	ev: IbcEvent,
) -> Option<RawIbcEvent>
where
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: IbcRuntimeApi<Block>,
	Block: BlockT,
{
	let mut event: RawIbcEvent = ev.try_into().ok()?;
	match &mut event {
		RawIbcEvent::SendPacket(ev) => {
			let channel_id = ev.src_channel_id();
			let port_id = ev.src_port_id();
			let sequence = u64::from(ev.packet.sequence);
			let packets: Vec<ibc_primitives::OffchainPacketType> = api
				.query_packets(
					at,
					channel_id.to_string().as_bytes().to_vec(),
					port_id.as_bytes().to_vec(),
					vec![sequence],
				)
				.ok()
				.flatten()?;
			let packet = packets.get(0)?.clone();
			let packet: Packet = packet.into();
			ev.packet = packet;
			Some(event)
		},
		RawIbcEvent::ReceivePacket(ev) => {
			let channel_id = ev.src_channel_id();
			let port_id = ev.src_port_id();
			let sequence = u64::from(ev.packet.sequence);
			let packets: Vec<ibc_primitives::OffchainPacketType> = api
				.query_packets(
					at,
					channel_id.to_string().as_bytes().to_vec(),
					port_id.as_bytes().to_vec(),
					vec![sequence],
				)
				.ok()
				.flatten()?;
			let packet = packets.get(0)?.clone();
			let packet: Packet = packet.into();
			ev.packet = packet;
			Some(event)
		},
		RawIbcEvent::WriteAcknowledgement(ev) => {
			let channel_id = ev.src_channel_id();
			let port_id = ev.src_port_id();
			let sequence = u64::from(ev.packet.sequence);
			let packets: Vec<ibc_primitives::OffchainPacketType> = api
				.query_packets(
					at,
					channel_id.to_string().as_bytes().to_vec(),
					port_id.as_bytes().to_vec(),
					vec![sequence],
				)
				.ok()
				.flatten()?;
			let acks: Vec<Vec<u8>> = api
				.query_acknowledgements(
					at,
					channel_id.to_string().as_bytes().to_vec(),
					port_id.as_bytes().to_vec(),
					vec![sequence],
				)
				.ok()
				.flatten()?;
			let packet = packets.get(0)?.clone();
			let packet: Packet = packet.into();
			let ack = acks.get(0)?.clone();
			ev.packet = packet;
			ev.ack = ack;
			Some(event)
		},
		RawIbcEvent::AcknowledgePacket(ev) => {
			let channel_id = ev.src_channel_id();
			let port_id = ev.src_port_id();
			let sequence = u64::from(ev.packet.sequence);
			let packets: Vec<ibc_primitives::OffchainPacketType> = api
				.query_packets(
					at,
					channel_id.to_string().as_bytes().to_vec(),
					port_id.as_bytes().to_vec(),
					vec![sequence],
				)
				.ok()
				.flatten()?;
			let packet = packets.get(0)?.clone();
			let packet: Packet = packet.into();
			ev.packet = packet;
			Some(event)
		},
		RawIbcEvent::TimeoutPacket(ev) => {
			let channel_id = ev.src_channel_id();
			let port_id = ev.src_port_id();
			let sequence = u64::from(ev.packet.sequence);
			let packets: Vec<ibc_primitives::OffchainPacketType> = api
				.query_packets(
					at,
					channel_id.to_string().as_bytes().to_vec(),
					port_id.as_bytes().to_vec(),
					vec![sequence],
				)
				.ok()
				.flatten()?;
			let packet = packets.get(0)?.clone();
			let packet: Packet = packet.into();
			ev.packet = packet;
			Some(event)
		},
		RawIbcEvent::TimeoutOnClosePacket(ev) => {
			let channel_id = ev.src_channel_id();
			let port_id = ev.src_port_id();
			let sequence = u64::from(ev.packet.sequence);
			let packets: Vec<ibc_primitives::OffchainPacketType> = api
				.query_packets(
					at,
					channel_id.to_string().as_bytes().to_vec(),
					port_id.as_bytes().to_vec(),
					vec![sequence],
				)
				.ok()
				.flatten()?;
			let packet = packets.get(0)?.clone();
			let packet: Packet = packet.into();
			ev.packet = packet;
			Some(event)
		},
		RawIbcEvent::NewBlock(_) |
		RawIbcEvent::AppModule(_) |
		RawIbcEvent::Empty(_) |
		RawIbcEvent::ChainError(_) => None,
		_ => Some(event),
	}
}
