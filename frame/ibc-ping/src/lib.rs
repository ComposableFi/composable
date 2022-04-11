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
		ics05_port::capabilities::ChannelCapability,
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
		ics26_routing::context::{Module, ModuleOutput, OnRecvPacketAck},
	},
	signer::Signer,
};
use sp_std::{marker::PhantomData, prelude::*};

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

pub const MODULE_ID: &'static str = "pallet-ibc-ping";
pub const PORT_ID: &'static str = "ibc-ping";

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
	use sp_std::str::FromStr;
	// Import various types used to declare pallet in scope.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use ibc_trait::IbcTrait;

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
		pub fn open_channel(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			Ok(())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PortBound,
	}

	#[pallet::storage]
	/// Port Capability
	pub type Capability<T> = StorageValue<_, u64, OptionQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// Error generating port id
		ErrorBindingPort,
		/// Port already bound
		PortAlreadyBound,
	}
}

#[derive(Clone)]
pub struct IbcHandler<T: Config>(PhantomData<T>);

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
		todo!()
	}

	fn on_chan_open_try(
		&mut self,
		_output: &mut ModuleOutput,
		_order: Order,
		_connection_hops: &[ConnectionId],
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_channel_cap: &ChannelCapability,
		_counterparty: &Counterparty,
		_counterparty_version: &Version,
	) -> Result<Version, Ics04Error> {
		todo!()
	}

	fn on_chan_open_ack(
		&mut self,
		_output: &mut ModuleOutput,
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_counterparty_version: &Version,
	) -> Result<(), Ics04Error> {
		todo!()
	}

	fn on_chan_open_confirm(
		&mut self,
		_output: &mut ModuleOutput,
		_port_id: &PortId,
		_channel_id: &ChannelId,
	) -> Result<(), Ics04Error> {
		todo!()
	}

	fn on_chan_close_init(
		&mut self,
		_output: &mut ModuleOutput,
		_port_id: &PortId,
		_channel_id: &ChannelId,
	) -> Result<(), Ics04Error> {
		todo!()
	}

	fn on_chan_close_confirm(
		&mut self,
		_output: &mut ModuleOutput,
		_port_id: &PortId,
		_channel_id: &ChannelId,
	) -> Result<(), Ics04Error> {
		todo!()
	}

	fn on_recv_packet(
		&self,
		_output: &mut ModuleOutput,
		_packet: &Packet,
		_relayer: &Signer,
	) -> OnRecvPacketAck {
		todo!()
	}

	fn on_acknowledgement_packet(
		&mut self,
		_output: &mut ModuleOutput,
		_packet: &Packet,
		_acknowledgement: &Acknowledgement,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		todo!()
	}

	fn on_timeout_packet(
		&mut self,
		_output: &mut ModuleOutput,
		_packet: &Packet,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		todo!()
	}
}
