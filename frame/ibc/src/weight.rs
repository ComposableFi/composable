use super::*;
use core::marker::PhantomData;
use frame_support::pallet_prelude::Weight;
use ibc::core::{
	ics02_client::msgs::ClientMsg,
	ics03_connection::msgs::ConnectionMsg,
	ics04_channel::msgs::{ChannelMsg, PacketMsg},
	ics26_routing::msgs::Ics26Envelope,
};
use ibc_trait::CallbackWeight;

pub trait WeightInfo {
	fn create_client() -> Weight;
	fn update_client() -> Weight;
	fn connection_init() -> Weight;
	fn conn_try_open() -> Weight;
	fn conn_open_ack() -> Weight;
	fn conn_open_confirm() -> Weight;
	fn create_channel() -> Weight;
	fn channel_open_try() -> Weight;
	fn channel_open_ack() -> Weight;
	fn channel_open_confirm() -> Weight;
	fn channel_close_init() -> Weight;
	fn channel_close_confirm() -> Weight;
	fn recv_packet(i: u32) -> Weight;
	fn ack_packet(i: u32, j: u32) -> Weight;
	fn timeout_packet(i: u32) -> Weight;
}

impl WeightInfo for () {
	fn create_client() -> Weight {
		0
	}

	fn update_client() -> Weight {
		0
	}

	fn connection_init() -> Weight {
		0
	}

	fn conn_try_open() -> Weight {
		0
	}

	fn conn_open_ack() -> Weight {
		0
	}

	fn conn_open_confirm() -> Weight {
		0
	}

	fn create_channel() -> Weight {
		0
	}

	fn channel_open_try() -> Weight {
		0
	}

	fn channel_open_ack() -> Weight {
		0
	}

	fn channel_open_confirm() -> Weight {
		0
	}

	fn channel_close_init() -> Weight {
		0
	}

	fn channel_close_confirm() -> Weight {
		0
	}

	fn recv_packet(_i: u32) -> Weight {
		0
	}

	fn ack_packet(_i: u32, _j: u32) -> Weight {
		0
	}

	fn timeout_packet(_i: u32) -> Weight {
		0
	}
}

pub struct WeightRouter<T: Config>(PhantomData<T>);

impl<T: Config> WeightRouter<T> {
	pub fn get_weight(port_id: &str) -> Option<Box<dyn CallbackWeight>> {
		match port_id {
			pallet_ibc_ping::PORT_ID => Some(Box::new(pallet_ibc_ping::WeightHandler::<T>::new())),
			_ => None,
		}
	}
}

