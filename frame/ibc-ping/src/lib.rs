#![cfg_attr(not(feature = "std"), no_std)]

use core::{fmt::Formatter, write};
use frame_support::dispatch::DispatchResult;
use ibc::{
	core::{
		ics04_channel::{
			channel::{Counterparty, Order},
			error::Error as Ics04Error,
			msgs::acknowledgement::Acknowledgement,
			packet::Packet,
			Version,
		},
		ics05_port::capabilities::{Capability as RawCapability, ChannelCapability},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
		ics26_routing::context::{
			Acknowledgement as GenericAcknowledgement, Module, ModuleOutput, OnRecvPacketAck,
		},
	},
	signer::Signer,
};
use scale_info::prelude::string::{String, ToString};
use sp_std::{marker::PhantomData, prelude::*, vec};

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

pub const MODULE_ID: &'static str = "pallet-ibc-ping";
pub const PORT_ID: &'static str = "ping";

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
	data: Vec<u8>,
	timeout_height_offset: u64,
	timeout_timestamp_offset: u64,
	channel_id: Vec<u8>,
	dest_port_id: Vec<u8>,
	dest_channel_id: Vec<u8>,
}

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
	use sp_std::str::FromStr;
	// Import various types used to declare pallet in scope.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use ibc::core::ics04_channel::channel::{ChannelEnd, Order, State};
	use ibc_primitives::SendPacketData;
	use ibc_trait::{connection_id_from_bytes, port_id_from_bytes, IbcTrait, OpenChannelParams};

	/// Our pallet's configuration trait. All our types and constants go in here. If the
	/// pallet is dependent on specific other pallets, then their configuration traits
	/// should be added to our implied traits list.
	///
	/// `frame_system::Config` should always be included.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type IbcHandler: ibc_trait::IbcTrait;
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
		pub fn bind_ibc_port(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			if Capability::<T>::get().is_some() {
				return Err(Error::<T>::PortAlreadyBound.into())
			}
			let port_id = PortId::from_str(PORT_ID).map_err(|_| Error::<T>::ErrorBindingPort)?;
			let capability =
				T::IbcHandler::bind_port(port_id).map_err(|_| Error::<T>::ErrorBindingPort)?;
			let cap = capability.index();
			Capability::<T>::put(cap);
			Self::deposit_event(Event::<T>::PortBound);
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn open_channel(origin: OriginFor<T>, params: OpenChannelParams) -> DispatchResult {
			ensure_root(origin)?;
			let state = match params.state {
				0 => State::Uninitialized,
				1 => State::Init,
				2 => State::TryOpen,
				3 => State::Open,
				4 => State::Closed,
				_ => return Err(Error::<T>::InvalidParams.into()),
			};
			let order = match params.order {
				0 => Order::None,
				1 => Order::Unordered,
				2 => Order::Ordered,
				_ => return Err(Error::<T>::InvalidParams.into()),
			};

			let connection_id = connection_id_from_bytes(params.connection_id)
				.map_err(|_| Error::<T>::InvalidParams)?;
			let version =
				String::from_utf8(params.version).map_err(|_| Error::<T>::InvalidParams)?;
			let counterparty_port_id = port_id_from_bytes(params.counterparty_port_id)
				.map_err(|_| Error::<T>::InvalidParams)?;
			let counterparty = Counterparty::new(counterparty_port_id, None);
			let channel_end = ChannelEnd::new(
				state,
				order,
				counterparty,
				vec![connection_id],
				Version::new(version),
			);

			let port_id = port_id_from_bytes(PORT_ID.as_bytes().to_vec())
				.map_err(|_| Error::<T>::ChannelInitError)?;
			let capability = Capability::<T>::get().ok_or(Error::<T>::MissingPortCapability)?;
			let capability = RawCapability::from(capability);
			T::IbcHandler::open_channel(port_id, capability.into(), channel_end)
				.map_err(|_| Error::<T>::ChannelInitError)?;
			Self::deposit_event(Event::<T>::ChannelOpened);
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn send_ping(origin: OriginFor<T>, params: SendPingParams) -> DispatchResult {
			ensure_root(origin)?;
			let capability = Capability::<T>::get().ok_or(Error::<T>::MissingPortCapability)?;
			let capability = RawCapability::from(capability);
			let send_packet = SendPacketData {
				data: params.data,
				timeout_height_offset: params.timeout_height_offset,
				timeout_timestamp_offset: params.timeout_timestamp_offset,
				capability,
				port_id: PORT_ID.as_bytes().to_vec(),
				channel_id: params.channel_id,
				dest_port_id: params.dest_port_id,
				dest_channel_id: params.dest_channel_id,
			};
			T::IbcHandler::send_packet(send_packet).map_err(|_| Error::<T>::PacketSendError)?;
			Self::deposit_event(Event::<T>::PacketSent);
			Ok(())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Port for pallet has been bound
		PortBound,
		/// A send packet has been registered
		PacketSent,
		/// A channel has been opened
		ChannelOpened,
	}

	#[pallet::storage]
	/// Port Capability
	pub type Capability<T> = StorageValue<_, u64, OptionQuery>;

	#[pallet::storage]
	/// A vector of Vec<channel_id>
	pub type Channels<T> = StorageValue<_, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// Error generating port id
		ErrorBindingPort,
		/// Port already bound
		PortAlreadyBound,
		/// Invalid params passed
		InvalidParams,
		/// Error opening channel
		ChannelInitError,
		/// Missing port capability
		MissingPortCapability,
		/// Error registering packet
		PacketSendError,
	}
}

