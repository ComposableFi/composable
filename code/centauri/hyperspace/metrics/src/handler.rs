use crate::data::Metrics;
use ibc::{
	core::{
		ics04_channel::{
			events::{TimeoutOnClosePacket, TimeoutPacket},
			packet::{Packet, Sequence},
		},
		ics24_host::identifier::{ChannelId, PortId},
	},
	events::IbcEvent,
};
use ibc_proto::google::protobuf::Any;
use prometheus::{Histogram, Registry};
use std::{
	cell::Cell,
	collections::HashMap,
	sync::{Arc, Mutex},
	time::Instant,
};

#[derive(Eq, PartialEq, Hash)]
pub struct PacketId {
	pub sequence: Sequence,
	pub destination_channel: ChannelId,
	pub destination_port: PortId,
}

impl From<Packet> for PacketId {
	fn from(packet: Packet) -> Self {
		Self {
			sequence: packet.sequence,
			destination_channel: packet.destination_channel,
			destination_port: packet.destination_port,
		}
	}
}

pub type PacketMap = Arc<Mutex<HashMap<PacketId, Instant>>>;

pub struct MetricsHandler {
	registry: Registry,
	metrics: Metrics,

	last_sent_packet_time: PacketMap,
	last_sent_acknowledgment_time: PacketMap,
	last_sent_timeout_packet_time: PacketMap,
	last_update_client_time: Cell<Option<Instant>>,

	counterparty_last_sent_packet_time: Option<PacketMap>,
	counterparty_last_sent_acknowledgment_time: Option<PacketMap>,
	counterparty_last_sent_timeout_packet_time: Option<PacketMap>,
}

impl MetricsHandler {
	pub fn new(registry: Registry, metrics: Metrics) -> Self {
		Self {
			registry,
			metrics,
			last_sent_packet_time: Arc::new(Mutex::new(HashMap::new())),
			last_sent_acknowledgment_time: Arc::new(Mutex::new(HashMap::new())),
			last_sent_timeout_packet_time: Arc::new(Mutex::new(HashMap::new())),
			last_update_client_time: Cell::new(None),
			counterparty_last_sent_packet_time: None,
			counterparty_last_sent_acknowledgment_time: None,
			counterparty_last_sent_timeout_packet_time: None,
		}
	}

	pub async fn handle_events(&mut self, events: &[IbcEvent]) -> anyhow::Result<()> {
		for event in events {
			match event {
				IbcEvent::SendPacket(packet) => {
					self.metrics.number_of_received_send_packets.inc();
					let packet_id = packet.packet.clone().into();
					self.last_sent_packet_time.lock().unwrap().insert(packet_id, Instant::now());
				},
				IbcEvent::ReceivePacket(packet) => {
					self.metrics.number_of_received_receive_packets.inc();
					self.observe_last_packet_time(
						&packet.packet,
						&self.counterparty_last_sent_packet_time,
						&self.metrics.sent_packet_time,
					);
				},
				IbcEvent::WriteAcknowledgement(packet) => {
					let packet_id = packet.packet.clone().into();
					self.last_sent_acknowledgment_time
						.lock()
						.unwrap()
						.insert(packet_id, Instant::now());
				},
				IbcEvent::AcknowledgePacket(packet) => {
					self.metrics.number_of_received_acknowledge_packets.inc();
					self.observe_last_packet_time(
						&packet.packet,
						&self.counterparty_last_sent_acknowledgment_time,
						&self.metrics.sent_acknowledgment_time,
					);
				},
				IbcEvent::TimeoutPacket(TimeoutPacket { packet, .. }) |
				IbcEvent::TimeoutOnClosePacket(TimeoutOnClosePacket { packet, .. }) => {
					self.metrics.number_of_received_timeouts.inc();
					self.observe_last_packet_time(
						packet,
						&self.counterparty_last_sent_timeout_packet_time,
						&self.metrics.sent_timeout_packet_time,
					);
				},
				IbcEvent::UpdateClient(update) => {
					observe_delta_time(
						&self.last_update_client_time,
						&self.metrics.sent_update_client_time,
					);
					self.metrics.update_light_client_height(
						&update.common.client_id,
						update.common.consensus_height,
						&self.registry,
					)?;
				},
				_ => (),
			}
		}
		Ok(())
	}

