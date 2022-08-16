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

//! IBC Transfer module for the runtime.
//! Implements Ibc transfer application
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod weight;
use composable_traits::defi::DeFiComposableConfig;
use frame_support::dispatch::Weight;
use primitives::currency::CurrencyId;
pub use weight::WeightInfo;

use codec::{Decode, Encode};
use core::{fmt::Formatter, marker::PhantomData};
use frame_system::ensure_signed;
use ibc::{
	applications::transfer::{
		acknowledgement::{Acknowledgement as Ics20Acknowledgement, ACK_ERR_STR},
		is_receiver_chain_source,
		packet::PacketData,
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
use ibc_primitives::{CallbackWeight, IbcTrait};
pub use pallet::*;
use scale_info::prelude::{
	format,
	string::{String, ToString},
};
use sp_std::{prelude::*, str::FromStr};

use sp_runtime::traits::IdentifyAccount;

#[frame_support::pallet]
pub mod pallet {
	use core::time::Duration;

	use super::*;
	use composable_traits::{
		currency::CurrencyFactory,
		xcm::assets::{RemoteAssetRegistryInspect, RemoteAssetRegistryMutate},
	};
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			EnsureOrigin, Get,
		},
		PalletId, Twox64Concat,
	};
	use frame_system::pallet_prelude::*;
	use ibc::{
		applications::transfer::{
			denom::is_sender_chain_source, msgs::transfer::MsgTransfer, Amount, PrefixedDenom,
		},
		core::ics04_channel::channel::{ChannelEnd, State},
		signer::Signer,
	};
	use ibc_primitives::{
		channel_id_from_bytes, connection_id_from_bytes, get_channel_escrow_address,
		port_id_from_bytes, runtime_interface, runtime_interface::SS58CodecError, IbcTrait,
		OpenChannelParams,
	};
	use primitives::currency::CurrencyId;
	use sp_runtime::AccountId32;
	use sp_std::collections::btree_set::BTreeSet;

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

	#[derive(
		frame_support::RuntimeDebug,
		PartialEq,
		Eq,
		scale_info::TypeInfo,
		Encode,
		Decode,
		Clone,
		Default,
	)]
	pub struct TransferParams {
		/// Valid utf8 string bytes
		pub to: Vec<u8>,
		/// Source channel on host chain
		pub source_channel: Vec<u8>,
		/// Timestamp at which this packet should timeout in counterparty in nanoseconds
		/// relative to the latest time stamp
		pub timeout_timestamp_offset: u64,
		/// Block height at which this packet should timeout on counterparty
		/// relative to the latest height
		pub timeout_height_offset: u64,
		/// Revision number, only needed when making a transfer to a parachain
		/// in which case this should be the para id
		pub revision_number: Option<u64>,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + balances::Config + composable_traits::defi::DeFiComposableConfig
	{
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// A type for creating local asset Ids
		type CurrencyFactory: CurrencyFactory<
			<Self as DeFiComposableConfig>::MayBeAssetId,
			<Self as DeFiComposableConfig>::Balance,
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
		type IbcHandler: ibc_primitives::IbcTrait;
		type AdminOrigin: EnsureOrigin<Self::Origin>;
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TokenTransferInitiated {
			from: <T as frame_system::Config>::AccountId,
			to: Vec<u8>,
			amount: <T as DeFiComposableConfig>::Balance,
		},
		/// A channel has been opened
		ChannelOpened { channel_id: Vec<u8>, port_id: Vec<u8> },
		/// Pallet params updated
		PalletParamsUpdated { send_enabled: bool, receive_enabled: bool },
		/// Token Transfer Completed
		IbcTransferCompleted {
			from: Vec<u8>,
			to: Vec<u8>,
			ibc_denom: Vec<u8>,
			local_asset_id: Option<<T as DeFiComposableConfig>::MayBeAssetId>,
			amount: <T as DeFiComposableConfig>::Balance,
		},
	}

	/// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
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
	}

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

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsic", which are often compared to transactions.
	// Dispatch able functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		CurrencyId: From<<T as DeFiComposableConfig>::MayBeAssetId>,
		AccountId32: From<T::AccountId>,
	{
		#[frame_support::transactional]
		#[pallet::weight(<T as Config>::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			params: TransferParams,
			asset_id: <T as DeFiComposableConfig>::MayBeAssetId,
			amount: <T as DeFiComposableConfig>::Balance,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			// Check if it's a local asset id, native asset or an ibc asset id
			// If native or local asset, get the string representation of the asset
			let denom = if let Some(denom) = IbcAssetIds::<T>::get(&asset_id) {
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
			let to = String::from_utf8(params.to).map_err(|_| Error::<T>::Utf8Error)?;
			let denom = PrefixedDenom::from_str(&denom).map_err(|_| Error::<T>::InvalidIbcDenom)?;
			let ibc_amount = Amount::from_str(&format!("{:?}", amount))
				.map_err(|_| Error::<T>::InvalidAmount)?;
			let coin = PrefixedCoin { denom, amount: ibc_amount };
			let source_channel = channel_id_from_bytes(params.source_channel.clone())
				.map_err(|_| Error::<T>::Utf8Error)?;
			let source_port = PortId::transfer();
			let (latest_height, latest_timestamp) =
				T::IbcHandler::latest_height_and_timestamp(&source_port, &source_channel)
					.map_err(|_| Error::<T>::TimestampAndHeightNotFound)?;
			let msg = MsgTransfer {
				source_port,
				source_channel,
				token: coin,
				sender: Signer::from_str(&from).map_err(|_| Error::<T>::Utf8Error)?,
				receiver: Signer::from_str(&to).map_err(|_| Error::<T>::Utf8Error)?,
				timeout_height: latest_height.add(params.timeout_height_offset),
				timeout_timestamp: (latest_timestamp +
					Duration::from_nanos(params.timeout_timestamp_offset))
				.map_err(|_| Error::<T>::InvalidTimestamp)?,
			};

			if is_sender_chain_source(msg.source_port.clone(), msg.source_channel, &msg.token.denom)
			{
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

			T::IbcHandler::send_transfer(msg).map_err(|e| {
				log::trace!(target: "ibc_transfer", "[transfer]: error: {:?}", e);
				Error::<T>::TransferFailed
			})?;

			Self::deposit_event(Event::<T>::TokenTransferInitiated {
				from: origin,
				to: to.as_bytes().to_vec(),
				amount,
			});
			Ok(())
		}

		#[frame_support::transactional]
		#[pallet::weight(<T as Config>::WeightInfo::open_channel())]
		pub fn open_channel(origin: OriginFor<T>, params: OpenChannelParams) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			let order: Order = (&params).try_into().map_err(|_| Error::<T>::InvalidParams)?;

			let connection_id = connection_id_from_bytes(params.connection_id)
				.map_err(|_| Error::<T>::InvalidParams)?;
			let counterparty_port_id = port_id_from_bytes(params.counterparty_port_id)
				.map_err(|_| Error::<T>::InvalidParams)?;
			let counterparty = Counterparty::new(counterparty_port_id, None);
			let channel_end = ChannelEnd::new(
				State::Init,
				order,
				counterparty,
				vec![connection_id],
				Version::new(VERSION.to_string()),
			);

			let port_id = PortId::transfer();
			let channel_id = T::IbcHandler::open_channel(port_id.clone(), channel_end)
				.map_err(|_| Error::<T>::ChannelInitError)?;
			Self::deposit_event(Event::<T>::ChannelOpened {
				channel_id: channel_id.to_string().as_bytes().to_vec(),
				port_id: port_id.as_bytes().to_vec(),
			});
			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::set_pallet_params())]
		pub fn set_pallet_params(origin: OriginFor<T>, params: PalletParams) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			<Params<T>>::put(params);
			Self::deposit_event(Event::<T>::PalletParamsUpdated {
				send_enabled: params.send_enabled,
				receive_enabled: params.receive_enabled,
			});
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn is_send_enabled() -> bool {
			Params::<T>::get().send_enabled
		}

		pub fn is_receive_enabled() -> bool {
			Params::<T>::get().receive_enabled
		}

		pub fn register_asset_id(
			asset_id: <T as DeFiComposableConfig>::MayBeAssetId,
			denom: Vec<u8>,
		) {
			IbcAssetIds::<T>::insert(asset_id, denom.clone());
			IbcDenoms::<T>::insert(denom, asset_id);
		}

		pub fn remove_channel_escrow_address(
			port_id: &PortId,
			channel_id: ChannelId,
		) -> Result<(), Ics04Error> {
			let escrow_address = get_channel_escrow_address(port_id, channel_id).map_err(|_| {
				Ics04Error::implementation_specific(
					"Failed to derive channel escrow address for removal".to_string(),
				)
			})?;
			let account_id = T::AccountIdConversion::try_from(escrow_address)
				.map_err(|_| {
					Ics04Error::implementation_specific(
						"Failed to derive channel escrow address for removal".to_string(),
					)
				})?
				.into_account();
			let _ = EscrowAddresses::<T>::try_mutate::<_, &'static str, _>(|addresses| {
				addresses.remove(&account_id);
				Ok(())
			});
			Ok(())
		}

		/// Returns true if address provided is an escrow address
		pub fn is_escrow_address(address: T::AccountId) -> bool {
			let set = EscrowAddresses::<T>::get();
			set.contains(&address)
		}
	}

	impl<T: Config> Pallet<T>
	where
		<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
		CurrencyId: From<<T as DeFiComposableConfig>::MayBeAssetId>,
	{
		pub fn get_denom_trace(asset_id: u128) -> Option<ibc_primitives::QueryDenomTraceResponse> {
			let asset_id: <T as DeFiComposableConfig>::MayBeAssetId = CurrencyId(asset_id).into();
			IbcAssetIds::<T>::get(asset_id)
				.map(|denom| ibc_primitives::QueryDenomTraceResponse { denom })
		}

		pub fn get_denom_traces(
			key: Option<u128>,
			offset: Option<u32>,
			mut limit: u64,
			count_total: bool,
		) -> ibc_primitives::QueryDenomTracesResponse {
			let mut iterator = if let Some(key) = key {
				let asset_id: <T as DeFiComposableConfig>::MayBeAssetId = CurrencyId(key).into();
				let raw_key = asset_id.encode();
				IbcAssetIds::<T>::iter_from(raw_key).skip(0)
			} else if let Some(offset) = offset {
				IbcAssetIds::<T>::iter().skip(offset as usize)
			} else {
				IbcAssetIds::<T>::iter().skip(0)
			};

			let mut denoms = vec![];
			for (_, denom) in iterator.by_ref() {
				denoms.push(denom);
				limit -= 1;
				if limit == 0 {
					break
				}
			}

			ibc_primitives::QueryDenomTracesResponse {
				denoms,
				total: count_total.then(|| IbcAssetIds::<T>::count() as u64),
				next_key: iterator.next().map(|(key, _)| {
					let asset_id: CurrencyId = key.into();
					asset_id.0
				}),
			}
		}
	}

	impl<T: Config> Pallet<T>
	where
		<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
	{
		pub fn ibc_denom_to_asset_id(
			full_denom: String,
			token: PrefixedCoin,
		) -> Option<<T as DeFiComposableConfig>::MayBeAssetId> {
			let is_local_asset = token.denom.trace_path().is_empty();
			if is_local_asset {
				if let Ok(asset_id) = CurrencyId::to_native_id(token.denom.base_denom().as_str()) {
					Some(asset_id.into())
				} else {
					let asset_id: CurrencyId =
						token.denom.base_denom().as_str().parse::<u128>().ok()?.into();
					Some(asset_id.into())
				}
			} else {
				IbcDenoms::<T>::get(full_denom.as_bytes().to_vec())
			}
		}
	}
}