#[derive(Clone)]
pub struct IbcHandler<T: Config>(PhantomData<T>);

pub struct PingAcknowledgement(Vec<u8>);

impl AsRef<[u8]> for PingAcknowledgement {
	fn as_ref(&self) -> &[u8] {
		self.0.as_slice()
	}
}

impl GenericAcknowledgement for PingAcknowledgement {}

impl<T: Config> core::fmt::Debug for IbcHandler<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "pallet-ibc-ping")
	}
}

impl<T: Config> IbcHandler<T> {
	pub fn new() -> Self {
		IbcHandler(PhantomData::default())
	}
}

impl<T: Config + Send + Sync> Module for IbcHandler<T> {
	fn on_chan_open_init(
		&mut self,
		_output: &mut ModuleOutput,
		_order: Order,
		_connection_hops: &[ConnectionId],
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_channel_cap: &ChannelCapability,
		_counterparty: &Counterparty,
		_version: &Version,
	) -> Result<(), Ics04Error> {
		log::info!("Channel initialised");
		Ok(())
	}

	fn on_chan_open_try(
		&mut self,
		_output: &mut ModuleOutput,
		_order: Order,
		_connection_hops: &[ConnectionId],
		port_id: &PortId,
		channel_id: &ChannelId,
		_channel_cap: &ChannelCapability,
		counterparty: &Counterparty,
		counterparty_version: &Version,
	) -> Result<Version, Ics04Error> {
		log::info!("Channel initialised {:?}, {:?}, {:?}", channel_id, port_id, counterparty);
		Ok(counterparty_version.clone())
	}

	fn on_chan_open_ack(
		&mut self,
		_output: &mut ModuleOutput,
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
		let channel_id = channel_id.to_string().as_bytes().to_vec();
		let _ = Channels::<T>::try_mutate::<_, Error<T>, _>(|val| {
			val.push(channel_id);
			Ok(())
		});
		Ok(())
	}

	fn on_chan_open_confirm(
		&mut self,
		_output: &mut ModuleOutput,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<(), Ics04Error> {
		log::info!("Channel open confirmed {:?}, {:?}", channel_id, port_id);
		let channel_id = channel_id.to_string().as_bytes().to_vec();
		let _ = Channels::<T>::try_mutate::<_, Error<T>, _>(|val| {
			val.push(channel_id);
			Ok(())
		});
		Ok(())
	}

	fn on_chan_close_init(
		&mut self,
		_output: &mut ModuleOutput,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<(), Ics04Error> {
		log::info!("Channel close started {:?} {:?}", channel_id, port_id);
		let channel_id = channel_id.to_string().as_bytes().to_vec();
		let _ = Channels::<T>::try_mutate::<_, Error<T>, _>(|val| {
			let new_val = val
				.into_iter()
				.filter_map(|ch| if ch != &channel_id { Some(ch.clone()) } else { None })
				.collect::<Vec<_>>();
			*val = new_val;
			Ok(())
		});
		Ok(())
	}

	fn on_chan_close_confirm(
		&mut self,
		_output: &mut ModuleOutput,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<(), Ics04Error> {
		log::info!("Channel close confirmed\n ChannelId: {:?}, PortId: {:?}", channel_id, port_id);
		let channel_id = channel_id.to_string().as_bytes().to_vec();
		let _ = Channels::<T>::try_mutate::<_, Error<T>, _>(|val| {
			let new_val = val
				.into_iter()
				.filter_map(|ch| if ch != &channel_id { Some(ch.clone()) } else { None })
				.collect::<Vec<_>>();
			*val = new_val;
			Ok(())
		});
		Ok(())
	}

	fn on_recv_packet(
		&self,
		_output: &mut ModuleOutput,
		packet: &Packet,
		_relayer: &Signer,
	) -> OnRecvPacketAck {
		let success = "ping-success".as_bytes().to_vec();
		log::info!("Received Packet {:?}", packet);
		OnRecvPacketAck::Successful(Box::new(PingAcknowledgement(success)), Box::new(|_| {}))
	}

	fn on_acknowledgement_packet(
		&mut self,
		_output: &mut ModuleOutput,
		packet: &Packet,
		acknowledgement: &Acknowledgement,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		log::info!("Acknowledged Packet {:?} {:?}", packet, acknowledgement);
		Ok(())
	}

	fn on_timeout_packet(
		&mut self,
		_output: &mut ModuleOutput,
		packet: &Packet,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		log::info!("Timout Packet {:?}", packet);
		Ok(())
	}
}
