#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unreachable_patterns)]
#![allow(clippy::type_complexity)]
#![allow(non_camel_case_types)]
#![deny(
	unused_imports,
	clippy::useless_conversion,
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]

//! Pallet IBC
//! Implements the core ibc features for substrate runtimes.
extern crate alloc;

use codec::{Decode, Encode};
use frame_system::ensure_signed;
pub use pallet::*;
use scale_info::{
	prelude::{
		format,
		string::{String, ToString},
		vec,
	},
	TypeInfo,
};
use sp_runtime::RuntimeDebug;
use sp_std::{marker::PhantomData, prelude::*, str::FromStr};

mod channel;
mod client;
mod connection;
mod errors;
pub mod events;
mod host_functions;
pub mod ics20;
mod ics23;
mod port;
pub mod routing;
mod state_machine;

pub const IBC_DIGEST_ID: [u8; 4] = *b"/IBC";
pub const MODULE_ID: &str = "pallet_ibc";

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Any {
	pub type_url: Vec<u8>,
	pub value: Vec<u8>,
}

impl From<ibc_proto::google::protobuf::Any> for Any {
	fn from(any: ibc_proto::google::protobuf::Any) -> Self {
		Self { type_url: any.type_url.as_bytes().to_vec(), value: any.value }
	}
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
/// Ibc consensus state values
pub struct IbcConsensusState {
	/// Timestamp at which this state root was generated in nanoseconds
	pub timestamp: u64,
	/// IBC Commitment root
	pub commitment_root: Vec<u8>,
}

impl Default for IbcConsensusState {
	// Using a default value of 1 for timestamp because using 0 will generate an
	// error when converting to an ibc::Timestamp in tests and benchmarks
	fn default() -> Self {
		Self { timestamp: 1, commitment_root: vec![] }
	}
}

#[cfg(any(test, feature = "runtime-benchmarks"))]
pub(crate) mod benchmarks;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod impls;
pub mod weight;

pub use weight::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		storage::bounded_btree_map::BoundedBTreeMap,
		traits::{Currency, UnixTime},
	};
	use frame_system::pallet_prelude::*;
	use ibc::core::{
		ics02_client::msgs::create_client,
		ics03_connection::msgs::{conn_open_ack, conn_open_init},
		ics03_connection::{
			connection::Counterparty, msgs::conn_open_init::MsgConnectionOpenInit, version::Version,
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics26_routing::handler::MsgReceipt,
	};

	use crate::{host_functions::HostFunctions, ics23::client_states::ClientStates};
	use composable_traits::defi::DeFiComposableConfig;
	pub use ibc::signer::Signer;
	use ibc_primitives::client_id_from_bytes;
	use tendermint_proto::Protobuf;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ balances::Config
		+ pallet_ibc_ping::Config
		+ parachain_info::Config
		+ transfer::Config
		+ DeFiComposableConfig
		+ assets::Config
	{
		type TimeProvider: UnixTime;
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Currency type of the runtime
		type Currency: Currency<Self::AccountId>;
		/// Prefix for events stored in the Off-chain DB via Indexing API.
		const INDEXING_PREFIX: &'static [u8];
		/// Prefix for ibc connection, should be valid utf8 string bytes
		const CONNECTION_PREFIX: &'static [u8];
		/// This is the key under the global state trie, where this pallet will
		/// incrementally build the ICS23 commitment trie
		const CHILD_TRIE_KEY: &'static [u8];
		/// Expected blocktime
		#[pallet::constant]
		type ExpectedBlockTime: Get<u64>;
		/// benchmarking weight info
		type WeightInfo: WeightInfo;
		/// Origin allowed to create light clients and initiate connections
		type AdminOrigin: EnsureOrigin<Self::Origin>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// client_id , Height => Height
	pub type ClientUpdateHeight<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Blake2_128Concat,
		Vec<u8>,
		Vec<u8>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// client_id , Height => Timestamp
	pub type ClientUpdateTime<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, Vec<u8>, Blake2_128Concat, Vec<u8>, u64, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	pub type ChannelCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	pub type PacketCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// connection_identifier => Vec<(port_id, channel_id)>
	pub type ChannelsConnection<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<(Vec<u8>, Vec<u8>)>, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// counter for clients
	pub type ClientCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// counter for clients
	pub type ConnectionCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// counter for acknowledgments
	pub type AcknowledgementCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// counter for packet reciepts
	pub type PacketReceiptCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// client_id => Vec<Connection_id>
	pub type ConnectionClient<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// height => IbcConsensusState
	pub type HostConsensusStates<T: Config> =
		StorageValue<_, BoundedBTreeMap<u64, IbcConsensusState, ConstU32<250>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T> {
		/// Events emitted by the ibc subsystem
		Events { events: Vec<events::IbcEvent> },
		/// Errors emitted by the ibc subsystem
		Errors { errors: Vec<errors::IbcError> },
	}

	/// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error processing ibc messages
		ProcessingError,
		/// Error decoding some type
		DecodingError,
		/// Error encoding some type
		EncodingError,
		/// Error generating trie proof
		ProofGenerationError,
		/// Client consensus state not found for height
		ConsensusStateNotFound,
		/// Channel not found
		ChannelNotFound,
		/// Client state not found
		ClientStateNotFound,
		/// Connection not found
		ConnectionNotFound,
		/// Packet commitment wasn't found
		PacketCommitmentNotFound,
		/// Packet receipt wasn't found
		PacketReceiptNotFound,
		/// Packet Acknowledgment wasn't found
		PacketAcknowledgmentNotFound,
		/// Error constructing packet
		SendPacketError,
		/// Other forms of errors
		Other,
		/// Invalid route
		InvalidRoute,
		/// Invalid message for extirnsic
		InvalidMessageType,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		u32: From<<T as frame_system::Config>::BlockNumber>,
		T: Send + Sync,
	{
		fn offchain_worker(_n: BlockNumberFor<T>) {
			let _ = Pallet::<T>::packet_cleanup();
		}
	}

	// Dispatch able functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsic", which are often compared to transactions.
	// Dispatch able functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		u32: From<<T as frame_system::Config>::BlockNumber>,
		T: Send + Sync,
	{
		#[pallet::weight(crate::weight::deliver::< T > (messages))]
		#[frame_support::transactional]
		pub fn deliver(origin: OriginFor<T>, messages: Vec<Any>) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			let mut ctx = routing::Context::<T>::new();
			let messages = messages
				.into_iter()
				.filter_map(|message| {
					let type_url = String::from_utf8(message.type_url.clone()).ok()?;
					let is_permissioned = matches!(
						type_url.as_str(),
						conn_open_init::TYPE_URL | conn_open_ack::TYPE_URL | create_client::TYPE_URL
					);
					if is_permissioned {
						return None;
					}
					Some(Ok(ibc_proto::google::protobuf::Any { type_url, value: message.value }))
				})
				.collect::<Result<Vec<ibc_proto::google::protobuf::Any>, Error<T>>>()?;
			Self::execute_ibc_messages(&mut ctx, messages);

			Ok(())
		}

		/// We permission the initiation and acceptance of connections, this is critical for security.
		/// 
		/// [see here](https://github.com/ComposableFi/ibc-rs/issues/31)
		#[pallet::weight(crate::weight::deliver::< T > (messages))]
		#[frame_support::transactional]
		pub fn deliver_permissioned(origin: OriginFor<T>, messages: Vec<Any>) -> DispatchResult {
			<T as Config>::AdminOrigin::ensure_origin(origin)?;

			let mut ctx = routing::Context::<T>::new();
			let messages = messages
				.into_iter()
				.filter_map(|message| {
					let type_url = String::from_utf8(message.type_url.clone()).ok()?;
					let is_permissioned = matches!(
						type_url.as_str(),
						conn_open_init::TYPE_URL | conn_open_ack::TYPE_URL | create_client::TYPE_URL
					);
					if !is_permissioned {
						return None;
					}
					Some(Ok(ibc_proto::google::protobuf::Any { type_url, value: message.value }))
				})
				.collect::<Result<Vec<ibc_proto::google::protobuf::Any>, Error<T>>>()?;
			Self::execute_ibc_messages(&mut ctx, messages);

			Ok(())
		}
	}
}