	pub async fn handle_messages(&self, messages: &[Any]) {
		for message in messages {
			match message.type_url.as_str() {
				"/ibc.core.channel.v1.MsgAcknowledgement" => {
					self.metrics.number_of_sent_acknowledgments.inc();
					self.metrics.number_of_undelivered_acknowledgements.set(
						self.metrics.number_of_sent_acknowledgments.get() -
							self.metrics.counterparty_number_of_received_acknowledgments().get(),
					);
				},
				"/ibc.core.channel.v1.MsgRecvPacket" => {
					self.metrics.number_of_undelivered_packets.set(
						self.metrics.number_of_sent_packets.get() -
							self.metrics.counterparty_number_of_received_packets().get(),
					);
					self.metrics.number_of_sent_packets.inc();
				},
				_ => (),
			}
		}
	}

	pub fn link_with_counterparty(&mut self, counterparty: &mut Self) {
		self.metrics.link_with_counterparty_metrics(&mut counterparty.metrics);

		self.counterparty_last_sent_packet_time = Some(counterparty.last_sent_packet_time.clone());
		self.counterparty_last_sent_acknowledgment_time =
			Some(counterparty.last_sent_acknowledgment_time.clone());
		self.counterparty_last_sent_timeout_packet_time =
			Some(counterparty.last_sent_timeout_packet_time.clone());

		counterparty.counterparty_last_sent_packet_time = Some(self.last_sent_packet_time.clone());
		counterparty.counterparty_last_sent_acknowledgment_time =
			Some(self.last_sent_acknowledgment_time.clone());
		counterparty.counterparty_last_sent_timeout_packet_time =
			Some(self.last_sent_timeout_packet_time.clone());
	}

	pub async fn handle_timeouts(&self, timeouts: &[Any]) {
		for message in timeouts {
			match message.type_url.as_str() {
				"/ibc.core.channel.v1.MsgTimeout" | "/ibc.core.channel.v1.MsgTimeoutOnClose" => {
					self.metrics.number_of_sent_timeout_packets.inc();
				},
				_ => (),
			}
		}
	}

	pub async fn handle_transaction_costs(&self, batch_weight: u64, messages: &[Any]) {
		let batch_size = messages.iter().map(|x| x.value.len()).sum::<usize>();
		self.metrics.gas_cost_for_sent_tx_bundle.observe(batch_weight as f64);
		self.metrics.transaction_length_for_sent_tx_bundle.observe(batch_size as f64);
	}

	pub fn observe_last_packet_time(
		&self,
		packet: &Packet,
		counterparty_map: &Option<PacketMap>,
		time_metrics: &Histogram,
	) {
		let now = Instant::now();
		let guard = counterparty_map.as_ref()
            .expect("counterparty_*_time is not set. Perhaps you forgot to call `link_with_counterparty`?")
            .lock()
            .unwrap();
		if let Some(last_time) = guard.get(&packet.clone().into()) {
			let elapsed = now.duration_since(*last_time);
			time_metrics.observe(elapsed.as_millis() as f64);
		} else {
			log::warn!("No last time found for packet {:?}", packet);
		}
	}
}

fn observe_delta_time(cell: &Cell<Option<Instant>>, time_metrics: &Histogram) {
	let now = Instant::now();
	let time = cell.get();
	if let Some(last_time) = time {
		let elapsed = now - last_time;
		cell.set(Some(now));
		time_metrics.observe(elapsed.as_millis() as f64);
	} else {
		cell.set(Some(now));
	}
}
