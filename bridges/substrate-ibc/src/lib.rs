#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unreachable_patterns)]
#![allow(clippy::type_complexity)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

//! # IBC Module
//!
//! This module implements the standard [IBC protocol](https://github.com/cosmos/ics).
//!
//! ## Overview
//!
//! The goal of this pallet is to allow the blockchains built on Substrate to gain the ability to
//! interact with other chains in a trustees way via IBC protocol
//!
//! This project is currently in an early stage and will eventually be submitted to upstream.
//!
//! The pallet implements the chain specific logic of [ICS spec](https://github.com/cosmos/ibc/tree/ee71d0640c23ec4e05e924f52f557b5e06c1d82f),  and is integrated with [ibc-rs](https://github.com/informalsystems/ibc-rs), which implements the generic cross-chain logic in [ICS spec](https://github.com/cosmos/ibc/tree/ee71d0640c23ec4e05e924f52f557b5e06c1d82f).
//!
//! The chain specific logic of the modules in ICS spec implemented::
//! * ics-002-client-semantics
//! * ics-003-connection-semantics
//! * ics-004-channel-and-packet-semantics
//! * ics-005-port-allocation
//! * ics-010-grandpa-client
//! * ics-018-relayer-algorithms
//! * ics-025-handler-interface
//! * ics-026-routing-module
//!
//! ### Terminology
//!
//! Please refer to [IBC Terminology](https://github.com/cosmos/ibc/blob/a983dd86815175969099d041906f6a14643e51ef/ibc/1_IBC_TERMINOLOGY.md).
//!
//! ### Goals
//!
//! This IBC module handles authentication, transport, and ordering of structured data packets
//! relayed between modules on separate machines.
//!
//! ## Interface
//!
//! ###  Public Functions
//!
//! * `deliver` - `ibc::ics26_routing::handler::deliver` Receives datagram transmitted from
//!   relayers/users, and pass to ICS26 router to look for the correct handler.
//!
//! ## Usage
//! Please refer to section "How to Interact with the Pallet" in the repository's README.md

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
pub use alloc::{format, string};

#[cfg(not(feature = "std"))]
use string::String;

use codec::{Decode, Encode};
use frame_system::ensure_signed;
pub use pallet::*;
use scale_info::{prelude::vec, TypeInfo};
use sp_runtime::RuntimeDebug;
use sp_std::{marker::PhantomData, prelude::*, str::FromStr};

use ibc::core::ics24_host::identifier::*;
mod channel;
mod client;
mod connection;
pub mod event;
mod port;
mod routing;

pub const IBC_DIGEST_ID: [u8; 4] = *b"IBC_";

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Any {
	pub type_url: Vec<u8>,
	pub value: Vec<u8>,
}

