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
use ibc::core::ics03_connection::msgs::conn_open_init::TYPE_URL as CONNECTION_OPEN_INIT_TYPE_URL;
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

pub const IBC_DIGEST_ID: [u8; 4] = *b"/IBC";
pub const MODULE_ID: &str = "pallet_ibc";

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Any {
	pub type_url: Vec<u8>,
	pub value: Vec<u8>,
}

pub(crate) type RawVersion = (Vec<u8>, Vec<Vec<u8>>);

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ConnectionParams {
	/// A vector of (identifier, features) all encoded as Utf8 string bytes
	pub version: RawVersion,
	/// Utf8 client_id bytes
	pub client_id: Vec<u8>,
	/// Counterparty client id
	pub counterparty_client_id: Vec<u8>,
	/// Counter party commitment prefix
	pub commitment_prefix: Vec<u8>,
	/// Delay period in nanoseconds
	pub delay_period: u64,
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
		ics02_client::msgs::create_client::TYPE_URL as CREATE_CLIENT_TYPE_URL,
		ics03_connection::{
			connection::Counterparty, msgs::conn_open_init::MsgConnectionOpenInit, version::Version,
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics26_routing::handler::MsgReceipt,
	};

	use crate::{host_functions::HostFunctions, ics23::client_states::ClientStates};
	use composable_traits::defi::DeFiComposableConfig;
	pub use ibc::signer::Signer;
	use ibc_trait::client_id_from_bytes;
	use sp_runtime::{generic::DigestItem, SaturatedConversion};
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
	/// counter for packet receipts
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
		/// Raw Ibc events
		IbcEvents { events: Vec<events::IbcEvent> },
		/// Ibc errors
		IbcErrors { errors: Vec<errors::IbcError> },
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
		/// Invalid message for extrinsic
		InvalidMessageType,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		u32: From<<T as frame_system::Config>::BlockNumber>,
		T: Send + Sync,
	{
		fn on_finalize(_n: BlockNumberFor<T>) {
			let root = Pallet::<T>::extract_ibc_commitment_root();
			let height = impls::host_height::<T>();
			let timestamp = T::TimeProvider::now().as_nanos().saturated_into::<u64>();
			let ibc_cs = IbcConsensusState { timestamp, commitment_root: root.clone() };
			let res = HostConsensusStates::<T>::try_mutate::<_, &'static str, _>(|val| {
				// Try inserting the new consensus state, if the bounded map has reached it's
				// limit this operation is a noop and just returns an error containing the
				// values that we tried inserting if not the value is inserted successfully
				// without any error
				if let Err((height, ibc_cs)) = val.try_insert(height, ibc_cs) {
					// If map is full, remove the oldest consensus state
					// Get the key to the oldest state
					let key = val.keys().cloned().next().ok_or("No keys in map")?;
					// Prune the oldest consensus state.
					val.remove(&key).ok_or("Unable to prune map")?;
					// Insert the new consensus state.
					val.try_insert(height, ibc_cs)
						.map_err(|_| "Failed to insert new consensus state")?;
				}
				Ok(())
			});
			if res.is_err() {
				log::error!("[pallet_ibc_on_finalize]: Failed to insert new consensus state");
			}
			let log = DigestItem::Consensus(IBC_DIGEST_ID, root);
			<frame_system::Pallet<T>>::deposit_log(log);
		}

		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			<T as Config>::WeightInfo::on_finalize(
				ClientCounter::<T>::get(),
				ConnectionCounter::<T>::get(),
				ChannelCounter::<T>::get(),
				PacketCounter::<T>::get(),
				AcknowledgementCounter::<T>::get(),
				PacketReceiptCounter::<T>::get(),
			)
		}

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
					if type_url.as_str() == CREATE_CLIENT_TYPE_URL {
						return None
					}
					Some(Ok(ibc_proto::google::protobuf::Any { type_url, value: message.value }))
				})
				.collect::<Result<Vec<ibc_proto::google::protobuf::Any>, Error<T>>>()?;

			let (events, logs, errors) = messages.into_iter().fold(
				(vec![], vec![], vec![]),
				|(mut events, mut logs, mut errors), msg| {
					match ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(
						&mut ctx, msg,
					) {
						Ok(MsgReceipt { events: temp_events, log: temp_logs }) => {
							events.extend(temp_events);
							logs.extend(temp_logs);
						},
						Err(e) => errors.push(e),
					}
					(events, logs, errors)
				},
			);

			log::trace!(target: "pallet_ibc", "[pallet_ibc_deliver]: logs: {:?}", logs);
			log::trace!(target: "pallet_ibc", "[pallet_ibc_deliver]: errors: {:?}", errors);

			Self::deposit_event(events.into());
			Self::deposit_event(errors.into());
			Ok(())
		}

		#[pallet::weight(< T as Config >::WeightInfo::create_client())]
		#[frame_support::transactional]
		pub fn create_client(origin: OriginFor<T>, msg: Any) -> DispatchResult {
			<T as Config>::AdminOrigin::ensure_origin(origin)?;
			let mut ctx = routing::Context::<T>::new();
			let type_url =
				String::from_utf8(msg.type_url.clone()).map_err(|_| Error::<T>::DecodingError)?;
			if type_url.as_str() != CREATE_CLIENT_TYPE_URL {
				return Err(Error::<T>::InvalidMessageType.into())
			}
			let msg = ibc_proto::google::protobuf::Any { type_url, value: msg.value };

			let MsgReceipt { events, log } =
				ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg)
					.map_err(|_| Error::<T>::ProcessingError)?;

			log::trace!(target: "pallet_ibc", "[pallet_ibc_deliver]: logs: {:?}", log);
			Self::deposit_event(events.into());
			Ok(())
		}

		#[pallet::weight(< T as Config >::WeightInfo::initiate_connection())]
		#[frame_support::transactional]
		pub fn initiate_connection(
			origin: OriginFor<T>,
			params: ConnectionParams,
		) -> DispatchResult {
			<T as Config>::AdminOrigin::ensure_origin(origin)?;
			let client_id =
				client_id_from_bytes(params.client_id).map_err(|_| Error::<T>::DecodingError)?;
			if !ClientStates::<T>::contains_key(&client_id) {
				return Err(Error::<T>::ClientStateNotFound.into())
			}

			let counterparty_client_id = client_id_from_bytes(params.counterparty_client_id)
				.map_err(|_| Error::<T>::DecodingError)?;
			let identifier = params.version.0;
			let features = params.version.1;
			let identifier =
				String::from_utf8(identifier).map_err(|_| Error::<T>::DecodingError)?;
			let features = features
				.into_iter()
				.map(String::from_utf8)
				.collect::<Result<Vec<_>, _>>()
				.map_err(|_| Error::<T>::DecodingError)?;
			let raw_version =
				ibc_proto::ibc::core::connection::v1::Version { identifier, features };
			let version: Version = raw_version.try_into().map_err(|_| Error::<T>::DecodingError)?;

			let commitment_prefix: CommitmentPrefix =
				params.commitment_prefix.try_into().map_err(|_| Error::<T>::DecodingError)?;
			let counterparty = Counterparty::new(counterparty_client_id, None, commitment_prefix);
			let delay_period = core::time::Duration::from_nanos(params.delay_period);
			let value = MsgConnectionOpenInit {
				client_id,
				counterparty,
				version: Some(version),
				delay_period,
				signer: Signer::from_str(MODULE_ID).map_err(|_| Error::<T>::DecodingError)?,
			}
			.encode_vec();
			let msg = ibc_proto::google::protobuf::Any {
				type_url: CONNECTION_OPEN_INIT_TYPE_URL.to_string(),
				value,
			};
			let mut ctx = routing::Context::<T>::new();
			let result =
				ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg)
					.map_err(|_| Error::<T>::ProcessingError)?;
			Self::deposit_event(result.events.into());
			Ok(())
		}
	}
}
