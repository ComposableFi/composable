#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use core::{
	fmt::{Debug, Formatter, Write},
	write,
};
use frame_support::{
	dispatch::DispatchResult,
	metadata::StorageEntryModifier::Default,
	traits::IsSubType,
	weights::{ClassifyDispatch, DispatchClass, Pays, PaysFee, WeighData, Weight},
};
use frame_system::ensure_signed;
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
use log::info;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{Bounded, DispatchInfoOf, SaturatedConversion, Saturating, SignedExtension},
	transaction_validity::{
		InvalidTransaction, TransactionValidity, TransactionValidityError, ValidTransaction,
	},
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

	/// Our pallet's configuration trait. All our types and constants go in here. If the
	/// pallet is dependent on specific other pallets, then their configuration traits
	/// should be added to our implied traits list.
	///
	/// `frame_system::Config` should always be included.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
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
			let port_id = PortId::from_str(PORT_ID).map_err(|_| Error::<T>::ErrorBindingPort)?;
			Ok(())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Success,
	}

	#[pallet::storage]
	/// Port Capability
	pub type Capability<T> = StorageValue<_, u64, OptionQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// Error generating port id
		ErrorBindingPort,
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
