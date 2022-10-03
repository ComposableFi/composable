#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::{
	format,
	string::{String, ToString},
};
use core::{fmt::Formatter, str::FromStr, write};
use frame_support::dispatch::{DispatchResult, Weight};
use ibc::{
	core::{
		ics04_channel::{
			channel::{Counterparty, Order},
			error::Error as Ics04Error,
			msgs::acknowledgement::Acknowledgement,
			packet::Packet,
			Version,
		},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
		ics26_routing::context::{
			Acknowledgement as GenericAcknowledgement, Module, ModuleOutputBuilder, OnRecvPacketAck,
		},
	},
	signer::Signer,
};
use ibc_primitives::{port_id_from_bytes, CallbackWeight, IbcHandler, SendPacketData};
use sp_std::{marker::PhantomData, prelude::*};
// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

pub const MODULE_ID: &str = "PalletIbcPing";
pub const PORT_ID: &str = "ping";
pub const VERSION: &str = "ping-1";

#[derive(
	Clone,
	PartialEq,
	Eq,
	codec::Encode,
	codec::Decode,
	frame_support::RuntimeDebug,
	scale_info::TypeInfo,
)]
pub struct SendPingParams {
	pub data: Vec<u8>,
	/// Timeout height offset relative to the client latest height
	pub timeout_height_offset: u64,
	/// Time out timestamp offset relative to client's latest height
	pub timeout_timestamp_offset: u64,
	// Channel counter, for example counter for channel-0 is 0
	pub channel_id: u64,
}

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
	// Import various types used to declare pallet in scope.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Our pallet's configuration trait. All our types and constants go in here. If the
	/// pallet is dependent on specific other pallets, then their configuration traits
	/// should be added to our implied traits list.
	///
	/// `frame_system::Config` should always be included.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// ibc subsystem
		type IbcHandler: ibc_primitives::IbcHandler;
	}

	// Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
	// method.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn send_ping(origin: OriginFor<T>, params: SendPingParams) -> DispatchResult {
			ensure_root(origin)?;
			Self::send_ping_impl(params).map_err(|e| {
				log::trace!(target: "pallet_ibc_ping", "[send_ping] error: {:?}", e);
				Error::<T>::PacketSendError
			})?;
			Self::deposit_event(Event::<T>::PacketSent);
			Ok(())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A send packet has been registered
		PacketSent,
		/// A channel has been opened
		ChannelOpened { channel_id: Vec<u8>, port_id: Vec<u8> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Invalid params passed
		InvalidParams,
		/// Error opening channel
		ChannelInitError,
		/// Error registering packet
		PacketSendError,
	}
}

impl<T: Config> Pallet<T> {
	pub fn send_ping_impl(params: SendPingParams) -> Result<(), ibc_primitives::Error> {
		let channel_id = ChannelId::new(params.channel_id);
		let send_packet = SendPacketData {
			data: b"ping".to_vec(),
			timeout_height_offset: params.timeout_height_offset,
			timeout_timestamp_offset: params.timeout_timestamp_offset,
			port_id: port_id_from_bytes(PORT_ID.as_bytes().to_vec())
				.expect("Valid port id expected"),
			channel_id,
		};
		T::IbcHandler::send_packet(send_packet)
	}
}

#[derive(Clone, Eq, PartialEq)]
pub struct IbcModule<T: Config>(PhantomData<T>);

impl<T: Config> Default for IbcModule<T> {
	fn default() -> Self {
		Self(PhantomData::default())
	}
}

pub struct PingAcknowledgement(Vec<u8>);

impl AsRef<[u8]> for PingAcknowledgement {
	fn as_ref(&self) -> &[u8] {
		self.0.as_slice()
	}
}

impl GenericAcknowledgement for PingAcknowledgement {}

impl<T: Config> core::fmt::Debug for IbcModule<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "pallet-ibc-ping")
	}
}

impl<T: Config + Send + Sync> Module for IbcModule<T> {
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
		log::info!("Channel initialized");
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
		if counterparty_version.to_string() != VERSION.to_string() ||
			version.to_string() != VERSION.to_string()
		{
			return Err(Ics04Error::no_common_version())
		}

		if order != Order::Ordered {
			return Err(Ics04Error::unknown_order_type(order.to_string()))
		}

		let ping_port = PortId::from_str(PORT_ID).unwrap();
		if counterparty.port_id() != &ping_port || port_id != &ping_port {
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
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty_version: &Version,
	) -> Result<(), Ics04Error> {
		log::info!(
			"Channel acknowledged {:?}, {:?}, {:?}",
			channel_id,
			port_id,
			counterparty_version
		);
		Ok(())
	}

	fn on_chan_open_confirm(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<(), Ics04Error> {
		log::info!("Channel open confirmed {:?}, {:?}", channel_id, port_id);
		Ok(())
	}

	fn on_chan_close_init(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<(), Ics04Error> {
		log::info!("Channel close started {:?} {:?}", channel_id, port_id);
		Ok(())
	}

	fn on_chan_close_confirm(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<(), Ics04Error> {
		log::info!("Channel close confirmed\n ChannelId: {:?}, PortId: {:?}", channel_id, port_id);
		Ok(())
	}

	fn on_recv_packet(
		&self,
		_output: &mut ModuleOutputBuilder,
		packet: &Packet,
		_relayer: &Signer,
	) -> OnRecvPacketAck {
		let success = "ping-success".as_bytes().to_vec();
		let data = String::from_utf8(packet.data.clone()).ok();
		log::info!("Received Packet Sequence {:?}, Packet Data {:?}", packet.sequence, data);
		let packet = packet.clone();
		OnRecvPacketAck::Successful(
			Box::new(PingAcknowledgement(success.clone())),
			Box::new(move |_| {
				T::IbcHandler::write_acknowledgement(&packet, success)
					.map_err(|e| format!("{:?}", e))
			}),
		)
	}

	fn on_acknowledgement_packet(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		packet: &Packet,
		acknowledgement: &Acknowledgement,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		log::info!("Acknowledged Packet {:?} {:?}", packet, acknowledgement);
		Ok(())
	}

	fn on_timeout_packet(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		packet: &Packet,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		log::info!("Timeout Packet {:?}", packet);
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
		0
	}

	fn on_chan_open_try(&self) -> Weight {
		0
	}

	fn on_chan_open_ack(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		0
	}

	fn on_chan_open_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		0
	}

	fn on_chan_close_init(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		0
	}

	fn on_chan_close_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		0
	}

	fn on_recv_packet(&self, _packet: &Packet) -> Weight {
		0
	}

	fn on_acknowledgement_packet(
		&self,
		_packet: &Packet,
		_acknowledgement: &Acknowledgement,
	) -> Weight {
		0
	}

	fn on_timeout_packet(&self, _packet: &Packet) -> Weight {
		0
	}
}
