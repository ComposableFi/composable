//! Benchmarking setup for pallet-template

#[allow(unused)]
use super::super::*;
use crate::{
	benchmarks::tendermint_benchmark_utils::*, host_functions::HostFunctions,
	pallet::Pallet as PalletIbc, Any, Config, HostConsensusStates,
};
use core::str::FromStr;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::traits::Hooks;
use frame_system::RawOrigin;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::AnyConsensusState,
			client_state::{AnyClientState, ClientState},
			client_type::ClientType,
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
				conn_open_confirm::TYPE_URL as CONN_OPEN_CONFIRM_TYPE_URL, conn_open_init,
				conn_open_try::TYPE_URL as CONN_TRY_OPEN_TYPE_URL,
			},
			version::Version as ConnVersion,
		},
		ics04_channel::{
			channel::{ChannelEnd, State as ChannelState},
			context::{ChannelKeeper, ChannelReader},
			error::Error as Ics04Error,
			msgs::{
				acknowledgement::TYPE_URL as ACK_PACKET_TYPE_URL,
				chan_close_confirm::TYPE_URL as CHAN_CLOSE_CONFIRM_TYPE_URL,
				chan_close_init::TYPE_URL as CHAN_CLOSE_INIT_TYPE_URL,
				chan_open_ack::TYPE_URL as CHAN_OPEN_ACK_TYPE_URL,
				chan_open_confirm::TYPE_URL as CHAN_OPEN_CONFIRM_TYPE_URL,
				chan_open_init::{MsgChannelOpenInit, TYPE_URL as CHAN_OPEN_TYPE_URL},
				chan_open_try::TYPE_URL as CHAN_OPEN_TRY_TYPE_URL,
				recv_packet::TYPE_URL as RECV_PACKET_TYPE_URL,
				timeout::TYPE_URL as TIMEOUT_TYPE_URL,
			},
			packet::Receipt,
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	signer::Signer,
};
use scale_info::prelude::string::ToString;
use sp_std::vec;
use tendermint_proto::Protobuf;

const TIMESTAMP: u64 = 1650894363;

