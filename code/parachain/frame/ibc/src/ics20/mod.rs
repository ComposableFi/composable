pub mod context;

use crate::{routing::Context, ChannelIds, Config, Event, Pallet, WeightInfo};
use alloc::{
	format,
	string::{String, ToString},
};
use composable_traits::{
	defi::DeFiComposableConfig,
	xcm::assets::{RemoteAssetRegistryInspect, RemoteAssetRegistryMutate, XcmAssetLocation},
};
use core::{fmt::Formatter, str::FromStr};
use frame_support::weights::Weight;
use ibc::{
	applications::transfer::{
		acknowledgement::{Acknowledgement as Ics20Acknowledgement, ACK_ERR_STR, ACK_SUCCESS_B64},
		error::Error as Ics20Error,
		is_receiver_chain_source, is_sender_chain_source,
		packet::PacketData,
		relay::{
			on_ack_packet::process_ack_packet, on_recv_packet::process_recv_packet,
			on_timeout_packet::process_timeout_packet,
		},
		PrefixedCoin, PrefixedDenom, TracePrefix, VERSION,
	},
	core::{
		ics04_channel::{
			channel::{Counterparty, Order},
			error::Error as Ics04Error,
			msgs::acknowledgement::Acknowledgement,
			packet::Packet,
			Version,
		},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
		ics26_routing::context::{Module, ModuleOutputBuilder, OnRecvPacketAck},
	},
	signer::Signer,
};
use ibc_primitives::{CallbackWeight, IbcHandler};
use primitives::currency::CurrencyId;
use sp_std::{boxed::Box, marker::PhantomData};

#[derive(Clone, Eq, PartialEq)]
pub struct IbcModule<T: Config>(PhantomData<T>);

impl<T: Config> core::fmt::Debug for IbcModule<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "ibc-transfer")
	}
}

impl<T: Config> Default for IbcModule<T> {
	fn default() -> Self {
		Self(PhantomData::default())
	}
}

