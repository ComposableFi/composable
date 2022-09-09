use super::*;
use core::marker::PhantomData;
use frame_support::pallet_prelude::Weight;
use ibc::core::{
	ics02_client::{client_type::ClientType, msgs::ClientMsg},
	ics03_connection::{context::ConnectionReader, msgs::ConnectionMsg},
	ics04_channel::msgs::{ChannelMsg, PacketMsg},
	ics24_host::identifier::ClientId,
	ics26_routing::msgs::Ics26Envelope,
};
use ibc_trait::{client_id_from_bytes, CallbackWeight};
use scale_info::prelude::string::ToString;

pub trait WeightInfo {
	fn create_client() -> Weight;
	fn initiate_connection() -> Weight;
	fn update_tendermint_client() -> Weight;
	fn connection_open_init() -> Weight;
	fn conn_try_open_tendermint() -> Weight;
	fn conn_open_ack_tendermint() -> Weight;
	fn conn_open_confirm_tendermint() -> Weight;
	fn channel_open_init() -> Weight;
	fn channel_open_try_tendermint() -> Weight;
	fn channel_open_ack_tendermint() -> Weight;
	fn channel_open_confirm_tendermint() -> Weight;
	fn channel_close_init() -> Weight;
	fn channel_close_confirm_tendermint() -> Weight;
	fn recv_packet_tendermint(i: u32) -> Weight;
	fn ack_packet_tendermint(i: u32, j: u32) -> Weight;
	fn timeout_packet_tendermint(i: u32) -> Weight;
	/// on_finalize benchmarks
	/// a => number of light clients
	/// b => number of connections
	/// c => number of channels
	/// d => number of packet commitments
	/// e => number of packet acknowledgements
	/// f => number of packet receipts
	fn on_finalize(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) -> Weight;
}

impl WeightInfo for () {
	fn create_client() -> Weight {
		0
	}

	fn initiate_connection() -> Weight {
		0
	}

	fn update_tendermint_client() -> Weight {
		0
	}

	fn connection_open_init() -> Weight {
		0
	}

	fn conn_try_open_tendermint() -> Weight {
		0
	}

	fn conn_open_ack_tendermint() -> Weight {
		0
	}

	fn conn_open_confirm_tendermint() -> Weight {
		0
	}

	fn channel_open_init() -> Weight {
		0
	}

	fn channel_open_try_tendermint() -> Weight {
		0
	}

	fn channel_open_ack_tendermint() -> Weight {
		0
	}

	fn channel_open_confirm_tendermint() -> Weight {
		0
	}

	fn channel_close_init() -> Weight {
		0
	}

	fn channel_close_confirm_tendermint() -> Weight {
		0
	}

	fn recv_packet_tendermint(_i: u32) -> Weight {
		0
	}

	fn ack_packet_tendermint(_i: u32, _j: u32) -> Weight {
		0
	}

	fn timeout_packet_tendermint(_i: u32) -> Weight {
		0
	}

	fn on_finalize(_a: u32, _b: u32, _c: u32, _d: u32, _e: u32, _f: u32) -> Weight {
		0
	}
}

pub struct WeightRouter<T: Config>(PhantomData<T>);

impl<T: Config> WeightRouter<T> {
	pub fn get_weight(port_id: &str) -> Option<Box<dyn CallbackWeight>> {
		match port_id {
			pallet_ibc_ping::PORT_ID =>
				Some(Box::new(pallet_ibc_ping::WeightHandler::<T>::default())),
			ibc::applications::transfer::PORT_ID_STR =>
				Some(Box::new(transfer::WeightHandler::<T>::default())),
			_ => None,
		}
	}
}

/// Get client id for a port and channel combination
pub fn channel_client<T: Config>(channel_id: &[u8], port_id: &[u8]) -> Result<ClientId, Error<T>> {
	for (connection_id, channels) in ChannelsConnection::<T>::iter() {
		if channels.contains(&(port_id.to_vec(), channel_id.to_vec())) {
			if let Some((client_id, ..)) = ConnectionClient::<T>::iter()
				.find(|(.., connection_ids)| connection_ids.contains(&connection_id))
			{
				return client_id_from_bytes(client_id).map_err(|_| Error::<T>::Other)
			}
		}
	}
	Err(Error::<T>::Other)
}