pub(crate) fn deliver<T: Config>(msgs: &Vec<Any>) -> Weight {
	msgs.into_iter()
		.filter_map(|msg| {
			let type_url = String::from_utf8(msg.type_url.clone()).unwrap_or_default();
			let msg = ibc_proto::google::protobuf::Any { type_url, value: msg.value.clone() };
			let msg: Option<Ics26Envelope> = msg.try_into().ok();
			msg
		})
		.fold(Weight::default(), |acc, msg| {
			// Decode message type and get port_id
			// Add benchmarked weight for that message type
			// Add benchmarked weight for module callback
			let temp = match msg {
				Ics26Envelope::Ics2Msg(msgs) => match msgs {
					ClientMsg::CreateClient(_) => <T as Config>::WeightInfo::create_client(),
					ClientMsg::UpdateClient(_) => <T as Config>::WeightInfo::update_client(),
					ClientMsg::Misbehaviour(_) => Weight::default(),
					ClientMsg::UpgradeClient(_) => Weight::default(),
				},
				Ics26Envelope::Ics3Msg(msgs) => match msgs {
					ConnectionMsg::ConnectionOpenInit(_) =>
						<T as Config>::WeightInfo::connection_init(),
					ConnectionMsg::ConnectionOpenTry(_) =>
						<T as Config>::WeightInfo::conn_try_open(),
					ConnectionMsg::ConnectionOpenAck(_) =>
						<T as Config>::WeightInfo::conn_open_ack(),
					ConnectionMsg::ConnectionOpenConfirm(_) =>
						<T as Config>::WeightInfo::conn_open_confirm(),
				},
				Ics26Envelope::Ics4ChannelMsg(msgs) => match msgs {
					ChannelMsg::ChannelOpenInit(channel_msg) => {
						let cb = WeightRouter::<T>::get_weight(channel_msg.port_id.as_str())
							.unwrap_or(Box::new(()));
						let cb_weight = cb.on_chan_open_init();
						cb_weight.saturating_add(<T as Config>::WeightInfo::create_channel())
					},
					ChannelMsg::ChannelOpenTry(channel_msg) => {
						let cb = WeightRouter::<T>::get_weight(channel_msg.port_id.as_str())
							.unwrap_or(Box::new(()));
						let cb_weight = cb.on_chan_open_try();
						cb_weight.saturating_add(<T as Config>::WeightInfo::channel_open_try())
					},
					ChannelMsg::ChannelOpenAck(channel_msg) => {
						let cb = WeightRouter::<T>::get_weight(channel_msg.port_id.as_str())
							.unwrap_or(Box::new(()));
						let cb_weight =
							cb.on_chan_open_ack(&channel_msg.port_id, &channel_msg.channel_id);
						cb_weight.saturating_add(<T as Config>::WeightInfo::channel_open_ack())
					},
					ChannelMsg::ChannelOpenConfirm(channel_msg) => {
						let cb = WeightRouter::<T>::get_weight(channel_msg.port_id.as_str())
							.unwrap_or(Box::new(()));
						let cb_weight =
							cb.on_chan_open_confirm(&channel_msg.port_id, &channel_msg.channel_id);
						cb_weight.saturating_add(<T as Config>::WeightInfo::channel_open_confirm())
					},
					ChannelMsg::ChannelCloseInit(channel_msg) => {
						let cb = WeightRouter::<T>::get_weight(channel_msg.port_id.as_str())
							.unwrap_or(Box::new(()));
						let cb_weight =
							cb.on_chan_close_init(&channel_msg.port_id, &channel_msg.channel_id);
						cb_weight.saturating_add(<T as Config>::WeightInfo::channel_close_init())
					},
					ChannelMsg::ChannelCloseConfirm(channel_msg) => {
						let cb = WeightRouter::<T>::get_weight(channel_msg.port_id.as_str())
							.unwrap_or(Box::new(()));
						let cb_weight =
							cb.on_chan_close_confirm(&channel_msg.port_id, &channel_msg.channel_id);
						cb_weight.saturating_add(<T as Config>::WeightInfo::channel_close_confirm())
					},
				},
				Ics26Envelope::Ics4PacketMsg(msgs) => match msgs {
					PacketMsg::RecvPacket(packet_msg) => {
						let cb = WeightRouter::<T>::get_weight(
							packet_msg.packet.destination_port.as_str(),
						)
						.unwrap_or(Box::new(()));
						let cb_weight = cb.on_recv_packet(&packet_msg.packet);
						cb_weight.saturating_add(<T as Config>::WeightInfo::recv_packet(
							packet_msg.packet.data.len() as u32,
						))
					},
					PacketMsg::AckPacket(packet_msg) => {
						let cb = WeightRouter::<T>::get_weight(
							packet_msg.packet.destination_port.as_str(),
						)
						.unwrap_or(Box::new(()));
						let cb_weight = cb.on_acknowledgement_packet(
							&packet_msg.packet,
							&packet_msg.acknowledgement,
						);
						cb_weight.saturating_add(<T as Config>::WeightInfo::ack_packet(
							packet_msg.packet.data.len() as u32,
							packet_msg.acknowledgement.into_bytes().len() as u32,
						))
					},
					PacketMsg::ToPacket(packet_msg) => {
						let cb = WeightRouter::<T>::get_weight(
							packet_msg.packet.destination_port.as_str(),
						)
						.unwrap_or(Box::new(()));
						let cb_weight = cb.on_timeout_packet(&packet_msg.packet);
						cb_weight.saturating_add(<T as Config>::WeightInfo::timeout_packet(
							packet_msg.packet.data.len() as u32,
						))
					},
					PacketMsg::ToClosePacket(packet_msg) => {
						let cb = WeightRouter::<T>::get_weight(
							packet_msg.packet.destination_port.as_str(),
						)
						.unwrap_or(Box::new(()));
						let cb_weight = cb.on_timeout_packet(&packet_msg.packet);
						cb_weight.saturating_add(<T as Config>::WeightInfo::timeout_packet(
							packet_msg.packet.data.len() as u32,
						))
					},
				},
				_ => Weight::default(),
			};
			acc.saturating_add(temp)
		})
}
