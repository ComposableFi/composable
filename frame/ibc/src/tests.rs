use crate::{mock::*, Any, ConnectionParams};
use frame_support::assert_ok;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::AnyConsensusState,
			client_state::{AnyClientState, ClientState},
			msgs::create_client::{MsgCreateAnyClient, TYPE_URL},
		},
		ics24_host::identifier::ClientId,
	},
	mock::{
		client_state::{MockClientState, MockConsensusState},
		header::MockHeader,
	},
	signer::Signer,
};
use ibc_trait::OpenChannelParams;
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
			Signer::new("relayer"),
		)
		.unwrap()
		.encode_vec()
		.unwrap();

		let msg = Any { type_url: TYPE_URL.to_string().as_bytes().to_vec(), value: msg };

		assert_ok!(Ibc::deliver(Origin::signed(1u64).into(), vec![msg]));

		let params = ConnectionParams {
			connnection_id: "test-connection".as_bytes().to_vec(),
			versions: vec![],
			client_id: client_id.as_bytes().to_vec(),
			counterparty_client_id: counterparty_client_id.as_bytes().to_vec(),
			commitment_prefix: "ibc".as_bytes().to_vec(),
			delay_period: 1000,
		};

		assert_ok!(Ibc::initiate_connection(Origin::root(), params));
	})
}

#[test]
fn should_open_a_channel() {
	new_test_ext().execute_with(|| {
		assert_ok!(Ping::bind_ibc_port(Origin::root()));

		let mock_client_state = MockClientState::new(MockHeader::default());
		let mock_cs_state = MockConsensusState::new(MockHeader::default());
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
		let msg = MsgCreateAnyClient::new(
			AnyClientState::Mock(mock_client_state),
			AnyConsensusState::Mock(mock_cs_state),
			Signer::new("relayer"),
		)
		.unwrap()
		.encode_vec()
		.unwrap();

		let msg = Any { type_url: TYPE_URL.to_string().as_bytes().to_vec(), value: msg };

		assert_ok!(Ibc::deliver(Origin::signed(1u64).into(), vec![msg]));

		let params = ConnectionParams {
			connnection_id: "test-connection".as_bytes().to_vec(),
			versions: vec![],
			client_id: client_id.as_bytes().to_vec(),
			counterparty_client_id: counterparty_client_id.as_bytes().to_vec(),
			commitment_prefix: "ibc".as_bytes().to_vec(),
			delay_period: 1000,
		};

		assert_ok!(Ibc::initiate_connection(Origin::root(), params));

		let params = OpenChannelParams {
			state: 1,
			order: 1,
			connection_id: "test-connection".as_bytes().to_vec(),
			counterparty_port_id: "ibc-ping".as_bytes().to_vec(),
			version: vec![],
		};

		assert_ok!(Ping::open_channel(Origin::root(), params));
	})
}