pub(crate) fn deliver<T: Config + Send + Sync>(msgs: &[Any]) -> Weight
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	msgs.iter()
		.filter_map(|msg| {
			let type_url = String::from_utf8(msg.type_url.clone()).unwrap_or_default();
			let msg = ibc_proto::google::protobuf::Any { type_url, value: msg.value.clone() };
			let msg: Option<Ics26Envelope> = msg.try_into().ok();
			msg
		})
		.fold(Weight::default(), |acc, msg| {
			// Add benchmarked weight for that message type
			// Add benchmarked weight for module callback
			let temp = match msg {
				Ics26Envelope::Ics2Msg(msgs) => match msgs {
					ClientMsg::CreateClient(_) => Weight::default(),
					ClientMsg::UpdateClient(msg) => {
						let client_type = msg.client_id.as_str().rsplit_once('-').and_then(
							|(client_type_str, ..)| ClientType::from_str(client_type_str).ok(),
						);
						match client_type {
							Some(ClientType::Tendermint) =>
								<T as Config>::WeightInfo::update_tendermint_client(),
							_ => Weight::default(),
						}
					},
					ClientMsg::Misbehaviour(_) => Weight::default(),
					ClientMsg::UpgradeClient(_) => Weight::default(),
				},
				Ics26Envelope::Ics3Msg(msgs) => match msgs {
					ConnectionMsg::ConnectionOpenInit(msg) => {
						let client_type = msg.client_id.as_str().rsplit_once('-').and_then(
							|(client_type_str, ..)| ClientType::from_str(client_type_str).ok(),
						);
						match client_type {
							Some(ClientType::Tendermint) =>
								<T as Config>::WeightInfo::connection_open_init(),
							_ => Weight::default(),
						}
					},
					ConnectionMsg::ConnectionOpenTry(msg) => {
						let client_type = msg.client_id.as_str().rsplit_once('-').and_then(
							|(client_type_str, ..)| ClientType::from_str(client_type_str).ok(),
						);
						match client_type {
							Some(ClientType::Tendermint) =>
								<T as Config>::WeightInfo::conn_try_open_tendermint(),
							_ => Weight::default(),
						}
					},
					ConnectionMsg::ConnectionOpenAck(msg) => {
						let connection_id = msg.connection_id;
						let ctx = routing::Context::<T>::new();
						let connection_end = ctx.connection_end(&connection_id).unwrap_or_default();
						let client_type =
							connection_end.client_id().as_str().rsplit_once('-').and_then(
								|(client_type_str, ..)| ClientType::from_str(client_type_str).ok(),
							);
						match client_type {
							Some(ClientType::Tendermint) =>
								<T as Config>::WeightInfo::conn_open_ack_tendermint(),
							_ => Weight::default(),
						}
					},
					ConnectionMsg::ConnectionOpenConfirm(msg) => {
						let connection_id = msg.connection_id;
						let ctx = routing::Context::<T>::new();
						let connection_end = ctx.connection_end(&connection_id).unwrap_or_default();
						let client_type =
							connection_end.client_id().as_str().rsplit_once('-').and_then(
								|(client_type_str, ..)| ClientType::from_str(client_type_str).ok(),
							);
						match client_type {
							Some(ClientType::Tendermint) =>
								<T as Config>::WeightInfo::conn_open_confirm_tendermint(),
							_ => Weight::default(),
						}
					},
				},
				Ics26Envelope::Ics4ChannelMsg(msgs) => match msgs {
					ChannelMsg::ChannelOpenInit(channel_msg) => {
						let cb = WeightRouter::<T>::get_weight(channel_msg.port_id.as_str())
							.unwrap_or_else(|| Box::new(()));
						let cb_weight = cb.on_chan_open_init();
						let lc_verification_weight =
							match channel_msg.channel.connection_hops.get(0) {
								Some(connection_id) => {
									let ctx = routing::Context::<T>::new();
									let connection_end =
										ctx.connection_end(connection_id).unwrap_or_default();
									let client_type = connection_end
										.client_id()
										.as_str()
										.rsplit_once('-')
										.and_then(|(client_type_str, ..)| {
											ClientType::from_str(client_type_str).ok()
										});
									match client_type {
										Some(ClientType::Tendermint) =>
											<T as Config>::WeightInfo::channel_open_init(),
										_ => Weight::default(),
									}
								},
								None => Weight::default(),
							};
						cb_weight.saturating_add(lc_verification_weight)
					},
					ChannelMsg::ChannelOpenTry(channel_msg) => {
						let cb = WeightRouter::<T>::get_weight(channel_msg.port_id.as_str())
							.unwrap_or_else(|| Box::new(()));
						let cb_weight = cb.on_chan_open_try();
						let lc_verification_weight =
							match channel_msg.channel.connection_hops.get(0) {
								Some(connection_id) => {
									let ctx = routing::Context::<T>::new();
									let connection_end =
										ctx.connection_end(connection_id).unwrap_or_default();
									let client_type = connection_end
										.client_id()
										.as_str()
										.rsplit_once('-')
										.and_then(|(client_type_str, ..)| {
											ClientType::from_str(client_type_str).ok()
										});
									match client_type {
										Some(ClientType::Tendermint) =>
											<T as Config>::WeightInfo::channel_open_try_tendermint(),
										_ => Weight::default(),
									}
								},
								None => Weight::default(),
							};
						cb_weight.saturating_add(lc_verification_weight)
					},
					ChannelMsg::ChannelOpenAck(channel_msg) => {
						let cb = WeightRouter::<T>::get_weight(channel_msg.port_id.as_str())
							.unwrap_or_else(|| Box::new(()));
						let cb_weight =
							cb.on_chan_open_ack(&channel_msg.port_id, &channel_msg.channel_id);
						let lc_verification_weight = match channel_client::<T>(
							channel_msg.port_id.as_bytes(),
							channel_msg.channel_id.to_string().as_bytes(),
						) {
							Ok(client_id) => {
								let client_type = client_id.as_str().rsplit_once('-').and_then(
									|(client_type_str, ..)| {
										ClientType::from_str(client_type_str).ok()
									},
								);
								match client_type {
									Some(ClientType::Tendermint) =>
										<T as Config>::WeightInfo::channel_open_ack_tendermint(),
									_ => Weight::default(),
								}
							},
							Err(_) => Weight::default(),
						};
						cb_weight.saturating_add(lc_verification_weight)
					},
					ChannelMsg::ChannelOpenConfirm(channel_msg) => {
						let cb = WeightRouter::<T>::get_weight(channel_msg.port_id.as_str())
							.unwrap_or_else(|| Box::new(()));
						let cb_weight =
							cb.on_chan_open_confirm(&channel_msg.port_id, &channel_msg.channel_id);
						let lc_verification_weight = match channel_client::<T>(
							channel_msg.port_id.as_bytes(),
							channel_msg.channel_id.to_string().as_bytes(),
						) {
							Ok(client_id) => {
								let client_type = client_id.as_str().rsplit_once('-').and_then(
									|(client_type_str, ..)| {
										ClientType::from_str(client_type_str).ok()
									},
								);
								match client_type {
									Some(ClientType::Tendermint) =>
										<T as Config>::WeightInfo::channel_open_confirm_tendermint(),
									_ => Weight::default(),
								}
							},
							Err(_) => Weight::default(),
						};
						cb_weight.saturating_add(lc_verification_weight)
					},
					ChannelMsg::ChannelCloseInit(channel_msg) => {
						let cb = WeightRouter::<T>::get_weight(channel_msg.port_id.as_str())
							.unwrap_or_else(|| Box::new(()));
						let cb_weight =
							cb.on_chan_close_init(&channel_msg.port_id, &channel_msg.channel_id);
						let lc_verification_weight = match channel_client::<T>(
							channel_msg.port_id.as_bytes(),
							channel_msg.channel_id.to_string().as_bytes(),
						) {
							Ok(client_id) => {
								let client_type = client_id.as_str().rsplit_once('-').and_then(
									|(client_type_str, ..)| {
										ClientType::from_str(client_type_str).ok()
									},
								);
								match client_type {
									Some(ClientType::Tendermint) =>
										<T as Config>::WeightInfo::channel_close_init(),
									_ => Weight::default(),
								}
							},
							Err(_) => Weight::default(),
						};
						cb_weight.saturating_add(lc_verification_weight)
					},
					ChannelMsg::ChannelCloseConfirm(channel_msg) => {
						let cb = WeightRouter::<T>::get_weight(channel_msg.port_id.as_str())
							.unwrap_or_else(|| Box::new(()));
						let cb_weight =
							cb.on_chan_close_confirm(&channel_msg.port_id, &channel_msg.channel_id);
						let lc_verification_weight =
							match channel_client::<T>(
								channel_msg.port_id.as_bytes(),
								channel_msg.channel_id.to_string().as_bytes(),
							) {
								Ok(client_id) => {
									let client_type = client_id.as_str().rsplit_once('-').and_then(
										|(client_type_str, ..)| {
											ClientType::from_str(client_type_str).ok()
										},
									);
									match client_type {
									Some(ClientType::Tendermint) => <T as Config>::WeightInfo::channel_close_confirm_tendermint(),
									_ => Weight::default()
								}
								},
								Err(_) => Weight::default(),
							};
						cb_weight.saturating_add(lc_verification_weight)
					},
				},
				Ics26Envelope::Ics4PacketMsg(msgs) => match msgs {
					PacketMsg::RecvPacket(packet_msg) => {
						let cb = WeightRouter::<T>::get_weight(
							packet_msg.packet.destination_port.as_str(),
						)
						.unwrap_or_else(|| Box::new(()));
						let cb_weight = cb.on_recv_packet(&packet_msg.packet);
						let lc_verification_weight = match channel_client::<T>(
							packet_msg.packet.destination_port.as_bytes(),
							packet_msg.packet.destination_channel.to_string().as_bytes(),
						) {
							Ok(client_id) => {
								let client_type = client_id.as_str().rsplit_once('-').and_then(
									|(client_type_str, ..)| {
										ClientType::from_str(client_type_str).ok()
									},
								);
								match client_type {
									Some(ClientType::Tendermint) =>
										<T as Config>::WeightInfo::recv_packet_tendermint(
											packet_msg.packet.data.len() as u32,
										),
									_ => Weight::default(),
								}
							},
							Err(_) => Weight::default(),
						};
						cb_weight.saturating_add(lc_verification_weight)
					},
					PacketMsg::AckPacket(packet_msg) => {
						let cb = WeightRouter::<T>::get_weight(
							packet_msg.packet.destination_port.as_str(),
						)
						.unwrap_or_else(|| Box::new(()));
						let cb_weight = cb.on_acknowledgement_packet(
							&packet_msg.packet,
							&packet_msg.acknowledgement,
						);
						let lc_verification_weight = match channel_client::<T>(
							packet_msg.packet.destination_port.as_bytes(),
							packet_msg.packet.destination_channel.to_string().as_bytes(),
						) {
							Ok(client_id) => {
								let client_type = client_id.as_str().rsplit_once('-').and_then(
									|(client_type_str, ..)| {
										ClientType::from_str(client_type_str).ok()
									},
								);
								match client_type {
									Some(ClientType::Tendermint) =>
										<T as Config>::WeightInfo::ack_packet_tendermint(
											packet_msg.packet.data.len() as u32,
											packet_msg.acknowledgement.into_bytes().len() as u32,
										),
									_ => Weight::default(),
								}
							},
							Err(_) => Weight::default(),
						};
						cb_weight.saturating_add(lc_verification_weight)
					},
					PacketMsg::ToPacket(packet_msg) => {
						let cb = WeightRouter::<T>::get_weight(
							packet_msg.packet.destination_port.as_str(),
						)
						.unwrap_or_else(|| Box::new(()));
						let cb_weight = cb.on_timeout_packet(&packet_msg.packet);
						let lc_verification_weight = match channel_client::<T>(
							packet_msg.packet.destination_port.as_bytes(),
							packet_msg.packet.destination_channel.to_string().as_bytes(),
						) {
							Ok(client_id) => {
								let client_type = client_id.as_str().rsplit_once('-').and_then(
									|(client_type_str, ..)| {
										ClientType::from_str(client_type_str).ok()
									},
								);
								match client_type {
									Some(ClientType::Tendermint) =>
										<T as Config>::WeightInfo::timeout_packet_tendermint(
											packet_msg.packet.data.len() as u32,
										),
									_ => Weight::default(),
								}
							},
							Err(_) => Weight::default(),
						};
						cb_weight.saturating_add(lc_verification_weight)
					},
					PacketMsg::ToClosePacket(packet_msg) => {
						let cb = WeightRouter::<T>::get_weight(
							packet_msg.packet.destination_port.as_str(),
						)
						.unwrap_or_else(|| Box::new(()));
						let cb_weight = cb.on_timeout_packet(&packet_msg.packet);
						let lc_verification_weight = match channel_client::<T>(
							packet_msg.packet.destination_port.as_bytes(),
							packet_msg.packet.destination_channel.to_string().as_bytes(),
						) {
							Ok(client_id) => {
								let client_type = client_id.as_str().rsplit_once('-').and_then(
									|(client_type_str, ..)| {
										ClientType::from_str(client_type_str).ok()
									},
								);
								match client_type {
									Some(ClientType::Tendermint) =>
										<T as Config>::WeightInfo::timeout_packet_tendermint(
											packet_msg.packet.data.len() as u32,
										),
									_ => Weight::default(),
								}
							},
							Err(_) => Weight::default(),
						};
						cb_weight.saturating_add(lc_verification_weight)
					},
				},
				_ => Weight::default(),
			};
			acc.saturating_add(temp)
		})
}