impl From<prost_types::Any> for Any {
	fn from(any: prost_types::Any) -> Self {
		Self { type_url: any.type_url.as_bytes().to_vec(), value: any.value }
	}
}

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::UnixTime};
	use frame_system::pallet_prelude::*;
	use ibc::{
		core::ics24_host::path::{
			AcksPath, ChannelEndsPath, ClientConnectionsPath, ClientConsensusStatePath,
			ClientStatePath, ClientTypePath, CommitmentsPath, ConnectionsPath, ReceiptsPath,
			SeqAcksPath, SeqRecvsPath, SeqSendsPath,
		},
		events::IbcEvent,
	};
	use sp_runtime::{generic::DigestItem, traits::BlakeTwo256};
	use sp_trie::{TrieDBMut, TrieMut};
	use tendermint_proto::Protobuf;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type TimeProvider: UnixTime;
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Prefix for events stored in the Off-chain DB via Indexing API.
		const INDEXING_PREFIX: &'static [u8];
		const CONNECTION_PREFIX: &'static [u8];
		#[pallet::constant]
		type ExpectedBlockTime: Get<u64>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	// client_id => ClientState
	pub type ClientStates<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	// client_id => (Height, ConsensusState)
	pub type ConsensusStates<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<(Vec<u8>, Vec<u8>)>, ValueQuery>;

	#[pallet::storage]
	// client_id , Height => Height
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
	// client_id , Height => Timestamp
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
	// connection_id => ConnectionEnd
	pub type Connections<T: Config> =
		CountedStorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	pub type ChannelCounter<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	// (port_identifier, channel_identifier) => ChannelEnd
	pub type Channels<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Blake2_128Concat,
		Vec<u8>,
		Vec<u8>,
		ValueQuery,
	>;

	// store_connection_channels
	#[pallet::storage]
	// connection_identifier => Vec<(port_id, channel_id)>
	pub type ChannelsConnection<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<(Vec<u8>, Vec<u8>)>, ValueQuery>;

	#[pallet::storage]
	// (port_identifier, channel_identifier) => Sequence
	pub type NextSequenceSend<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Blake2_128Concat,
		Vec<u8>,
		Vec<u8>,
		ValueQuery,
	>;

	#[pallet::storage]
	// (port_identifier, channel_identifier) => Sequence
	pub type NextSequenceRecv<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Blake2_128Concat,
		Vec<u8>,
		Vec<u8>,
		ValueQuery,
	>;

	#[pallet::storage]
	// (port_identifier, channel_identifier) = Sequence
	pub type NextSequenceAck<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Blake2_128Concat,
		Vec<u8>,
		Vec<u8>,
		ValueQuery,
	>;

	#[pallet::storage]
	// (port_identifier, channel_identifier, sequence) => Hash
	pub type Acknowledgements<T: Config> =
		StorageMap<_, Blake2_128Concat, (Vec<u8>, Vec<u8>, Vec<u8>), Vec<u8>, ValueQuery>;

	#[pallet::storage]
	// clientId => ClientType
	pub type Clients<T: Config> =
		CountedStorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	// client_id => Connection id
	pub type ConnectionClient<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	// (port_id, channel_id, sequence) => receipt
	pub type PacketReceipt<T: Config> =
		StorageMap<_, Blake2_128Concat, (Vec<u8>, Vec<u8>, Vec<u8>), Vec<u8>, ValueQuery>;

	#[pallet::storage]
	// (port_id, channel_id, sequence) => hash
	pub type PacketCommitment<T: Config> =
		StorageMap<_, Blake2_128Concat, (Vec<u8>, Vec<u8>, Vec<u8>), Vec<u8>, ValueQuery>;

	#[pallet::storage]
	// store latest height
	pub type LatestHeight<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	// store latest height
	pub type OldHeight<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T> {
		/// Processed incoming ibc messages
		ProcessedIBCMessages,
	}
	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error processing ibc messages
		ProcessingError,
		/// Error decoding some type
		DecodingError,
		/// Error inserting a value in trie
		TrieInsertError,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(_n: BlockNumberFor<T>) {
			let root = Pallet::<T>::build_ibc_state_root();
			if let Ok(root) = root {
				let log = DigestItem::Consensus(IBC_DIGEST_ID, root);
				<frame_system::Pallet<T>>::deposit_log(log);
			}
		}
	}

	// Dispatch able functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsic", which are often compared to transactions.
	// Dispatch able functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn deliver(origin: OriginFor<T>, messages: Vec<Any>, tmp: u8) -> DispatchResult {
			log::info!("in deliver");

			let _sender = ensure_signed(origin)?;
			let mut ctx = routing::Context { _pd: PhantomData::<T>, tmp };
			let messages = messages
				.iter()
				.map(|message| {
					let type_url = String::from_utf8(message.type_url.clone())
						.map_err(|_| Error::DecodingError)?;
					Ok(prost_types::Any { type_url, value: message.value.clone() })
				})
				.collect::<Result<Vec<prost_types::Any>, Error<T>>>()?;
			let result = ibc::core::ics26_routing::handler::deliver(&mut ctx, messages)
				.map_err(|_| Error::<T>::ProcessingError)?;

			log::info!("result: {:?}", result);

			Self::store_latest_height(result);
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn store_latest_height(ibc_events: Vec<IbcEvent>) -> () {
			let mut latest_height = ibc::Height::default();

			for ibc_event in ibc_events {
				match ibc_event {
					IbcEvent::Empty(_value) => {
						log::info!("ibc event: {}", "Empty");
					},
					IbcEvent::NewBlock(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::SendPacket(value) => {
						// store height
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::WriteAcknowledgement(value) => {
						// store height
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::UpdateClient(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::ReceivePacket(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::CloseConfirmChannel(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::CreateClient(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::UpgradeClient(value) => {
						let height = value.0.height.clone();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::ClientMisbehaviour(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::OpenInitConnection(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::OpenTryConnection(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::OpenAckConnection(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::OpenConfirmConnection(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::OpenInitChannel(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::OpenTryChannel(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::OpenAckChannel(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::OpenConfirmChannel(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::CloseInitChannel(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::AcknowledgePacket(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::TimeoutPacket(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::TimeoutOnClosePacket(value) => {
						let height = value.height();
						if height > latest_height {
							latest_height = height
						}
					},
					IbcEvent::ChainError(_value) => {
						log::info!("Ibc event: {}", "chainError");
					},
				}
			}
			if latest_height != ibc::Height::default() {
				<LatestHeight<T>>::set(latest_height.encode_vec().unwrap());
			}
		}

		fn offchain_key(packet_type: &str) -> Vec<u8> {
			let parent_blockhash = frame_system::Pallet::<T>::parent_hash();
			(T::INDEXING_PREFIX, packet_type, parent_blockhash).encode()
		}

		pub fn store_send_packet_offchain() {}

		pub fn store_timeout_packet_offchain() {}

		fn build_ibc_state_root() -> Result<Vec<u8>, Error<T>> {
			let mut db = sp_trie::MemoryDB::<BlakeTwo256>::default();

			let mut root = Default::default();

			let mut trie = <TrieDBMut<sp_trie::LayoutV0<BlakeTwo256>>>::new(&mut db, &mut root);
			let prefix = T::CONNECTION_PREFIX.to_vec();

			// Insert client and consensus states in trie
			for (client_id, client_state) in ClientStates::<T>::iter() {
				let consensus_states = ConsensusStates::<T>::get(&client_id);
				let client_connection = ConnectionClient::<T>::get(&client_id);
				let client_type = Clients::<T>::get(&client_id);

				let id = ClientId::from_str(
					&String::from_utf8(client_id).map_err(|_| Error::<T>::DecodingError)?,
				)
				.map_err(|_| Error::<T>::DecodingError)?;
				let client_state_path = format!("{}", ClientStatePath(id.clone()));
				let client_type_path = format!("{}", ClientTypePath(id.clone()));
				let mut client_type_key = prefix.clone();
				let mut client_state_key = prefix.clone();
				client_state_key.extend_from_slice(client_state_path.as_bytes());
				client_type_key.extend_from_slice(client_type_path.as_bytes());
				trie.insert(&client_state_key, &client_state)
					.map_err(|_| Error::<T>::TrieInsertError)?;
				trie.insert(&client_type_key, &client_type)
					.map_err(|_| Error::<T>::TrieInsertError)?;

				let client_connections_path = format!("{}", ClientConnectionsPath(id.clone()));
				let mut client_connections_path_key = prefix.clone();
				client_connections_path_key.extend_from_slice(client_connections_path.as_bytes());
				trie.insert(&client_connections_path_key, &client_connection)
					.map_err(|_| Error::<T>::TrieInsertError)?;
				for (height, consensus_state) in consensus_states {
					let height =
						ibc::Height::decode(&*height).map_err(|_| Error::<T>::DecodingError)?;
					let consensus_path = ClientConsensusStatePath {
						client_id: id.clone(),
						epoch: height.revision_number,
						height: height.revision_height,
					};
					let mut key = prefix.clone();
					let path = format!("{}", consensus_path);
					key.extend_from_slice(&path.as_bytes());
					trie.insert(&key, &consensus_state).map_err(|_| Error::<T>::TrieInsertError)?;
				}
			}

			// Insert connection ends in trie
			for (connection, connection_end) in Connections::<T>::iter() {
				let mut key = prefix.clone();
				let connection_id = ConnectionId::from_str(
					&String::from_utf8(connection).map_err(|_| Error::<T>::DecodingError)?,
				)
				.map_err(|_| Error::<T>::DecodingError)?;
				let path = format!("{}", ConnectionsPath(connection_id));
				key.extend_from_slice(path.as_bytes());
				trie.insert(&key, &connection_end).map_err(|_| Error::<T>::TrieInsertError)?;
			}

			// Insert channel ends and sequences in trie
			for (port, channel, channel_end) in Channels::<T>::iter() {
				let next_seq_send = NextSequenceSend::<T>::get(&port, &channel);
				let next_seq_recv = NextSequenceRecv::<T>::get(&port, &channel);
				let next_seq_ack = NextSequenceAck::<T>::get(&port, &channel);
				let mut channel_key = prefix.clone();
				let channel_id = channel_id_from_bytes::<T>(channel)?;
				let port_id = port_id_from_bytes::<T>(port)?;
				let channel_path =
					format!("{}", ChannelEndsPath(port_id.clone(), channel_id.clone()));
				let next_seq_send_path =
					format!("{}", SeqSendsPath(port_id.clone(), channel_id.clone()));
				let next_seq_recv_path =
					format!("{}", SeqRecvsPath(port_id.clone(), channel_id.clone()));
				let next_seq_ack_path =
					format!("{}", SeqAcksPath(port_id.clone(), channel_id.clone()));
				let mut next_seq_send_key = prefix.clone();
				let mut next_seq_recv_key = prefix.clone();
				let mut next_seq_ack_key = prefix.clone();
				next_seq_recv_key.extend_from_slice(next_seq_recv_path.as_bytes());
				next_seq_send_key.extend_from_slice(next_seq_send_path.as_bytes());
				next_seq_ack_key.extend_from_slice(next_seq_ack_path.as_bytes());

				channel_key.extend_from_slice(channel_path.as_bytes());
				trie.insert(&channel_key, &channel_end)
					.map_err(|_| Error::<T>::TrieInsertError)?;

				trie.insert(&next_seq_ack_key, &next_seq_ack)
					.map_err(|_| Error::<T>::TrieInsertError)?;
				trie.insert(&next_seq_send_key, &next_seq_send)
					.map_err(|_| Error::<T>::TrieInsertError)?;
				trie.insert(&next_seq_recv_key, &next_seq_recv)
					.map_err(|_| Error::<T>::TrieInsertError)?;
			}

			// Insert packet commitments in trie
			for ((port, channel, sequence), commitment) in PacketCommitment::<T>::iter() {
				let channel_id = channel_id_from_bytes::<T>(channel)?;
				let port_id = port_id_from_bytes::<T>(port)?;
				let sequence = ibc::core::ics04_channel::packet::Sequence::from(
					u64::decode(&mut &*sequence).map_err(|_| Error::<T>::DecodingError)?,
				);

				let commitment_path = CommitmentsPath { port_id, channel_id, sequence };

				let mut commitment_key = prefix.clone();
				let commitment_path_str = format!("{}", commitment_path);
				commitment_key.extend_from_slice(commitment_path_str.as_bytes());

				trie.insert(&commitment_key, &commitment)
					.map_err(|_| Error::<T>::TrieInsertError)?;
			}

			// Insert packet acknowledgements in trie
			for ((port, channel, sequence), ack) in Acknowledgements::<T>::iter() {
				let channel_id = channel_id_from_bytes::<T>(channel)?;
				let port_id = port_id_from_bytes::<T>(port)?;
				let sequence = ibc::core::ics04_channel::packet::Sequence::from(
					u64::decode(&mut &*sequence).map_err(|_| Error::<T>::DecodingError)?,
				);

				let ack_path = AcksPath { port_id, channel_id, sequence };

				let mut ack_key = prefix.clone();
				let ack_path_str = format!("{}", ack_path);
				ack_key.extend_from_slice(ack_path_str.as_bytes());

				trie.insert(&ack_key, &ack).map_err(|_| Error::<T>::TrieInsertError)?;
			}

			// Insert packet receipts in trie
			for ((port, channel, sequence), receipt) in PacketReceipt::<T>::iter() {
				let channel_id = channel_id_from_bytes::<T>(channel)?;
				let port_id = port_id_from_bytes::<T>(port)?;
				let sequence = ibc::core::ics04_channel::packet::Sequence::from(
					u64::decode(&mut &*sequence).map_err(|_| Error::<T>::DecodingError)?,
				);

				let receipt_path = ReceiptsPath { port_id, channel_id, sequence };

				let mut receipt_key = prefix.clone();
				let receipt_path_str = format!("{}", receipt_path);
				receipt_key.extend_from_slice(receipt_path_str.as_bytes());

				trie.insert(&receipt_key, &receipt).map_err(|_| Error::<T>::TrieInsertError)?;
			}
			Ok(trie.root().as_bytes().to_vec())
		}
	}
}

fn port_id_from_bytes<T: Config>(port: Vec<u8>) -> Result<PortId, Error<T>> {
	PortId::from_str(&String::from_utf8(port).map_err(|_| Error::<T>::DecodingError)?)
		.map_err(|_| Error::<T>::DecodingError)
}

fn channel_id_from_bytes<T: Config>(channel: Vec<u8>) -> Result<ChannelId, Error<T>> {
	ChannelId::from_str(&String::from_utf8(channel).map_err(|_| Error::<T>::DecodingError)?)
		.map_err(|_| Error::<T>::DecodingError)
}