benchmarks! {
	where_clause {
		where u32: From<<T as frame_system::Config>::BlockNumber>,
				T: Send + Sync + pallet_timestamp::Config<Moment = u64> + parachain_info::Config,
	}

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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let value = create_client_update().encode_vec();

		let msg = Any { type_url: UPDATE_CLIENT_TYPE_URL.to_string().as_bytes().to_vec(), value };
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let client_state = ClientStates::<T>::get(client_id.as_bytes().to_vec());
		let client_state = AnyClientState::decode_vec(&*client_state).unwrap();
		assert_eq!(client_state.latest_height(), Height::new(0, 2));
	}

	// create connection
	connection_open_init {
		let mut ctx = routing::Context::<T>::new();
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let counterparty = Counterparty::new(counterparty_client_id, None, commitment_prefix);
		let delay_period = core::time::Duration::from_nanos(1000);

		let value = conn_open_init::MsgConnectionOpenInit {
			client_id: client_id.clone(),
			counterparty,
			version: None,
			delay_period,
			signer: Signer::from_str(MODULE_ID).unwrap()
		}.encode_vec();

		let msg = Any { type_url: conn_open_init::TYPE_URL.as_bytes().to_vec(), value };
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		assert_eq!(ConnectionClient::<T>::get(client_id.as_bytes().to_vec()).len(), 1);
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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		// Create a connection end and put in storage
		// Successful processing of a connection try open message requires a compatible connection end with state INIT
		// to exist on the local chain
		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Init, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();

		// We update the light client state so it can have the required client and consensus states required to process
		// the proofs that will be submitted
		let value = create_client_update().encode_vec();
		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };
		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();

		let (cs_state, value) = create_conn_open_try::<T>();
		// Update consensus state with the new root that we'll enable proofs to be correctly verified
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		let msg = Any { type_url: CONN_TRY_OPEN_TYPE_URL.as_bytes().to_vec(), value: value.encode_vec() };
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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();
		// Create a connection end and put in storage
		// Successful processing of a connection open confirm message requires a compatible connection end with state INIT or TRYOPEN
		// to exist on the local chain
		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Init, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();

		let value = create_client_update().encode_vec();
		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };
		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();

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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		// Create a connection end and put in storage
		// Successful processing of a connection open confirm message requires a compatible connection end with state TryOpen
		// to exist on the local chain
		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::TryOpen, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();

		// We update the light client state so it can have the required client and consensus states required to process
		// the proofs that will be submitted
		let value = create_client_update().encode_vec();
		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };
		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();

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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		// We update the light client state so it can have the required client and consensus states required to process
		// the proofs that will be submitted
		let value = create_client_update().encode_vec();
		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();

		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();

		let counterparty_channel = ibc::core::ics04_channel::channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(0)));
		// Create a channel end with a INIT state
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

		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();

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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();

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

		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();

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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();

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
		ctx.store_connection_channels(ConnectionId::new(0), &(port_id, ChannelId::new(0))).unwrap();

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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();

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
		ctx.store_connection_channels(ConnectionId::new(0), &(port_id, ChannelId::new(0))).unwrap();

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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();

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
		ctx.store_connection_channels(ConnectionId::new(0), &(port_id, ChannelId::new(0))).unwrap();

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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(0);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();
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
		ctx.store_next_sequence_recv((port_id, ChannelId::new(0)), 1u64.into()).unwrap();

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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(0);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();

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
		ctx.store_next_sequence_recv((port_id, ChannelId::new(0)), 1u64.into()).unwrap();

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
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(0);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		let value = create_client_update().encode_vec();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver::<_, HostFunctions>(&mut ctx, msg).unwrap();

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
		ctx.store_next_sequence_send((port_id, ChannelId::new(0)), 1u64.into()).unwrap();

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

	on_finalize {
		// counter for clients
		let a in 1..50;
		// counter for connections
		let b in 1..50;
		// counter for channels
		let c in 1..50;
		// counter for packet commitments
		let d in 1..50;
		// counter for packet acknowledgements
		let e in 1..50;
		// counter for packet receipts
		let f in 1..50;
		let mut ctx = crate::routing::Context::<T>::new();
		for i in 1..a {
			let (mock_client_state, mock_cs_state) = create_mock_state();
			let mock_client_state = AnyClientState::Tendermint(mock_client_state);
			let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
			let client_id = ClientId::new(mock_client_state.client_type(), i.into()).unwrap();
			ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
			ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
			ctx.store_consensus_state(client_id, Height::new(0, 1), mock_cs_state).unwrap();
		}

		for j in 1..b {
			let connection_id = ConnectionId::new(j.into());
			let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
			let delay_period = core::time::Duration::from_nanos(1000);
			let client_id = ClientId::new(ClientType::Tendermint, 0).unwrap();
			let connection_counterparty = Counterparty::new(client_id.clone(), Some(ConnectionId::new(j.into())), commitment_prefix);
			let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);
			ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		}

		for k in 1..c {
			let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
			let counterparty_channel = ibc::core::ics04_channel::channel::Counterparty::new(port_id.clone(), Some(ChannelId::new(k.into())));
			let channel_end = ChannelEnd::new(
				ibc::core::ics04_channel::channel::State::Open,
				ibc::core::ics04_channel::channel::Order::Unordered,
				counterparty_channel,
				vec![ConnectionId::new(k.into())],
				ibc::core::ics04_channel::Version::default()
			);

			ctx.store_channel((port_id.clone(), ChannelId::new(k.into())), &channel_end).unwrap();
		}

		for l in 1..d {
			let commitment = vec![0;32];
			let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
			let channel_id = ChannelId::new(0);
			PacketCommitment::<T>::insert((port_id.as_bytes(), channel_id.to_string().as_bytes(), l as u64), commitment)
		}

		for m in 1..e {
			let ack = vec![0;32];
			let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
			let channel_id = ChannelId::new(0);
			Acknowledgements::<T>::insert((port_id.as_bytes(), channel_id.to_string().as_bytes(), m as u64), ack)
		}

		for n in 1..f {
			let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
			let channel_id = ChannelId::new(0);
			PacketReceipt::<T>::insert((port_id.as_bytes(), channel_id.to_string().as_bytes(), n as u64), "Ok".as_bytes())
		}

	}: { PalletIbc::<T>::on_finalize(0u32.into())}
	verify {
		let commitment_roots = HostConsensusStates::<T>::get();
		assert_eq!(commitment_roots.len(), 1);
	}

	initiate_connection {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let params = ConnectionParams {
			version: (
				"1".as_bytes().to_vec(),
				vec![
					ibc::core::ics04_channel::channel::Order::Ordered.as_str().as_bytes().to_vec(),
					ibc::core::ics04_channel::channel::Order::Unordered.as_str().as_bytes().to_vec(),
				],
			),
			client_id: client_id.as_bytes().to_vec(),
			counterparty_client_id: counterparty_client_id.as_bytes().to_vec(),
			commitment_prefix: "ibc".as_bytes().to_vec(),
			delay_period: 1000,
		};

	}: _(RawOrigin::Root, params)
	verify {
		let connection_end = ConnectionReader::connection_end(&ctx, &ConnectionId::new(0)).unwrap();
		assert_eq!(connection_end.state, State::Init);
	}

	create_client {
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let msg = MsgCreateAnyClient::new(
			AnyClientState::Tendermint(mock_client_state),
			Some(AnyConsensusState::Tendermint(mock_cs_state)),
			Signer::from_str(MODULE_ID).unwrap(),
		)
		.unwrap()
		.encode_vec();

		let msg = Any { type_url: TYPE_URL.to_string().as_bytes().to_vec(), value: msg };
	}: _(RawOrigin::Root, msg)
	verify {
		assert_eq!(Clients::<T>::count(), 1)
	}
}