impl<T: Config + Send + Sync> Module for IbcModule<T>
where
	<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
	u32: From<<T as frame_system::Config>::BlockNumber>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetId>,
	<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetNativeLocation: From<XcmAssetLocation>,
	<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetNativeLocation: From<XcmAssetLocation>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<<T as assets::Config>::AssetId>,
{
	fn on_chan_open_init(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		_order: Order,
		_connection_hops: &[ConnectionId],
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_counterparty: &Counterparty,
		_version: &Version,
	) -> Result<(), Ics04Error> {
		Ok(())
	}

	fn on_chan_open_try(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		order: Order,
		_connection_hops: &[ConnectionId],
		port_id: &PortId,
		_channel_id: &ChannelId,
		counterparty: &Counterparty,
		version: &Version,
		counterparty_version: &Version,
	) -> Result<Version, Ics04Error> {
		if counterparty_version.to_string() != *VERSION || version.to_string() != *VERSION {
			return Err(Ics04Error::no_common_version())
		}

		if order != Order::Unordered {
			return Err(Ics04Error::unknown_order_type(order.to_string()))
		}

		if counterparty.port_id() != &PortId::transfer() || port_id != &PortId::transfer() {
			return Err(Ics04Error::implementation_specific(format!(
				"Invalid counterparty port {:?}",
				counterparty.port_id()
			)))
		}

		Ok(version.clone())
	}

	fn on_chan_open_ack(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		_port_id: &PortId,
		channel_id: &ChannelId,
		_counterparty_version: &Version,
	) -> Result<(), Ics04Error> {
		let _ = ChannelIds::<T>::try_mutate::<_, (), _>(|channels| {
			channels.push(channel_id.to_string().as_bytes().to_vec());
			Ok(())
		});
		Ok(())
	}

	fn on_chan_open_confirm(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		_port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<(), Ics04Error> {
		let _ = ChannelIds::<T>::try_mutate::<_, (), _>(|channels| {
			channels.push(channel_id.to_string().as_bytes().to_vec());
			Ok(())
		});
		Ok(())
	}

	fn on_chan_close_init(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<(), Ics04Error> {
		let _ = ChannelIds::<T>::try_mutate::<_, (), _>(|channels| {
			let rem = channels
				.iter()
				.filter(|chan| chan.as_slice() != channel_id.to_string().as_bytes())
				.cloned()
				.collect();
			*channels = rem;
			Ok(())
		});
		// Remove escrow address for closed channel if it exists
		Pallet::<T>::remove_channel_escrow_address(port_id, *channel_id)
	}

	fn on_chan_close_confirm(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<(), Ics04Error> {
		let _ = ChannelIds::<T>::try_mutate::<_, (), _>(|channels| {
			let rem = channels
				.iter()
				.filter(|chan| chan.as_slice() != channel_id.to_string().as_bytes())
				.cloned()
				.collect();
			*channels = rem;
			Ok(())
		});
		// Remove escrow address for closed channel if it exists
		Pallet::<T>::remove_channel_escrow_address(port_id, *channel_id)
	}

	fn on_recv_packet(
		&self,
		output: &mut ModuleOutputBuilder,
		packet: &Packet,
		_relayer: &Signer,
	) -> OnRecvPacketAck {
		let mut ctx = Context::<T>::default();
		let result = serde_json::from_slice(packet.data.as_slice())
			.map_err(|e| {
				Ics04Error::implementation_specific(format!("Failed to decode packet data {:?}", e))
			})
			.and_then(|packet_data: PacketData| {
				process_recv_packet(&ctx, output, packet, packet_data.clone())
					.and_then(|write_fn| {
						write_fn(&mut ctx)
							.map(|_| packet_data)
							.map_err(Ics20Error::unknown_msg_type)
					})
					.map_err(|e| {
						log::trace!(target: "pallet_ibc", "[on_recv_packet]: {:?}", e);
						Ics04Error::implementation_specific(e.to_string())
					})
			});
		match result {
			Err(err) => {
				let packet = packet.clone();
				OnRecvPacketAck::Nil(Box::new(move |_ctx| {
					Pallet::<T>::write_acknowledgement(
						&packet,
						format!("{}: {:?}", ACK_ERR_STR, err).as_bytes().to_vec(),
					)
					.map_err(|e| format!("[on_recv_packet] {:#?}", e))
				}))
			},
			Ok(packet_data) => {
				let denom = full_ibc_denom(packet, packet_data.token.clone());
				let prefixed_denom = PrefixedDenom::from_str(&denom).expect("Should not fail");
				let token = PrefixedCoin {
					denom: prefixed_denom.clone(),
					amount: packet_data.token.amount,
				};
				Pallet::<T>::deposit_event(Event::<T>::TokenReceived {
					from: packet_data.sender.to_string().as_bytes().to_vec(),
					to: packet_data.receiver.to_string().as_bytes().to_vec(),
					ibc_denom: denom.as_bytes().to_vec(),
					local_asset_id: Pallet::<T>::ibc_denom_to_asset_id(denom, token),
					amount: packet_data.token.amount.as_u256().as_u128().into(),
					is_receiver_source: is_receiver_chain_source(
						packet.source_port.clone(),
						packet.source_channel.clone(),
						&prefixed_denom,
					),
				});
				let packet = packet.clone();
				OnRecvPacketAck::Successful(
					Box::new(Ics20Acknowledgement::success()),
					Box::new(move |_ctx| {
						Pallet::<T>::write_acknowledgement(
							&packet,
							Ics20Acknowledgement::success().as_ref().to_vec(),
						)
						.map_err(|e| format!("[on_recv_packet] {:#?}", e))
					}),
				)
			},
		}
	}

	fn on_acknowledgement_packet(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		packet: &Packet,
		acknowledgement: &Acknowledgement,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let mut ctx = Context::<T>::default();
		let packet_data: PacketData =
			serde_json::from_slice(packet.data.as_slice()).map_err(|e| {
				Ics04Error::implementation_specific(format!("Failed to decode packet data {:?}", e))
			})?;
		let ack = String::from_utf8(acknowledgement.as_ref().to_vec())
			.map(|val| {
				if val.as_bytes() == ACK_SUCCESS_B64 {
					Ics20Acknowledgement::Success(ACK_SUCCESS_B64.to_vec())
				} else {
					Ics20Acknowledgement::Error(val)
				}
			})
			.map_err(|e| {
				Ics04Error::implementation_specific(format!(
					"Failed to decode acknowledgement data {:?}",
					e
				))
			})?;
		process_ack_packet(&mut ctx, packet, &packet_data, &ack)
			.map_err(|e| Ics04Error::implementation_specific(e.to_string()))?;

		let denom = full_ibc_denom(packet, packet_data.token.clone());
		let prefixed_denom = PrefixedDenom::from_str(&denom).expect("Should not fail");

		match ack {
			Ics20Acknowledgement::Success(_) =>
				Pallet::<T>::deposit_event(Event::<T>::TokenTransferCompleted {
					from: packet_data.sender.to_string().as_bytes().to_vec(),
					to: packet_data.receiver.to_string().as_bytes().to_vec(),
					ibc_denom: packet_data.token.denom.to_string().as_bytes().to_vec(),
					local_asset_id: Pallet::<T>::ibc_denom_to_asset_id(
						packet_data.token.denom.to_string(),
						packet_data.token.clone(),
					),
					amount: packet_data.token.amount.as_u256().as_u128().into(),
					is_sender_source: is_sender_chain_source(
						packet.source_port.clone(),
						packet.source_channel.clone(),
						&prefixed_denom,
					),
				}),
			Ics20Acknowledgement::Error(_) =>
				Pallet::<T>::deposit_event(Event::<T>::TokenTransferFailed {
					from: packet_data.sender.to_string().as_bytes().to_vec(),
					to: packet_data.receiver.to_string().as_bytes().to_vec(),
					ibc_denom: packet_data.token.denom.to_string().as_bytes().to_vec(),
					local_asset_id: Pallet::<T>::ibc_denom_to_asset_id(
						packet_data.token.denom.to_string(),
						packet_data.token.clone(),
					),
					amount: packet_data.token.amount.as_u256().as_u128().into(),
					is_sender_source: is_sender_chain_source(
						packet.source_port.clone(),
						packet.source_channel.clone(),
						&prefixed_denom,
					),
				}),
		}

		Ok(())
	}

	fn on_timeout_packet(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		packet: &Packet,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let mut ctx = Context::<T>::default();
		let packet_data: PacketData = serde_json::from_slice(packet.data.as_slice())
			.map_err(|e| Ics04Error::app_module(format!("Failed to decode packet data {:?}", e)))?;
		process_timeout_packet(&mut ctx, packet, &packet_data)
			.map_err(|e| Ics04Error::app_module(e.to_string()))?;

		Ok(())
	}
}

pub struct WeightHandler<T: Config>(PhantomData<T>);

impl<T: Config> Default for WeightHandler<T> {
	fn default() -> Self {
		Self(PhantomData::default())
	}
}

impl<T: Config> CallbackWeight for WeightHandler<T> {
	fn on_chan_open_init(&self) -> Weight {
		<T as Config>::WeightInfo::on_chan_open_init()
	}

	fn on_chan_open_try(&self) -> Weight {
		<T as Config>::WeightInfo::on_chan_open_try()
	}

	fn on_chan_open_ack(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		<T as Config>::WeightInfo::on_chan_open_ack()
	}

	fn on_chan_open_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		<T as Config>::WeightInfo::on_chan_open_confirm()
	}

	fn on_chan_close_init(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		<T as Config>::WeightInfo::on_chan_close_init()
	}

	fn on_chan_close_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		<T as Config>::WeightInfo::on_chan_close_confirm()
	}

	fn on_recv_packet(&self, _packet: &Packet) -> Weight {
		<T as Config>::WeightInfo::on_recv_packet()
	}

	fn on_acknowledgement_packet(
		&self,
		_packet: &Packet,
		_acknowledgement: &Acknowledgement,
	) -> Weight {
		<T as Config>::WeightInfo::on_acknowledgement_packet()
	}

	fn on_timeout_packet(&self, _packet: &Packet) -> Weight {
		<T as Config>::WeightInfo::on_timeout_packet()
	}
}

pub fn full_ibc_denom(packet: &Packet, mut token: PrefixedCoin) -> String {
	if is_receiver_chain_source(packet.source_port.clone(), packet.source_channel, &token.denom) {
		let prefix = TracePrefix::new(packet.source_port.clone(), packet.source_channel);

		token.denom.remove_trace_prefix(&prefix);
		token.denom.to_string()
	} else {
		let prefix = TracePrefix::new(packet.destination_port.clone(), packet.destination_channel);

		token.denom.add_trace_prefix(prefix);
		token.denom.to_string()
	}
}
