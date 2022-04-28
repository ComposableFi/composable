//! Benchmarking setup for pallet-template

use super::*;

use crate::benchmark_utils::*;
#[allow(unused)]
use crate::Pallet as PalletIbc;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::AnyConsensusState,
			client_state::{AnyClientState, ClientState},
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
				conn_open_init,
				conn_open_try::{MsgConnectionOpenTry, TYPE_URL as CONN_TRY_OPEN},
			},
			version::Version as ConnVersion,
		},
		ics04_channel::{
			channel::{ChannelEnd, State as ChannelState},
			context::{ChannelKeeper, ChannelReader},
			msgs::{
				chan_open_ack::TYPE_URL as CHAN_OPEN_ACK_TYPE_URL,
				chan_open_confirm::TYPE_URL as CHAN_OPEN_CONFIRM_TYPE_URL,
				chan_open_init::{MsgChannelOpenInit, TYPE_URL as CHAN_OPEN_TYPE_URL},
				chan_open_try::TYPE_URL as CHAN_OPEN_TRY_TYPE_URL,
			},
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	signer::Signer,
};
use ibc_trait::IbcTrait;
use scale_info::prelude::string::ToString;
use sp_std::vec;
use tendermint_proto::Protobuf;

benchmarks! {
	where_clause {
		where u32: From<<T as frame_system::Config>::BlockNumber>,
				T: Send + Sync + pallet_timestamp::Config<Moment = u64>,
	}
	// create_client
	create_client {
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let msg = MsgCreateAnyClient::new(
			AnyClientState::Tendermint(mock_client_state),
			AnyConsensusState::Tendermint(mock_cs_state),
			Signer::new("relayer"),
		)
		.unwrap()
		.encode_vec()
		.unwrap();

		let msg = Any { type_url: TYPE_URL.to_string().as_bytes().to_vec(), value: msg };
		let caller: T::AccountId = whitelisted_caller();

	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		assert_eq!(Clients::<T>::count(), 1)
	}

	// update_client
	update_client {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = 1650894363u64.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let value = create_client_update().encode_vec().unwrap();

		let msg = Any { type_url: UPDATE_CLIENT_TYPE_URL.to_string().as_bytes().to_vec(), value };
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		assert_last_event::<T>(Event::<T>::ProcessedIBCMessages.into());
		let client_state = ClientStates::<T>::get(client_id.as_bytes().to_vec());
		let client_state = AnyClientState::decode_vec(&*client_state).unwrap();
		assert_eq!(client_state.latest_height(), Height::new(0, 2));
	}

	// create connection
	connection_init {
		let mut ctx = routing::Context::<T>::new();
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
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
			signer: Signer::new("relayer")
		}.encode_vec().unwrap();

		let msg = Any { type_url: conn_open_init::TYPE_URL.as_bytes().to_vec(), value };
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		assert_eq!(ConnectionClient::<T>::get(client_id.as_bytes().to_vec()).len(), 1);
		assert_last_event::<T>(Event::<T>::ProcessedIBCMessages.into())
	}


	// connection open try
	// conn_try_open {
	// 	let mut ctx = routing::Context::<T>::new();
	// 	let (mock_client_state, mock_cs_state) = create_mock_state();
	// 	let mock_client_state = AnyClientState::Tendermint(mock_client_state);
	// 	let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
	// 	let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
	// 	let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
	// 	ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
	// 	ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
	// 	ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();
	//
	// 	let connection_id = ConnectionId::new(0);
	// 	let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
	// 	let delay_period = core::time::Duration::from_nanos(1000);
	// 	let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
	// 	let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);
	//
	// 	ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
	// 	ctx.store_connection_to_client(connection_id, &client_id).unwrap();
	//
	// 	let value = create_conn_open_try().encode_vec().unwrap();
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	let msg = Any { type_url: CONN_TRY_OPEN.as_bytes().to_vec(), value };
	// }: deliver(RawOrigin::Signed(caller), vec![msg])
	// verify {
	// 	let connection_end = ctx.connection_end(&ConnectionId::new(0)).unwrap();
	// 	assert_eq!(connection_end.state, State::TryOpen);
	// }

	// create channel
	create_channel {
		let mut ctx = routing::Context::<T>::new();
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
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
			signer: Signer::new("relayer")
		}.encode_vec().unwrap();

		let capability = PalletIbc::<T>::bind_port(port_id).unwrap();
		pallet_ibc_ping::Pallet::<T>::set_capability(capability.index());

		let caller: T::AccountId = whitelisted_caller();
		let msg = Any { type_url: CHAN_OPEN_TYPE_URL.as_bytes().to_vec(), value };
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		assert_eq!(ChannelCounter::<T>::get(), 1);
	}

	// channel_open_try
	channel_open_try {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = 1650894363u64.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
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
		let value = create_client_update().encode_vec().unwrap();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
		let capability = PalletIbc::<T>::bind_port(port_id.clone()).unwrap();
		pallet_ibc_ping::Pallet::<T>::set_capability(capability.index());

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
			signer: Signer::new("relayer")
		}.encode_vec().unwrap();

		let msg = ibc_proto::google::protobuf::Any  { type_url: CHAN_OPEN_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let (cs_state, value) = create_chan_open_try();
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let msg = Any {
			type_url: CHAN_OPEN_TRY_TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec().unwrap()
		};
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let channel_end = ctx.channel_end(&(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), ChannelId::new(0))).unwrap();
		assert_eq!(channel_end.state, ChannelState::TryOpen);
	}

	// channel_open_ack
	channel_open_ack {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = 1650894363u64.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
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
		let value = create_client_update().encode_vec().unwrap();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
		let capability = PalletIbc::<T>::bind_port(port_id.clone()).unwrap();
		pallet_ibc_ping::Pallet::<T>::set_capability(capability.index());

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
			signer: Signer::new("relayer")
		}.encode_vec().unwrap();

		let msg = ibc_proto::google::protobuf::Any  { type_url: CHAN_OPEN_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let (cs_state, value) = create_chan_open_ack();
		ctx.store_consensus_state(client_id, Height::new(0, 2), AnyConsensusState::Tendermint(cs_state)).unwrap();
		let msg = Any {
			type_url: CHAN_OPEN_ACK_TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec().unwrap()
		};
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let channel_end = ctx.channel_end(&(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), ChannelId::new(0))).unwrap();
		assert_eq!(channel_end.state, ChannelState::Open);
	}

	// channel_open_confirm
	channel_open_confirm {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = 1650894363u64.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
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
		let value = create_client_update().encode_vec().unwrap();

		let msg = ibc_proto::google::protobuf::Any  { type_url: UPDATE_CLIENT_TYPE_URL.to_string(), value };

		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();

		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
		let capability = PalletIbc::<T>::bind_port(port_id.clone()).unwrap();
		pallet_ibc_ping::Pallet::<T>::set_capability(capability.index());

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
			value: value.encode_vec().unwrap()
		};
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		let channel_end = ctx.channel_end(&(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), ChannelId::new(0))).unwrap();
		assert_eq!(channel_end.state, ChannelState::Open);
	}
}

// impl_benchmark_test_suite!(PalletIbc, crate::mock::new_test_ext(), crate::mock::Test,);
