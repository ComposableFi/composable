//! Benchmarking setup for pallet-template

#[allow(unused)]
use super::super::*;
use crate::{
	benchmarks::tendermint_benchmark_utils::*,
	ics20::IbcModule,
	ics23::client_states::ClientStates,
	light_clients::{AnyClientState, AnyConsensusState},
	Any, Config,
};
use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	defi::DeFiComposableConfig,
	xcm::assets::{RemoteAssetRegistryInspect, RemoteAssetRegistryMutate, XcmAssetLocation},
};
use core::str::FromStr;
use frame_benchmarking::{benchmarks, whitelisted_caller, Zero};
use frame_support::traits::fungibles::{Inspect, Mutate};
use frame_system::RawOrigin;
use ibc_primitives::IbcHandler;
use sp_runtime::traits::IdentifyAccount;

use crate::routing::Context;
use ibc::{
	applications::transfer::{
		acknowledgement::ACK_ERR_STR, packet::PacketData, Amount, Coin, PrefixedDenom, VERSION,
	},
	core::{
		ics02_client::{
			client_state::ClientState,
			context::ClientKeeper,
			height::Height,
			msgs::{
				create_client::{MsgCreateAnyClient, TYPE_URL},
				update_client::TYPE_URL as UPDATE_CLIENT_TYPE_URL,
			},
		},
		ics03_connection::{
			connection::{ConnectionEnd, Counterparty, State},
			context::{ConnectionKeeper, ConnectionReader},
			msgs::{
				conn_open_ack::TYPE_URL as CONN_OPEN_ACK_TYPE_URL,
				conn_open_confirm::TYPE_URL as CONN_OPEN_CONFIRM_TYPE_URL,
				conn_open_init as conn_open_init_mod,
				conn_open_try::TYPE_URL as CONN_TRY_OPEN_TYPE_URL,
			},
			version::Version as ConnVersion,
		},
		ics04_channel::{
			channel::{self, ChannelEnd, Order, State as ChannelState},
			context::{ChannelKeeper, ChannelReader},
			error::Error as Ics04Error,
			msgs::{
				acknowledgement::{Acknowledgement, TYPE_URL as ACK_PACKET_TYPE_URL},
				chan_close_confirm::TYPE_URL as CHAN_CLOSE_CONFIRM_TYPE_URL,
				chan_close_init::TYPE_URL as CHAN_CLOSE_INIT_TYPE_URL,
				chan_open_ack::TYPE_URL as CHAN_OPEN_ACK_TYPE_URL,
				chan_open_confirm::TYPE_URL as CHAN_OPEN_CONFIRM_TYPE_URL,
				chan_open_init::{MsgChannelOpenInit, TYPE_URL as CHAN_OPEN_TYPE_URL},
				chan_open_try::TYPE_URL as CHAN_OPEN_TRY_TYPE_URL,
				recv_packet::TYPE_URL as RECV_PACKET_TYPE_URL,
				timeout::TYPE_URL as TIMEOUT_TYPE_URL,
			},
			packet::{Packet, Receipt},
			Version,
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
		ics26_routing::context::{AsAnyMut, Module, OnRecvPacketAck},
	},
	handler::HandlerOutputBuilder,
	signer::Signer,
	timestamp::Timestamp,
};
use ibc_primitives::{get_channel_escrow_address, ibc_denom_to_foreign_asset_id};
use primitives::currency::CurrencyId;
use scale_info::prelude::string::ToString;
use sp_core::crypto::AccountId32;
use sp_std::vec;
use tendermint_proto::Protobuf;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

const TIMESTAMP: u64 = 1650894363;

benchmarks! {
	where_clause {
		where u32: From<<T as frame_system::Config>::BlockNumber>,
				<T as frame_system::Config>::BlockNumber: From<u32>,
				T: Send + Sync + pallet_timestamp::Config<Moment = u64> + parachain_info::Config + Config,
		CurrencyId: From<<T as DeFiComposableConfig>::MayBeAssetId>,
		AccountId32: From<T::AccountId>,
		<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
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
	}

	// Run these benchmarks via
	// ```bash
	// cargo +nightly test -p pallet-ibc  --features=runtime-benchmarks
	// ```
	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);

	// update_client
	update_tendermint_client {
		let mut ctx = routing::Context::<T>::new();
		// Set timestamp to the same timestamp used in generating tendermint header, because there
		// will be a comparison between the local timestamp and the timestamp existing in the header
		// after factoring in the trusting period for the light client.
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let value = create_client_update::<T>().encode_vec();

		let msg = Any { type_url: UPDATE_CLIENT_TYPE_URL.to_string().as_bytes().to_vec(), value };
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let client_state = ClientStates::<T>::get(&client_id).unwrap();
		let client_state = AnyClientState::decode_vec(&*client_state).unwrap();
		assert_eq!(client_state.latest_height(), Height::new(0, 2));
	}

	// connection open try
	conn_try_open_tendermint {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		// Create initial client state and consensus state
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();


		// We update the light client state so it can have the required client and consensus states required to process
		// the proofs that will be submitted
		let value = create_client_update::<T>().encode_vec();
		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };
		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let (cs_state, value) = create_conn_open_try::<T>();
		// Update consensus state with the new root that we'll enable proofs to be correctly verified
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		let msg = Any { type_url: CONN_TRY_OPEN_TYPE_URL.as_bytes().to_vec(), value: value.encode_vec() };
		log::trace!(target: "pallet_ibc", "\n\n\n\n\n\n<=============== Begin benchmark ====================>\n\n\n\n\n");
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let connection_end = ConnectionReader::connection_end(&ctx, &ConnectionId::new(0)).unwrap();
		assert_eq!(connection_end.state, State::TryOpen);
	}

	// connection open ack
	conn_open_ack_tendermint {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();
		// Create a connection end and put in storage
		// Successful processing of a connection open confirm message requires a compatible connection end with state INIT or TRYOPEN
		// to exist on the local chain
		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = <T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Init, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();

		let value = create_client_update::<T>().encode_vec();
		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };
		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let (cs_state, value) = create_conn_open_ack::<T>();
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		let msg = Any { type_url: CONN_OPEN_ACK_TYPE_URL.as_bytes().to_vec(), value: value.encode_vec() };
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let connection_end = ConnectionReader::connection_end(&ctx, &ConnectionId::new(0)).unwrap();
		assert_eq!(connection_end.state, State::Open);
	}

	// connection open confirm
	conn_open_confirm_tendermint {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		// Create a connection end and put in storage
		// Successful processing of a connection open confirm message requires a compatible connection end with state TryOpen
		// to exist on the local chain
		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = <T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::TryOpen, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();

		// We update the light client state so it can have the required client and consensus states required to process
		// the proofs that will be submitted
		let value = create_client_update::<T>().encode_vec();
		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };
		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let (cs_state, value) = create_conn_open_confirm::<T>();
		// Update consensus state with the new root that we'll enable proofs to be correctly verified
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		let msg = Any { type_url: CONN_OPEN_CONFIRM_TYPE_URL.as_bytes().to_vec(), value: value.encode_vec() };
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let connection_end = ConnectionReader::connection_end(&ctx, &ConnectionId::new(0)).unwrap();
		assert_eq!(connection_end.state, State::Open);
	}

	// For all channel messages to be processed successfully, a connection end must exist and be in the OPEN state
	// create channel
	channel_open_init {
		let mut ctx = routing::Context::<T>::new();
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = <T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
		let counterparty_channel = ibc::core::ics04_channel::channel::Counterparty::new(port_id.clone(), None);
		let channel_end = ChannelEnd::new(
			ibc::core::ics04_channel::channel::State::Init,
			ibc::core::ics04_channel::channel::Order::Unordered,
			counterparty_channel,
			vec![ConnectionId::new(0)],
			ibc::core::ics04_channel::Version::default()
		);

		let value = MsgChannelOpenInit {
			port_id: port_id.clone(),
			channel: channel_end,
			signer: Signer::from_str(MODULE_ID).unwrap()
		}.encode_vec();

		let caller: T::AccountId = whitelisted_caller();
		let msg = Any { type_url: CHAN_OPEN_TYPE_URL.as_bytes().to_vec(), value };
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		assert_eq!(ChannelCounter::<T>::get(), 1);
	}

	// channel_open_try
	channel_open_try_tendermint {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = <T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		// We update the light client state so it can have the required client and consensus states required to process
		// the proofs that will be submitted
		let value = create_client_update::<T>().encode_vec();
		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();

		let counterparty_channel = ibc::core::ics04_channel::channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(0)));

		let (cs_state, value) = create_chan_open_try();
		// Update consensus root for light client
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let msg = Any {
			type_url: CHAN_OPEN_TRY_TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec()
		};
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let channel_end = ctx.channel_end(&(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), ChannelId::new(0))).unwrap();
		assert_eq!(channel_end.state, ChannelState::TryOpen);
	}

	// channel_open_ack
	channel_open_ack_tendermint {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = <T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update::<T>().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();

		let counterparty_channel = ibc::core::ics04_channel::channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(0)));
		let channel_end = ChannelEnd::new(
			ibc::core::ics04_channel::channel::State::Init,
			ibc::core::ics04_channel::channel::Order::Unordered,
			counterparty_channel,
			vec![ConnectionId::new(0)],
			ibc::core::ics04_channel::Version::default()
		);

		let value = MsgChannelOpenInit {
			port_id,
			channel: channel_end,
			signer: Signer::from_str(MODULE_ID).unwrap()
		}.encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: CHAN_OPEN_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let (cs_state, value) = create_chan_open_ack();
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let msg = Any {
			type_url: CHAN_OPEN_ACK_TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec()
		};
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let channel_end = ctx.channel_end(&(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), ChannelId::new(0))).unwrap();
		assert_eq!(channel_end.state, ChannelState::Open);
	}

	// channel_open_confirm
	channel_open_confirm_tendermint {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = <T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update::<T>().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();

		let counterparty_channel = ibc::core::ics04_channel::channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(0)));
		let channel_end = ChannelEnd::new(
			ibc::core::ics04_channel::channel::State::TryOpen,
			ibc::core::ics04_channel::channel::Order::Unordered,
			counterparty_channel,
			vec![ConnectionId::new(0)],
			ibc::core::ics04_channel::Version::default()
		);

		ctx.store_channel((port_id.clone(), ChannelId::new(0)), &channel_end).unwrap();
		ctx.store_connection_channels(ConnectionId::new(0), &(port_id.clone(), ChannelId::new(0))).unwrap();

		let (cs_state, value) = create_chan_open_confirm();
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let msg = Any {
			type_url: CHAN_OPEN_CONFIRM_TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec()
		};
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let channel_end = ctx.channel_end(&(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), ChannelId::new(0))).unwrap();
		assert_eq!(channel_end.state, ChannelState::Open);
	}

	// channel_close_init
	channel_close_init {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = <T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update::<T>().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();

		let counterparty_channel = ibc::core::ics04_channel::channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(0)));
		let channel_end = ChannelEnd::new(
			ibc::core::ics04_channel::channel::State::Open,
			ibc::core::ics04_channel::channel::Order::Unordered,
			counterparty_channel,
			vec![ConnectionId::new(0)],
			ibc::core::ics04_channel::Version::default()
		);

		ctx.store_channel((port_id.clone(), ChannelId::new(0)), &channel_end).unwrap();
		ctx.store_connection_channels(ConnectionId::new(0), &(port_id.clone(), ChannelId::new(0))).unwrap();

		let value = create_chan_close_init();
		let msg = Any {
			type_url: CHAN_CLOSE_INIT_TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec()
		};
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let channel_end = ctx.channel_end(&(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), ChannelId::new(0))).unwrap();
		assert_eq!(channel_end.state, ChannelState::Closed);
	}

	// channel_close_confirm
	channel_close_confirm_tendermint {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = <T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update::<T>().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();

		let counterparty_channel = ibc::core::ics04_channel::channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(0)));
		let channel_end = ChannelEnd::new(
			ibc::core::ics04_channel::channel::State::Open,
			ibc::core::ics04_channel::channel::Order::Unordered,
			counterparty_channel,
			vec![ConnectionId::new(0)],
			ibc::core::ics04_channel::Version::default()
		);

		ctx.store_channel((port_id.clone(), ChannelId::new(0)), &channel_end).unwrap();
		ctx.store_connection_channels(ConnectionId::new(0), &(port_id.clone(), ChannelId::new(0))).unwrap();

		let (cs_state, value) = create_chan_close_confirm();
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let msg = Any {
			type_url: CHAN_CLOSE_CONFIRM_TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec()
		};
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let channel_end = ctx.channel_end(&(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), ChannelId::new(0))).unwrap();
		assert_eq!(channel_end.state, ChannelState::Closed);
	}


	// recv_packet
	recv_packet_tendermint {
		let i in 1..1000u32;
		let data = vec![0u8;i.try_into().unwrap()];
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		frame_system::Pallet::<T>::set_block_number(2u32.into());
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = <T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(0);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update::<T>().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();
		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
		let counterparty_channel = ibc::core::ics04_channel::channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(0)));
		let channel_end = ChannelEnd::new(
			ibc::core::ics04_channel::channel::State::Open,
			ibc::core::ics04_channel::channel::Order::Unordered,
			counterparty_channel,
			vec![ConnectionId::new(0)],
			ibc::core::ics04_channel::Version::default()
		);

		ctx.store_channel((port_id.clone(), ChannelId::new(0)), &channel_end).unwrap();
		ctx.store_connection_channels(ConnectionId::new(0), &(port_id.clone(), ChannelId::new(0))).unwrap();
		ctx.store_next_sequence_recv((port_id.clone(), ChannelId::new(0)), 1u64.into()).unwrap();

		let (cs_state, value) = create_recv_packet::<T>(data);
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let msg = Any {
			type_url: RECV_PACKET_TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec()
		};
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let receipt = ctx.get_packet_receipt(&(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), ChannelId::new(0), 1u64.into())).unwrap();
		match receipt {
			Receipt::Ok => {},
			_ => panic!("Commitment should not exist")
		}
	}

	// ack_packet
	ack_packet_tendermint {
		let i in 1..1000u32;
		let j in 1..1000u32;
		let data = vec![0u8;i.try_into().unwrap()];
		let ack = vec![0u8;j.try_into().unwrap()];
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		frame_system::Pallet::<T>::set_block_number(2u32.into());
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = <T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(0);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update::<T>().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
		let counterparty_channel = ibc::core::ics04_channel::channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(0)));
		let channel_end = ChannelEnd::new(
			ibc::core::ics04_channel::channel::State::Open,
			ibc::core::ics04_channel::channel::Order::Unordered,
			counterparty_channel,
			vec![ConnectionId::new(0)],
			ibc::core::ics04_channel::Version::default()
		);

		ctx.store_channel((port_id.clone(), ChannelId::new(0)), &channel_end).unwrap();
		ctx.store_connection_channels(ConnectionId::new(0), &(port_id.clone(), ChannelId::new(0))).unwrap();
		ctx.store_next_sequence_recv((port_id.clone(), ChannelId::new(0)), 1u64.into()).unwrap();

		let (cs_state, value) = create_ack_packet::<T>(data, ack);
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let msg = Any {
			type_url: ACK_PACKET_TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec()
		};
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let res = ctx.get_packet_commitment(&(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), ChannelId::new(0), 1u64.into()));
		match res {
			Ok(_) => panic!("Commitment should not exist"),
			Err(e) => assert_eq!(e.detail(), Ics04Error::packet_commitment_not_found(1u64.into()).detail())
		}
	}

	timeout_packet_tendermint {
		let i in 1..1000u32;
		let data = vec![0u8;i.try_into().unwrap()];
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		frame_system::Pallet::<T>::set_block_number(2u32.into());
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = <T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(0);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update::<T>().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
		let counterparty_channel = ibc::core::ics04_channel::channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(0)));
		let channel_end = ChannelEnd::new(
			ibc::core::ics04_channel::channel::State::Open,
			ibc::core::ics04_channel::channel::Order::Ordered,
			counterparty_channel,
			vec![ConnectionId::new(0)],
			ibc::core::ics04_channel::Version::default()
		);

		ctx.store_channel((port_id.clone(), ChannelId::new(0)), &channel_end).unwrap();
		ctx.store_connection_channels(ConnectionId::new(0), &(port_id.clone(), ChannelId::new(0))).unwrap();
		ctx.store_next_sequence_recv((port_id.clone(), ChannelId::new(0)), 1u64.into()).unwrap();
		ctx.store_next_sequence_send((port_id.clone(), ChannelId::new(0)), 1u64.into()).unwrap();

		let (cs_state, value) = create_timeout_packet::<T>(data);
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let msg = Any {
			type_url: TIMEOUT_TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec()
		};
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let res = ctx.get_packet_commitment(&(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), ChannelId::new(0), 1u64.into()));
		let channel_end = ctx.channel_end(&(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), ChannelId::new(0))).unwrap();
		assert_eq!(channel_end.state, ChannelState::Closed);
		match res {
			Ok(_) => panic!("Commitment should not exist"),
			Err(e) => assert_eq!(e.detail(), Ics04Error::packet_commitment_not_found(1u64.into()).detail())
		}
	}


	conn_open_init {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();
		let commitment_prefix: CommitmentPrefix = <T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let value = conn_open_init_mod::MsgConnectionOpenInit {
			client_id: client_id.clone(),
			counterparty: Counterparty::new(
				counterparty_client_id.clone(),
				Some(ConnectionId::new(1)),
				commitment_prefix.clone(),
			),
			version: Some(ConnVersion::default()),
			delay_period: core::time::Duration::from_nanos(1000),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		};

		let msg = Any {
			type_url: conn_open_init_mod::TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec()
		};
		let caller: T::AccountId = whitelisted_caller();

	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let connection_end = ConnectionReader::connection_end(&ctx, &ConnectionId::new(0)).unwrap();
		assert_eq!(connection_end.state, State::Init);
	}

	create_client {
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let msg = MsgCreateAnyClient::<Context<T>>::new(
			AnyClientState::Tendermint(mock_client_state),
			AnyConsensusState::Tendermint(mock_cs_state),
			Signer::from_str(MODULE_ID).unwrap(),
		)
		.unwrap()
		.encode_vec();

		let msg = Any { type_url: TYPE_URL.to_string().as_bytes().to_vec(), value: msg };
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		assert_eq!(ClientCounter::<T>::get(), 1)
	}


	transfer {
		let caller: T::AccountId = whitelisted_caller();
		let client_id = Pallet::<T>::create_client().unwrap();
		let connection_id = ConnectionId::new(0);
		Pallet::<T>::create_connection(client_id, connection_id.clone()).unwrap();
		let port_id = PortId::transfer();
		let counterparty = channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(1)));
		let channel_end = ChannelEnd::new(
			channel::State::Init,
			Order::Unordered,
			counterparty,
			vec![connection_id],
			Version::new(VERSION.to_string()),
		);

		let balance = 100000 * CurrencyId::milli::<u128>();
		let channel_id = Pallet::<T>::open_channel(port_id.clone(), channel_end).unwrap();
		let denom = "transfer/channel-15/uatom";
		let foreign_asset_id = ibc_denom_to_foreign_asset_id(denom);
		let asset_id = <T as Config>::CurrencyFactory::create(
			RangeId::IBC_ASSETS,
			<T as DeFiComposableConfig>::Balance::zero(),
		).unwrap();
		<T as Config>::AssetRegistry::set_reserve_location(
			asset_id.into(),
			foreign_asset_id.into(),
			None,
			None,
		).unwrap();
		<<T as Config>::MultiCurrency as Mutate<T::AccountId>>::mint_into(
			asset_id.into(),
			&caller,
			balance.into(),
		).unwrap();

		let timeout = Timeout::Offset { timestamp: Some(1690894363), height: Some(2000) };

		let transfer_params = TransferParams {
			to:  MultiAddress::Raw("bob".to_string().as_bytes().to_vec()),
			source_channel: channel_id.sequence(),
			timeout,
		};

		Pallet::<T>::register_asset_id(asset_id.into(), denom.as_bytes().to_vec());
		<Params<T>>::put(PalletParams {
			send_enabled: true,
			receive_enabled: true
		});

		let amt = 1000 * CurrencyId::milli::<u128>();

	}:_(RawOrigin::Signed(caller.clone()), transfer_params, asset_id.into(), amt.into())
	verify {
		assert_eq!(<<T as Config>::MultiCurrency as Inspect<T::AccountId>>::balance(
			asset_id.into(),
			&caller
		), (balance - amt).into());
	}

	set_params {
		let pallet_params = PalletParams {
			send_enabled: true,
			receive_enabled: true
		};

	}:_(RawOrigin::Root, pallet_params)
	verify {
		assert_last_event::<T>(Event::<T>::ParamsUpdated {
			send_enabled: true,
			receive_enabled: true
		}.into())
	}

	on_chan_open_init {
		let mut output = HandlerOutputBuilder::new();
		let port_id = PortId::transfer();
		let counterparty = channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(1)));
		let connection_hops = vec![ConnectionId::new(0)];
		let version = Version::new(VERSION.to_string());
		let order = Order::Unordered;
		let channel_id = ChannelId::new(0);
		let mut handler = IbcModule::<T>::default();
	}:{
		handler.on_chan_open_init(&mut output, order, &connection_hops, &port_id, &channel_id, &counterparty, &version).unwrap();
	}

	on_chan_open_try {
		let mut output = HandlerOutputBuilder::new();
		let port_id = PortId::transfer();
		let counterparty = channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(1)));
		let connection_hops = vec![ConnectionId::new(0)];
		let version = Version::new(VERSION.to_string());
		let order = Order::Unordered;
		let channel_id = ChannelId::new(0);
		let mut handler = IbcModule::<T>::default();
	}:{
		handler.on_chan_open_try(&mut output, order, &connection_hops, &port_id, &channel_id, &counterparty, &version, &version).unwrap();
	}

	on_chan_open_ack {
		let mut output = HandlerOutputBuilder::new();
		let port_id = PortId::transfer();
		let version = Version::new(VERSION.to_string());
		let channel_id = ChannelId::new(0);
		let mut handler = IbcModule::<T>::default();
	}:{
		handler.on_chan_open_ack(&mut output, &port_id, &channel_id, &version).unwrap();
	}
	verify {
		assert_eq!(ChannelIds::<T>::get().len(), 1)
	}

	on_chan_open_confirm {
		let mut output = HandlerOutputBuilder::new();
		let port_id = PortId::transfer();
		let channel_id = ChannelId::new(0);
		let mut handler = IbcModule::<T>::default();
	}:{
		handler.on_chan_open_confirm(&mut output, &port_id, &channel_id).unwrap();
	}
	verify {
		assert_eq!(ChannelIds::<T>::get().len(), 1)
	}

	on_chan_close_init {
		let mut output = HandlerOutputBuilder::new();
		let port_id = PortId::transfer();
		let channel_id = ChannelId::new(0);
		let channel_ids = vec![channel_id.to_string().as_bytes().to_vec()];
		ChannelIds::<T>::put(channel_ids);
		let mut handler = IbcModule::<T>::default();
	}:{
		handler.on_chan_close_init(&mut output, &port_id, &channel_id).unwrap();
	}
	verify {
		assert_eq!(ChannelIds::<T>::get().len(), 0)
	}

	on_chan_close_confirm {
		let mut output = HandlerOutputBuilder::new();
		let port_id = PortId::transfer();
		let channel_id = ChannelId::new(0);
		let channel_ids = vec![channel_id.to_string().as_bytes().to_vec()];
		ChannelIds::<T>::put(channel_ids);
		let mut handler = IbcModule::<T>::default();
	}:{
		handler.on_chan_close_confirm(&mut output, &port_id, &channel_id).unwrap();
	}
	verify {
		assert_eq!(ChannelIds::<T>::get().len(), 0)
	}

	on_recv_packet {
		let caller: T::AccountId = whitelisted_caller();
		let client_id = Pallet::<T>::create_client().unwrap();
		let connection_id = ConnectionId::new(0);
		Pallet::<T>::create_connection(client_id, connection_id.clone()).unwrap();
		let port_id = PortId::transfer();
		let counterparty = channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(1)));
		let channel_end = channel::ChannelEnd::new(
			channel::State::Init,
			Order::Unordered,
			counterparty,
			vec![connection_id],
			Version::new(VERSION.to_string()),
		);


		let balance = 100000 * CurrencyId::milli::<u128>();
		let channel_id = Pallet::<T>::open_channel(port_id.clone(), channel_end).unwrap();
		let denom = "transfer/channel-1/PICA";
		let channel_escrow_address = get_channel_escrow_address(&port_id, channel_id).unwrap();
		let channel_escrow_address = <T as Config>::AccountIdConversion::try_from(channel_escrow_address).map_err(|_| ()).unwrap();
		let channel_escrow_address: T::AccountId = channel_escrow_address.into_account();

		<<T as Config>::MultiCurrency as Mutate<T::AccountId>>::mint_into(
			CurrencyId::PICA.into(),
			&channel_escrow_address,
			balance.into(),
		).unwrap();


		<Params<T>>::put(PalletParams {
			send_enabled: true,
			receive_enabled: true
		});

		let raw_user: AccountId32 =  caller.clone().into();
		let raw_user: &[u8] = raw_user.as_ref();
		let mut hex_string = hex::encode_upper(raw_user.to_vec());
		hex_string.insert_str(0, "0x");
		let prefixed_denom = PrefixedDenom::from_str(denom).unwrap();
		let amt = 1000 * CurrencyId::milli::<u128>();
		let coin = Coin {
			denom: prefixed_denom,
			amount: Amount::from_str(&format!("{:?}", amt)).unwrap()
		};
		let packet_data = PacketData {
			token: coin,
			sender: Signer::from_str("alice").unwrap(),
			receiver: Signer::from_str(&hex_string).unwrap(),
		};

		let data = serde_json::to_vec(&packet_data).unwrap();
		let packet = Packet {
			sequence: 0u64.into(),
			source_port: port_id.clone(),
			source_channel: ChannelId::new(1),
			destination_port: port_id,
			destination_channel: ChannelId::new(0),
			data,
			timeout_height: Height::new(2000, 5),
			timeout_timestamp: Timestamp::from_nanoseconds(1690894363u64.saturating_mul(1000000000))
				.unwrap(),
		 };
		 let mut handler = IbcModule::<T>::default();
		 let mut output = HandlerOutputBuilder::new();
		 let signer = Signer::from_str("relayer").unwrap();
	}:{

		let res = handler.on_recv_packet(&mut output, &packet, &signer);
		match res {
			OnRecvPacketAck::Successful(_, write_fn) => {
				write_fn(handler.as_any_mut()).unwrap()
			}
			_ => panic!("Expected successful execution")
		}

	 }
	verify {
		assert_eq!(<<T as Config>::MultiCurrency as Inspect<T::AccountId>>::balance(
			CurrencyId::PICA.into(),
			&caller
		), amt.into());
	}

	on_acknowledgement_packet {
		let caller: T::AccountId = whitelisted_caller();
		let client_id = Pallet::<T>::create_client().unwrap();
		let connection_id = ConnectionId::new(0);
		Pallet::<T>::create_connection(client_id, connection_id.clone()).unwrap();
		let port_id = PortId::transfer();
		let counterparty = channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(1)));
		let channel_end = channel::ChannelEnd::new(
			channel::State::Init,
			Order::Unordered,
			counterparty,
			vec![connection_id],
			Version::new(VERSION.to_string()),
		);


		let balance = 100000 * CurrencyId::milli::<u128>();
		let channel_id = Pallet::<T>::open_channel(port_id.clone(), channel_end).unwrap();
		let denom = "PICA";
		let channel_escrow_address = get_channel_escrow_address(&port_id, channel_id).unwrap();
		let channel_escrow_address = <T as Config>::AccountIdConversion::try_from(channel_escrow_address).map_err(|_| ()).unwrap();
		let channel_escrow_address: T::AccountId = channel_escrow_address.into_account();

		<<T as Config>::MultiCurrency as Mutate<T::AccountId>>::mint_into(
			CurrencyId::PICA.into(),
			&channel_escrow_address,
			balance.into(),
		).unwrap();


		<Params<T>>::put(PalletParams {
			send_enabled: true,
			receive_enabled: true
		});

		let raw_user: AccountId32 =  caller.clone().into();
		let raw_user: &[u8] = raw_user.as_ref();
		let mut hex_string = hex::encode_upper(raw_user.to_vec());
		hex_string.insert_str(0, "0x");
		let prefixed_denom = PrefixedDenom::from_str(denom).unwrap();
		let amt = 1000 * CurrencyId::milli::<u128>();
		let coin = Coin {
			denom: prefixed_denom,
			amount: Amount::from_str(&format!("{:?}", amt)).unwrap()
		};
		let packet_data = PacketData {
			token: coin,
			sender: Signer::from_str(&hex_string).unwrap(),
			receiver: Signer::from_str("alice").unwrap(),
		};

		let data = serde_json::to_vec(&packet_data).unwrap();
		let packet = Packet {
			sequence: 0u64.into(),
			source_port: port_id.clone(),
			source_channel: ChannelId::new(0),
			destination_port: port_id,
			destination_channel: ChannelId::new(1),
			data,
			timeout_height: Height::new(2000, 5),
			timeout_timestamp: Timestamp::from_nanoseconds(1690894363u64.saturating_mul(1000000000))
				.unwrap(),
		 };
		 let mut handler = IbcModule::<T>::default();
		 let mut output = HandlerOutputBuilder::new();
		 let signer = Signer::from_str("relayer").unwrap();
		 let ack: Acknowledgement = ACK_ERR_STR.to_string().as_bytes().to_vec().into();
	}:{
	   handler.on_acknowledgement_packet(&mut output, &packet, &ack, &signer).unwrap();
	}
	verify {
		assert_eq!(<<T as Config>::MultiCurrency as Inspect<T::AccountId>>::balance(
			CurrencyId::PICA.into(),
			&caller
		), amt.into());
	}

	on_timeout_packet {
		let caller: T::AccountId = whitelisted_caller();
		let client_id = Pallet::<T>::create_client().unwrap();
		let connection_id = ConnectionId::new(0);
		Pallet::<T>::create_connection(client_id, connection_id.clone()).unwrap();
		let port_id = PortId::transfer();
		let counterparty = channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(1)));
		let channel_end = channel::ChannelEnd::new(
			channel::State::Init,
			Order::Unordered,
			counterparty,
			vec![connection_id],
			Version::new(VERSION.to_string()),
		);


		let balance = 100000 * CurrencyId::milli::<u128>();
		let channel_id = Pallet::<T>::open_channel(port_id.clone(), channel_end).unwrap();
		let denom = "PICA";
		let channel_escrow_address = get_channel_escrow_address(&port_id, channel_id).unwrap();
		let channel_escrow_address = <T as Config>::AccountIdConversion::try_from(channel_escrow_address).map_err(|_| ()).unwrap();
		let channel_escrow_address: T::AccountId = channel_escrow_address.into_account();

		<<T as Config>::MultiCurrency as Mutate<T::AccountId>>::mint_into(
			CurrencyId::PICA.into(),
			&channel_escrow_address,
			balance.into(),
		).unwrap();


		<Params<T>>::put(PalletParams {
			send_enabled: true,
			receive_enabled: true
		});

		let raw_user: AccountId32 =  caller.clone().into();
		let raw_user: &[u8] = raw_user.as_ref();
		let mut hex_string = hex::encode_upper(raw_user.to_vec());
		hex_string.insert_str(0, "0x");
		let prefixed_denom = PrefixedDenom::from_str(denom).unwrap();
		let amt = 1000 * CurrencyId::milli::<u128>();
		let coin = Coin {
			denom: prefixed_denom,
			amount: Amount::from_str(&format!("{:?}", amt)).unwrap()
		};
		let packet_data = PacketData {
			token: coin,
			sender: Signer::from_str(&hex_string).unwrap(),
			receiver: Signer::from_str("alice").unwrap(),
		};

		let data = serde_json::to_vec(&packet_data).unwrap();
		let packet = Packet {
			sequence: 0u64.into(),
			source_port: port_id.clone(),
			source_channel: ChannelId::new(0),
			destination_port: port_id,
			destination_channel: ChannelId::new(1),
			data,
			timeout_height: Height::new(2000, 5),
			timeout_timestamp: Timestamp::from_nanoseconds(1690894363u64.saturating_mul(1000000000))
				.unwrap(),
		 };
		 let mut handler = IbcModule::<T>::default();
		 let mut output = HandlerOutputBuilder::new();
		 let signer = Signer::from_str("relayer").unwrap();
	}:{
		handler.on_timeout_packet(&mut output, &packet, &signer).unwrap();
	}
	verify {
		assert_eq!(<<T as Config>::MultiCurrency as Inspect<T::AccountId>>::balance(
			CurrencyId::PICA.into(),
			&caller
		), amt.into());
	}
}
