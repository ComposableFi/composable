#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unreachable_patterns)]
#![allow(clippy::type_complexity)]
#![allow(clippy::useless_format)]
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
//! Implements the ibc protocol for substrate runtimes.
extern crate alloc;

use codec::{Decode, Encode};
use cumulus_primitives_core::ParaId;
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
pub mod ics20;
mod ics23;
pub mod light_clients;
mod port;
pub mod routing;
pub use client::HostConsensusProof;

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

#[derive(
	frame_support::RuntimeDebug,
	PartialEq,
	Eq,
	scale_info::TypeInfo,
	Encode,
	Decode,
	Copy,
	Clone,
	Default,
	codec::MaxEncodedLen,
)]
pub struct PalletParams {
	pub send_enabled: bool,
	pub receive_enabled: bool,
}

/// Params needed to upgrade clients for all connected chains.
#[derive(
	frame_support::RuntimeDebug, PartialEq, Eq, scale_info::TypeInfo, Encode, Decode, Clone,
)]
pub struct UpgradeParams {
	/// Protobuf encoded client state
	pub client_state: Vec<u8>,
	/// Protobuf encoded consensus state
	pub consensus_state: Vec<u8>,
}

#[derive(
	frame_support::RuntimeDebug, PartialEq, Eq, scale_info::TypeInfo, Encode, Decode, Clone,
)]
pub enum MultiAddress<AccountId> {
	Id(AccountId),
	Raw(Vec<u8>),
}

#[derive(
	frame_support::RuntimeDebug, PartialEq, Eq, scale_info::TypeInfo, Encode, Decode, Clone,
)]
pub struct TransferParams<AccountId> {
	/// Account id or valid utf8 string bytes
	pub to: MultiAddress<AccountId>,
	/// Source channel identifier on host chain
	pub source_channel: u64,
	/// Timeout for this packet
	pub timeout: Timeout,
}

/// Packet timeout, could be an offset, or absolute value.
#[derive(
	frame_support::RuntimeDebug, PartialEq, Eq, scale_info::TypeInfo, Encode, Decode, Clone,
)]
pub enum Timeout {
	Offset {
		/// Timestamp at which this packet should timeout in counterparty in seconds
		/// relative to the latest time stamp
		timestamp: Option<u64>,
		/// Block height at which this packet should timeout on counterparty
		/// relative to the latest height
		height: Option<u64>,
	},
	/// Absolute value
	Absolute {
		/// Timestamp at which this packet should timeout on the counterparty in nanoseconds
		timestamp: Option<u64>,
		/// Block height at which this packet should timeout on the counterparty
		height: Option<u64>,
	},
}

pub enum LightClientProtocol {
	Beefy,
	Grandpa,
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
	use composable_traits::{
		currency::CurrencyFactory,
		defi::DeFiComposableConfig,
		xcm::assets::{RemoteAssetRegistryInspect, RemoteAssetRegistryMutate, XcmAssetLocation},
	};