#[derive(Clone)]
pub struct IbcCallbackHandler<T: Config>(PhantomData<T>);

impl<T: Config> core::fmt::Debug for IbcCallbackHandler<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "ibc-transfer")
	}
}

impl<T: Config> Default for IbcCallbackHandler<T> {
	fn default() -> Self {
		Self(PhantomData::default())
	}
}

impl<T: Config + Send + Sync> Module for IbcCallbackHandler<T>
where
	<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
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
		_order: Order,
		_connection_hops: &[ConnectionId],
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_counterparty: &Counterparty,
		_version: &Version,
		_counterparty_version: &Version,
	) -> Result<Version, Ics04Error> {
		Ok(Version::new(VERSION.to_string()))
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
		let ack = if T::IbcHandler::on_receive_packet(output, packet).is_err() {
			ACK_ERR_STR.to_string().as_bytes().to_vec()
		} else {
			let packet_data: PacketData = serde_json::from_slice(packet.data.as_slice())
				.expect("packet data should deserialize successfully");
			let denom = full_ibc_denom(packet, packet_data.token.clone());
			let prefixed_denom = PrefixedDenom::from_str(&denom).expect("Should not fail");
			let token = PrefixedCoin { denom: prefixed_denom, amount: packet_data.token.amount };
			Pallet::<T>::deposit_event(Event::<T>::IbcTransferCompleted {
				from: packet_data.sender.to_string().as_bytes().to_vec(),
				to: packet_data.receiver.to_string().as_bytes().to_vec(),
				ibc_denom: denom.as_bytes().to_vec(),
				local_asset_id: Pallet::<T>::ibc_denom_to_asset_id(denom, token),
				amount: packet_data.token.amount.as_u256().as_u128().into(),
			});
			Ics20Acknowledgement::success().as_ref().to_vec()
		};
		let packet = packet.clone();
		OnRecvPacketAck::Successful(
			Box::new(Ics20Acknowledgement::success()),
			Box::new(move |_ctx| {
				T::IbcHandler::write_acknowlegdement(&packet, ack)
					.map_err(|e| format!("[on_recv_packet] {:#?}", e))
			}),
		)
	}

	fn on_acknowledgement_packet(
		&mut self,
		output: &mut ModuleOutputBuilder,
		packet: &Packet,
		acknowledgement: &Acknowledgement,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let packet_data: PacketData =
			serde_json::from_slice(packet.data.as_slice()).map_err(|e| {
				Ics04Error::implementation_specific(format!("Failed to decode packet data {:?}", e))
			})?;
		T::IbcHandler::on_ack_packet(output, packet, acknowledgement)
			.map(|_| {
				Pallet::<T>::deposit_event(Event::<T>::IbcTransferCompleted {
					from: packet_data.sender.to_string().as_bytes().to_vec(),
					to: packet_data.receiver.to_string().as_bytes().to_vec(),
					ibc_denom: packet_data.token.denom.to_string().as_bytes().to_vec(),
					local_asset_id: Pallet::<T>::ibc_denom_to_asset_id(
						packet_data.token.denom.to_string(),
						packet_data.token.clone(),
					),
					amount: packet_data.token.amount.as_u256().as_u128().into(),
				})
			})
			.map_err(|e| {
				Ics04Error::app_module(format!(
					"[ibc-transfer]: Error processing acknowledgement {:#?}",
					e
				))
			})
	}

	fn on_timeout_packet(
		&mut self,
		output: &mut ModuleOutputBuilder,
		packet: &Packet,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		T::IbcHandler::on_timeout_packet(output, packet).map_err(|e| {
			Ics04Error::app_module(format!(
				"[ibc-transfer]: Error processing timeout packet {:#?}",
				e
			))
		})
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
