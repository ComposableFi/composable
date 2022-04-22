#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unreachable_patterns)]
#![allow(clippy::type_complexity)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use codec::{Decode, Encode};
use frame_system::ensure_signed;
pub use pallet::*;
use scale_info::{
	prelude::{format, string::String, vec},
	TypeInfo,
};
use sp_runtime::RuntimeDebug;
use sp_std::{marker::PhantomData, prelude::*, str::FromStr};

mod channel;
mod client;
mod connection;
mod port;
mod routing;

pub const IBC_DIGEST_ID: [u8; 4] = *b"/IBC";

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Any {
	pub type_url: Vec<u8>,
	pub value: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ConnectionParams {
	/// A vector of (identifer, features) all encoded as Utf8 string bytes
	pub versions: Vec<(Vec<u8>, Vec<Vec<u8>>)>,
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
pub struct IbcConsensusState {
	/// Timestamp at which this state root was generated in nanoseconds
	pub timestamp: u64,
	/// IBC Commitment root
	pub root: Vec<u8>,
}

impl Default for IbcConsensusState {
	fn default() -> Self {
		Self { timestamp: 0, root: vec![] }
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod impls;
pub mod weight;
pub mod weights;

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
		ics03_connection::{
			connection::{ConnectionEnd, Counterparty, State},
			context::ConnectionKeeper,
			version::Version,
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::ConnectionId,
	};

	use ibc_trait::client_id_from_bytes;
	use sp_runtime::{generic::DigestItem, SaturatedConversion};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + balances::Config + pallet_ibc_ping::Config {
		type TimeProvider: UnixTime;
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Currency type of the runtime
		type Currency: Currency<Self::AccountId>;
		/// Prefix for events stored in the Off-chain DB via Indexing API.
		const INDEXING_PREFIX: &'static [u8];
		/// Prefix for ibc connection
		const CONNECTION_PREFIX: &'static [u8];
		#[pallet::constant]
		type ExpectedBlockTime: Get<u64>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	/// client_id => ClientState
	pub type ClientStates<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	/// client_id, height => ConsensusState
	pub type ConsensusStates<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Blake2_128Concat,
		Vec<u8>,
		Vec<u8>,
		ValueQuery,
	>;

	#[pallet::storage]
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
	/// client_id , Height => Timestamp
	pub type ClientUpdateTime<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Blake2_128Concat,
		Vec<u8>,
		Vec<u8>,
		ValueQuery,
	>;

	#[pallet::storage]
	/// connection_id => ConnectionEnd
	pub type Connections<T: Config> =
		CountedStorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	pub type ChannelCounter<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	/// (port_identifier, channel_identifier) => ChannelEnd
	pub type Channels<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Blake2_128Concat,
		Vec<u8>,
		Vec<u8>,
		ValueQuery,
	>;

	#[pallet::storage]
	/// connection_identifier => Vec<(port_id, channel_id)>
	pub type ChannelsConnection<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<(Vec<u8>, Vec<u8>)>, ValueQuery>;

	#[pallet::storage]
	/// capability_name => capability
	pub type Capabilities<T: Config> =
		CountedStorageMap<_, Blake2_128Concat, Vec<u8>, u64, OptionQuery>;

	#[pallet::storage]
	/// (port_identifier, channel_identifier) => Sequence
	pub type NextSequenceSend<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, Vec<u8>, Blake2_128Concat, Vec<u8>, u64, ValueQuery>;

	#[pallet::storage]
	/// (port_identifier, channel_identifier) => Sequence
	pub type NextSequenceRecv<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, Vec<u8>, Blake2_128Concat, Vec<u8>, u64, ValueQuery>;

	#[pallet::storage]
	/// (port_identifier, channel_identifier) = Sequence
	pub type NextSequenceAck<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, Vec<u8>, Blake2_128Concat, Vec<u8>, u64, ValueQuery>;

	#[pallet::storage]
	/// (port_identifier, channel_identifier, Sequence) => Hash
	pub type Acknowledgements<T: Config> =
		StorageMap<_, Blake2_128Concat, (Vec<u8>, Vec<u8>, u64), Vec<u8>, ValueQuery>;

	#[pallet::storage]
	/// clientId => ClientType
	pub type Clients<T: Config> =
		CountedStorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	/// client_id => Vec<Connection_id>
	pub type ConnectionClient<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::storage]
	/// (port_id, channel_id, sequence) => receipt
	pub type PacketReceipt<T: Config> =
		StorageMap<_, Blake2_128Concat, (Vec<u8>, Vec<u8>, u64), Vec<u8>, ValueQuery>;

	#[pallet::storage]
	/// (port_id, channel_id, sequence) => hash
	pub type PacketCommitment<T: Config> =
		StorageMap<_, Blake2_128Concat, (Vec<u8>, Vec<u8>, u64), Vec<u8>, ValueQuery>;

	#[pallet::storage]
	/// height => IbcConsensusState
	pub type CommitmentRoot<T: Config> =
		StorageValue<_, BoundedBTreeMap<u64, IbcConsensusState, ConstU32<250>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T> {
		/// Processed incoming ibc messages
		ProcessedIBCMessages,
		/// Initiated a new connection
		ConnectionInitiated,
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
		/// Client state not found
		ClientStateNotFound,
		/// Error constructing packet
		SendPacketError,
		/// Other forms of errors
		Other,
		/// Invalid route
		InvalidRoute,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		u32: From<<T as frame_system::Config>::BlockNumber>,
	{
		fn on_finalize(_n: BlockNumberFor<T>) {
			let root = Pallet::<T>::extract_ibc_state_root();
			if let Ok(root) = root {
				let height = crate::impls::host_height::<T>();
				let timestamp = T::TimeProvider::now().as_nanos().saturated_into::<u64>();
				let ibc_cs = IbcConsensusState { timestamp, root: root.clone() };
				let _ = CommitmentRoot::<T>::try_mutate::<_, (), _>(|val| {
					if let Err((height, ibc_cs)) = val.try_insert(height, ibc_cs) {
						let first_key = val.keys().cloned().next();
						if let Some(key) = first_key {
							val.remove(&key).ok_or(())?;
							val.try_insert(height, ibc_cs).map_err(|_| ())?;
						}
					}
					Ok(())
				});
				let log = DigestItem::Consensus(IBC_DIGEST_ID, root);
				<frame_system::Pallet<T>>::deposit_log(log);
			}
		}

		fn offchain_worker(_n: BlockNumberFor<T>) {
			Pallet::<T>::packet_cleanup();
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
		#[pallet::weight(0)]
		#[frame_support::transactional]
		pub fn deliver(origin: OriginFor<T>, messages: Vec<Any>) -> DispatchResult {
			let _sender = ensure_signed(origin)?;
			let mut ctx = routing::Context::<T>::new();
			let messages = messages
				.into_iter()
				.map(|message| {
					let type_url = String::from_utf8(message.type_url.clone())
						.map_err(|_| Error::DecodingError)?;
					Ok(ibc_proto::google::protobuf::Any { type_url, value: message.value })
				})
				.collect::<Result<Vec<ibc_proto::google::protobuf::Any>, Error<T>>>()?;

			let result = messages
				.into_iter()
				.map(|msg| ibc::core::ics26_routing::handler::deliver(&mut ctx, msg))
				.collect::<Result<Vec<_>, _>>()
				.map_err(|_| Error::<T>::ProcessingError)?;

			log::trace!("result: {:?}", result);
			Self::deposit_event(Event::<T>::ProcessedIBCMessages);
			Ok(())
		}
		#[pallet::weight(0)]
		#[frame_support::transactional]
		pub fn initiate_connection(
			origin: OriginFor<T>,
			params: ConnectionParams,
		) -> DispatchResult {
			ensure_root(origin)?;
			if !ClientStates::<T>::contains_key(params.client_id.clone()) {
				return Err(Error::<T>::ClientStateNotFound.into())
			}
			let client_id =
				client_id_from_bytes(params.client_id).map_err(|_| Error::<T>::DecodingError)?;
			let connection_id = ConnectionId::new(Connections::<T>::count() as u64);
			let counterparty_client_id = client_id_from_bytes(params.counterparty_client_id)
				.map_err(|_| Error::<T>::DecodingError)?;
			let versions = params
				.versions
				.into_iter()
				.map(|(identifier, features)| {
					let identifier =
						String::from_utf8(identifier).map_err(|_| Error::<T>::DecodingError)?;
					let features = features
						.into_iter()
						.map(|feat| String::from_utf8(feat))
						.collect::<Result<Vec<_>, _>>()
						.map_err(|_| Error::<T>::DecodingError)?;
					let raw_version =
						ibc_proto::ibc::core::connection::v1::Version { identifier, features };
					let version: Version =
						raw_version.try_into().map_err(|_| Error::<T>::DecodingError)?;
					Ok(version)
				})
				.collect::<Result<Vec<_>, Error<T>>>()?;
			let commitment_prefix: CommitmentPrefix =
				params.commitment_prefix.try_into().map_err(|_| Error::<T>::DecodingError)?;
			let counterparty = Counterparty::new(counterparty_client_id, None, commitment_prefix);
			let delay = core::time::Duration::from_nanos(params.delay_period);
			let connection_end =
				ConnectionEnd::new(State::Init, client_id.clone(), counterparty, versions, delay);
			let mut ctx = routing::Context::<T>::new();
			ctx.store_connection(connection_id.clone(), &connection_end)
				.map_err(|_| Error::<T>::Other)?;
			ctx.store_connection_to_client(connection_id, &client_id)
				.map_err(|_| Error::<T>::Other)?;
			Self::deposit_event(Event::<T>::ConnectionInitiated);
			Ok(())
		}
	}
}