	use core::time::Duration;

	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			Currency, UnixTime,
		},
	};
	use frame_system::pallet_prelude::*;
	pub use ibc::signer::Signer;

	use ibc::{
		applications::transfer::{
			is_sender_chain_source, msgs::transfer::MsgTransfer, Amount, PrefixedCoin,
			PrefixedDenom,
		},
		core::ics24_host::identifier::{ChannelId, PortId},
		timestamp::Timestamp,
		Height,
	};
	use ibc_primitives::{
		get_channel_escrow_address,
		runtime_interface::{self, SS58CodecError},
		IbcHandler, PacketInfo,
	};
	use primitives::currency::CurrencyId;
	use sp_runtime::{traits::IdentifyAccount, AccountId32};
	use sp_std::collections::btree_set::BTreeSet;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_ibc_ping::Config
		+ parachain_info::Config
		+ DeFiComposableConfig
		+ assets::Config
		+ core::fmt::Debug
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
		/// must be the same as the connection prefix
		const CHILD_TRIE_KEY: &'static [u8];
		/// Light client protocol this chain is operating
		const LIGHT_CLIENT_PROTOCOL: LightClientProtocol;
		/// A type for creating local asset Ids
		type CurrencyFactory: CurrencyFactory<
			AssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			Balance = <Self as DeFiComposableConfig>::Balance,
		>;
		/// Account Id Conversion from SS58 string or hex string
		type AccountIdConversion: TryFrom<Signer>
			+ IdentifyAccount<AccountId = Self::AccountId>
			+ Clone;
		/// A type that allows us map ibc assets to local assets
		type AssetRegistry: RemoteAssetRegistryMutate + RemoteAssetRegistryInspect;
		/// MultiCurrency System
		type MultiCurrency: Transfer<
				Self::AccountId,
				Balance = <Self as DeFiComposableConfig>::Balance,
				AssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			> + Mutate<
				Self::AccountId,
				Balance = <Self as DeFiComposableConfig>::Balance,
				AssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			> + Inspect<
				Self::AccountId,
				Balance = <Self as DeFiComposableConfig>::Balance,
				AssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			>;
		/// Expected blocktime in milliseconds
		#[pallet::constant]
		type ExpectedBlockTime: Get<u64>;
		/// ParaId of the runtime
		type ParaId: Get<ParaId>;
		/// Relay chain this runtime is attached to
		type RelayChain: Get<light_client_common::RelayChain>;
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
	/// client_id , Height => Height
	pub type ClientUpdateHeight<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Blake2_128Concat,
		Vec<u8>,
		Vec<u8>,
		OptionQuery,
	>;

	#[pallet::storage]
	/// client_id , Height => Timestamp
	pub type ClientUpdateTime<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, Vec<u8>, Blake2_128Concat, Vec<u8>, u64, OptionQuery>;

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

	// temporary until offchain indexing is fixed
	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// (ChannelId, PortId, Sequence) => Packet
	pub type SendPackets<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		(Vec<u8>, Vec<u8>),
		Blake2_128Concat,
		u64,
		PacketInfo,
		OptionQuery,
	>;

	// temporary
	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// (ChannelId, PortId, Sequence) => Packet
	pub type ReceivePackets<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		(Vec<u8>, Vec<u8>),
		Blake2_128Concat,
		u64,
		PacketInfo,
		OptionQuery,
	>;

	// temporary
	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// (ChannelId, PortId, Sequence) => Vec<u8>
	pub type WriteAcknowledgements<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		(Vec<u8>, Vec<u8>),
		Blake2_128Concat,
		u64,
		Vec<u8>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// Pallet Params used to disable sending or receipt of ibc tokens
	pub type Params<T: Config> = StorageValue<_, PalletParams, ValueQuery>;

	#[pallet::storage]
	/// Map of asset id to ibc denom pairs (T::AssetId, Vec<u8>)
	/// ibc denoms represented as utf8 string bytes
	pub type IbcAssetIds<T: Config> = CountedStorageMap<
		_,
		Twox64Concat,
		<T as DeFiComposableConfig>::MayBeAssetId,
		Vec<u8>,
		OptionQuery,
	>;

	#[pallet::storage]
	/// Map of asset id to ibc denom pairs (Vec<u8>, T::AssetId)
	/// ibc denoms represented as utf8 string bytes
	pub type IbcDenoms<T: Config> = CountedStorageMap<
		_,
		Twox64Concat,
		Vec<u8>,
		<T as DeFiComposableConfig>::MayBeAssetId,
		OptionQuery,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// ChannelIds open from this module
	pub type ChannelIds<T: Config> = StorageValue<_, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// Active Escrow addresses
	pub type EscrowAddresses<T: Config> = StorageValue<_, BTreeSet<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Events emitted by the ibc subsystem
		Events { events: Vec<events::IbcEvent> },
		/// Errors emitted by the ibc subsystem
		Errors { errors: Vec<errors::IbcError> },
		/// An Ibc token transfer has been started
		TokenTransferInitiated {
			from: <T as frame_system::Config>::AccountId,
			to: Vec<u8>,
			ibc_denom: Vec<u8>,
			local_asset_id: Option<<T as DeFiComposableConfig>::MayBeAssetId>,
			amount: <T as DeFiComposableConfig>::Balance,
			is_sender_source: bool,
		},
		/// A channel has been opened
		ChannelOpened { channel_id: Vec<u8>, port_id: Vec<u8> },
		/// Pallet params updated
		ParamsUpdated { send_enabled: bool, receive_enabled: bool },
		/// An outgoing Ibc token transfer has been completed and burnt
		TokenTransferCompleted {
			from: Vec<u8>,
			to: Vec<u8>,
			ibc_denom: Vec<u8>,
			local_asset_id: Option<<T as DeFiComposableConfig>::MayBeAssetId>,
			amount: <T as DeFiComposableConfig>::Balance,
			is_sender_source: bool,
		},
		/// Ibc tokens have been received and minted
		TokenReceived {
			from: Vec<u8>,
			to: Vec<u8>,
			ibc_denom: Vec<u8>,
			local_asset_id: Option<<T as DeFiComposableConfig>::MayBeAssetId>,
			amount: <T as DeFiComposableConfig>::Balance,
			is_receiver_source: bool,
		},
		/// Ibc transfer failed, received an acknowledgement error, tokens have been refunded
		TokenTransferFailed {
			from: Vec<u8>,
			to: Vec<u8>,
			ibc_denom: Vec<u8>,
			local_asset_id: Option<<T as DeFiComposableConfig>::MayBeAssetId>,
			amount: <T as DeFiComposableConfig>::Balance,
			is_sender_source: bool,
		},
		/// On recv packet was not processed successfully processes
		OnRecvPacketError { msg: Vec<u8> },
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
		/// Invalid channel id
		InvalidChannelId,
		/// Invalid port id
		InvalidPortId,
		/// Other forms of errors
		Other,
		/// Invalid route
		InvalidRoute,
		/// Invalid message for extrinsic
		InvalidMessageType,
		/// The interchain token transfer was not successfully initiated
		TransferFailed,
		/// Error Decoding utf8 bytes
		Utf8Error,
		/// Invalid asset id
		InvalidAssetId,
		/// Invalid Ibc denom
		InvalidIbcDenom,
		/// Invalid amount
		InvalidAmount,
		/// Invalid timestamp
		InvalidTimestamp,
		/// Unable to get client revision number
		FailedToGetRevisionNumber,
		/// Invalid params passed
		InvalidParams,
		/// Error opening channel
		ChannelInitError,
		/// Latest height and timestamp for a client not found
		TimestampAndHeightNotFound,
		/// Failed to derive channel escrow address
		ChannelEscrowAddress,
		/// Error writing acknowledgement to storage
		WriteAckError,
		/// Client update time and height not found
		ClientUpdateNotFound,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		u32: From<<T as frame_system::Config>::BlockNumber>,
		<T as DeFiComposableConfig>::MayBeAssetId: From<primitives::currency::CurrencyId>,
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
		T: Send + Sync,
		CurrencyId: From<<T as DeFiComposableConfig>::MayBeAssetId>,
		AccountId32: From<T::AccountId>,
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
		<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetNativeLocation:
			From<XcmAssetLocation>,
		<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetNativeLocation:
			From<XcmAssetLocation>,
		<T as DeFiComposableConfig>::MayBeAssetId: From<<T as assets::Config>::AssetId>,
	{
		#[pallet::weight(crate::weight::deliver::< T > (messages))]
		#[frame_support::transactional]
		pub fn deliver(origin: OriginFor<T>, messages: Vec<Any>) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			// todo: reserve a fixed deposit for every client and connection created
			// so people don't spam our chain with useless clients.
			let mut ctx = routing::Context::<T>::new();
			let messages = messages
				.into_iter()
				.filter_map(|message| {
					let type_url = String::from_utf8(message.type_url.clone()).ok()?;
					Some(Ok(ibc_proto::google::protobuf::Any { type_url, value: message.value }))
				})
				.collect::<Result<Vec<ibc_proto::google::protobuf::Any>, Error<T>>>()?;
			Self::execute_ibc_messages(&mut ctx, messages);

			Ok(())
		}

		#[frame_support::transactional]
		#[pallet::weight(<T as Config>::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			params: TransferParams<T::AccountId>,
			asset_id: <T as DeFiComposableConfig>::MayBeAssetId,
			amount: <T as DeFiComposableConfig>::Balance,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			// Check if it's a local asset id, native asset or an ibc asset id
			// If native or local asset, get the string representation of the asset
			let denom = if let Some(denom) = IbcAssetIds::<T>::get(asset_id) {
				String::from_utf8(denom).map_err(|_| Error::<T>::Utf8Error)?
			} else {
				let asset_id: CurrencyId = asset_id.into();
				CurrencyId::native_asset_name(asset_id.0)
					.map(|val| val.to_string())
					.unwrap_or_else(|_| asset_id.to_string())
			};

			let account_id_32: AccountId32 = origin.clone().into();
			let from = runtime_interface::account_id_to_ss58(account_id_32.into())
				.and_then(|val| {
					String::from_utf8(val).map_err(|_| SS58CodecError::InvalidAccountId)
				})
				.map_err(|_| Error::<T>::Utf8Error)?;
			let to = match params.to {
				MultiAddress::Id(id) => {
					let account_id_32: AccountId32 = id.into();
					runtime_interface::account_id_to_ss58(account_id_32.into())
						.and_then(|val| {
							String::from_utf8(val).map_err(|_| SS58CodecError::InvalidAccountId)
						})
						.map_err(|_| Error::<T>::Utf8Error)?
				},
				MultiAddress::Raw(bytes) =>
					String::from_utf8(bytes).map_err(|_| Error::<T>::Utf8Error)?,
			};
			let denom = PrefixedDenom::from_str(&denom).map_err(|_| Error::<T>::InvalidIbcDenom)?;
			let ibc_amount = Amount::from_str(&format!("{:?}", amount))
				.map_err(|_| Error::<T>::InvalidAmount)?;
			let coin = PrefixedCoin { denom, amount: ibc_amount };
			let source_channel = ChannelId::new(params.source_channel);
			let source_port = PortId::transfer();
			let (latest_height, latest_timestamp) =
				Pallet::<T>::latest_height_and_timestamp(&source_port, &source_channel)
					.map_err(|_| Error::<T>::TimestampAndHeightNotFound)?;

			let (timeout_height, timeout_timestamp) = match params.timeout {
				Timeout::Offset { timestamp, height } => {
					let timestamp = timestamp
						.map(|offset| (latest_timestamp + Duration::from_secs(offset)))
						.transpose()
						.map_err(|_| Error::<T>::InvalidTimestamp)?
						.unwrap_or_default();
					let height = height.map(|offset| latest_height.add(offset)).unwrap_or_default();
					(height, timestamp)
				},
				Timeout::Absolute { timestamp, height } => {
					let timestamp = timestamp
						.map(Timestamp::from_nanoseconds)
						.transpose()
						.map_err(|_| Error::<T>::InvalidTimestamp)?
						.unwrap_or_default();
					let height = height
						.map(|revision_height| {
							Height::new(latest_height.revision_number, revision_height)
						})
						.unwrap_or_default();
					(height, timestamp)
				},
			};

			let msg = MsgTransfer {
				source_port,
				source_channel,
				token: coin.clone(),
				sender: Signer::from_str(&from).map_err(|_| Error::<T>::Utf8Error)?,
				receiver: Signer::from_str(&to).map_err(|_| Error::<T>::Utf8Error)?,
				timeout_height,
				timeout_timestamp,
			};

			let is_sender_source = is_sender_chain_source(
				msg.source_port.clone(),
				msg.source_channel,
				&msg.token.denom,
			);
			if is_sender_source {
				// Store escrow address, so we can use this to identify accounts to keep alive when
				// making transfers in callbacks Escrow addresses do not need to be kept alive
				let escrow_address =
					get_channel_escrow_address(&msg.source_port, msg.source_channel)
						.map_err(|_| Error::<T>::ChannelEscrowAddress)?;
				let account_id = T::AccountIdConversion::try_from(escrow_address)
					.map_err(|_| Error::<T>::ChannelEscrowAddress)?
					.into_account();
				let _ = EscrowAddresses::<T>::try_mutate::<_, &'static str, _>(|addresses| {
					if !addresses.contains(&account_id) {
						addresses.insert(account_id);
						Ok(())
					} else {
						Err("Address already exists")
					}
				});
			}

			Pallet::<T>::send_transfer(msg).map_err(|e| {
				log::trace!(target: "pallet_ibc", "[transfer]: error: {:?}", e);
				Error::<T>::TransferFailed
			})?;

			Self::deposit_event(Event::<T>::TokenTransferInitiated {
				from: origin,
				to: to.as_bytes().to_vec(),
				amount,
				local_asset_id: Pallet::<T>::ibc_denom_to_asset_id(
					coin.denom.to_string(),
					coin.clone(),
				),
				ibc_denom: coin.denom.to_string().as_bytes().to_vec(),
				is_sender_source,
			});
			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::set_params())]
		pub fn set_params(origin: OriginFor<T>, params: PalletParams) -> DispatchResult {
			<T as Config>::AdminOrigin::ensure_origin(origin)?;
			<Params<T>>::put(params);
			Self::deposit_event(Event::<T>::ParamsUpdated {
				send_enabled: params.send_enabled,
				receive_enabled: params.receive_enabled,
			});
			Ok(())
		}

		/// We write the consensus & client state under these predefined paths so that
		/// we can produce state proofs of the values to connected chains
		/// in order to execute client upgrades.
		#[pallet::weight(0)]
		pub fn upgrade_client(origin: OriginFor<T>, params: UpgradeParams) -> DispatchResult {
			<T as Config>::AdminOrigin::ensure_origin(origin)?;
			const CLIENT_STATE_UPGRADE_PATH: &[u8] = b"client-state-upgrade-path";
			const CONSENSUS_STATE_UPGRADE_PATH: &[u8] = b"consensus-state-upgrade-path";

			sp_io::storage::set(CLIENT_STATE_UPGRADE_PATH, &params.client_state);
			sp_io::storage::set(CONSENSUS_STATE_UPGRADE_PATH, &params.consensus_state);

			// todo: emit ibc.Event

			Ok(())
		}
	}
}
