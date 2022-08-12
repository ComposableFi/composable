use crate::{mock::*, Any, Config, MODULE_ID};
use core::time::Duration;
use frame_support::{assert_ok, traits::Get};
use ibc::{
	core::{
		ics02_client::{
			client_consensus::AnyConsensusState,
			client_state::{AnyClientState, ClientState},
			height::Height,
			msgs::create_client::{MsgCreateAnyClient, TYPE_URL},
		},
		ics03_connection::{
			connection::Counterparty,
			msgs::{conn_open_ack, conn_open_init},
			version::Version as ConnVersion,
		},
		ics04_channel::{msgs::chan_open_ack, Version as ChanVersion},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	mock::{
		client_state::{MockClientState, MockConsensusState},
		header::MockHeader,
	},
	proofs::{ConsensusProof, Proofs},
	signer::Signer,
};
use ibc_primitives::OpenChannelParams;
use pallet_ibc_ping::SendPingParams;
use sp_runtime::AccountId32;
use std::str::FromStr;
use tendermint_proto::Protobuf;

// Create a client and initialize a connection
#[test]
fn initialize_connection() {
	new_test_ext().execute_with(|| {
		let mock_client_state = MockClientState::new(MockHeader::default());
		let mock_cs_state = MockConsensusState::new(MockHeader::default());
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
		let msg = MsgCreateAnyClient::new(
			AnyClientState::Mock(mock_client_state),
			AnyConsensusState::Mock(mock_cs_state),
			Signer::from_str(MODULE_ID).unwrap(),
		)
		.unwrap()
		.encode_vec();

		let commitment_prefix: CommitmentPrefix =
			<Test as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();

		let msg = Any { type_url: TYPE_URL.to_string().as_bytes().to_vec(), value: msg };

		assert_ok!(Ibc::deliver_permissioned(Origin::root(), vec![msg]));

		let value = conn_open_init::MsgConnectionOpenInit {
			client_id: client_id.clone(),
			counterparty: Counterparty::new(
				counterparty_client_id.clone(),
				Some(ConnectionId::new(1)),
				commitment_prefix.clone(),
			),
			version: Some(ConnVersion::default()),
			delay_period: Duration::from_nanos(1000),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		};

		let msg = Any {
			type_url: conn_open_init::TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec(),
		};

		assert_ok!(Ibc::deliver_permissioned(Origin::root(), vec![msg]));
	})
}

#[test]
fn should_open_a_channel() {
	new_test_ext().execute_with(|| {
		let mock_client_state = MockClientState::new(MockHeader::default());
		let mock_cs_state = MockConsensusState::new(MockHeader::default());
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
		let msg = MsgCreateAnyClient::new(
			AnyClientState::Mock(mock_client_state),
			AnyConsensusState::Mock(mock_cs_state),
			Signer::from_str(MODULE_ID).unwrap(),
		)
		.unwrap()
		.encode_vec();

		let msg = Any { type_url: TYPE_URL.to_string().as_bytes().to_vec(), value: msg };

		assert_ok!(Ibc::deliver_permissioned(Origin::root(), vec![msg]));
		let commitment_prefix: CommitmentPrefix =
			<Test as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let value = conn_open_init::MsgConnectionOpenInit {
			client_id: client_id.clone(),
			counterparty: Counterparty::new(
				counterparty_client_id.clone(),
				Some(ConnectionId::new(1)),
				commitment_prefix.clone(),
			),
			version: Some(ConnVersion::default()),
			delay_period: Duration::from_nanos(1000),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		};

		let msg = Any {
			type_url: conn_open_init::TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec(),
		};

		assert_ok!(Ibc::deliver_permissioned(Origin::root(), vec![msg]));

		let params = OpenChannelParams {
			order: 1,
			connection_id: "connection-0".as_bytes().to_vec(),
			counterparty_port_id: "ping".as_bytes().to_vec(),
			version: vec![],
		};

		assert_ok!(Ping::open_channel(Origin::root(), params));
	})
}

#[test]
fn should_send_ping_packet() {
	let mut ext = new_test_ext();
	ext.execute_with(|| {
		frame_system::Pallet::<Test>::set_block_number(1u32.into());

		let mock_client_state = MockClientState::new(MockHeader::new(Height::new(0, 1)));
		let mock_cs_state = MockConsensusState::new(MockHeader::new(Height::new(0, 1)));
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
		let msg = MsgCreateAnyClient::new(
			AnyClientState::Mock(mock_client_state),
			AnyConsensusState::Mock(mock_cs_state),
			Signer::from_str(MODULE_ID).unwrap(),
		)
		.unwrap()
		.encode_vec();

		let msg = Any { type_url: TYPE_URL.to_string().as_bytes().to_vec(), value: msg };
		assert_ok!(Ibc::deliver_permissioned(Origin::root(), vec![msg]));
		let commitment_prefix: CommitmentPrefix =
			<Test as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap();
		let value = conn_open_init::MsgConnectionOpenInit {
			client_id: client_id.clone(),
			counterparty: Counterparty::new(
				counterparty_client_id.clone(),
				Some(ConnectionId::new(1)),
				commitment_prefix.clone(),
			),
			version: Some(ConnVersion::default()),
			delay_period: Duration::from_nanos(1000),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		};

		let msg = Any {
			type_url: conn_open_init::TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec(),
		};

		assert_ok!(Ibc::deliver_permissioned(Origin::root(), vec![msg]));

		crate::Pallet::<Test>::insert_default_consensus_state(1);
		// Acknowledge connection so it's state is open
		let value = conn_open_ack::MsgConnectionOpenAck {
			connection_id: ConnectionId::new(0),
			counterparty_connection_id: ConnectionId::new(1),
			client_state: None,
			proofs: Proofs::new(
				vec![0u8; 32].try_into().unwrap(),
				Some(vec![0u8; 32].try_into().unwrap()),
				Some(
					ConsensusProof::new(
						vec![0u8; 32].try_into().unwrap(),
						Height::new(u32::from(ParachainInfo::get()).into(), 1),
					)
					.unwrap(),
				),
				None,
				Height::new(0, 1),
			)
			.unwrap(),
			version: ConnVersion::default(),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		}
		.encode_vec();

		let msg = Any { type_url: conn_open_ack::TYPE_URL.as_bytes().to_vec(), value };

		assert_ok!(Ibc::deliver_permissioned(Origin::root(), vec![msg]));

		let params = OpenChannelParams {
			order: 1,
			connection_id: "connection-0".as_bytes().to_vec(),
			counterparty_port_id: "ping".as_bytes().to_vec(),
			version: ChanVersion::default().to_string().as_bytes().to_vec(),
		};

		assert_ok!(Ping::open_channel(Origin::root(), params));

		// Acknowledge channel so it's state is open
		let value = chan_open_ack::MsgChannelOpenAck {
			port_id: PortId::from_str("ping").unwrap(),
			channel_id: ChannelId::new(0),
			counterparty_channel_id: ChannelId::new(1),
			counterparty_version: ChanVersion::default(),
			proofs: Proofs::new(
				vec![0u8; 32].try_into().unwrap(),
				Some(vec![0u8; 32].try_into().unwrap()),
				Some(
					ConsensusProof::new(
						vec![0u8; 32].try_into().unwrap(),
						Height::new(u32::from(ParachainInfo::get()).into(), 1),
					)
					.unwrap(),
				),
				None,
				Height::new(0, 1),
			)
			.unwrap(),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		}
		.encode_vec();

		let msg = Any { type_url: chan_open_ack::TYPE_URL.as_bytes().to_vec(), value };

		assert_ok!(Ibc::deliver(Origin::signed(AccountId32::new([0; 32])).into(), vec![msg]));

		let params = SendPingParams {
			data: "ping".as_bytes().to_vec(),
			timeout_height_offset: 10,
			timeout_timestamp_offset: 10000u64,
			channel_id: 0,
		};

		assert_ok!(Ping::send_ping(Origin::root(), params));
	});

	// ext.persist_offchain_overlay();

	// ext.execute_with(|| {
	// 	let offchain_packet = crate::Pallet::<Test>::get_offchain_packets(
	// 		"channel-0".as_bytes().to_vec(),
	// 		"ping".as_bytes().to_vec(),
	// 		vec![1],
	// 	)
	// 	.unwrap();
	// 	assert_eq!(offchain_packet.len(), 1);
	// })
}
