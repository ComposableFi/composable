#![allow(unreachable_code)]

pub mod context;
pub mod host;

use crate::{
	client_def::TendermintClient,
	client_state::{
		ClientState as TendermintClientState, UpgradeOptions as TendermintUpgradeOptions,
	},
	consensus_state::ConsensusState as TendermintConsensusState,
	HostFunctionsProvider,
};

use crate::{client_message::ClientMessage, mock::host::MockHostBlock};
use ibc::{
	core::{
		ics02_client,
		ics02_client::{client_consensus::ConsensusState, client_state::ClientState},
	},
	mock::{
		client_def::MockClient,
		client_state::{MockClientState, MockConsensusState},
		context::ClientTypes,
		header::MockClientMessage,
	},
	prelude::*,
};
use ibc_derive::{ClientDef, ClientMessage, ClientState, ConsensusState};
use ibc_proto::google::protobuf::Any;
use tendermint_light_client_verifier::host_functions::HostFunctionsProvider as TendermintHostFunctionsProvider;
use tendermint_proto::Protobuf;

pub const MOCK_CLIENT_STATE_TYPE_URL: &str = "/ibc.mock.ClientState";
pub const MOCK_CLIENT_MESSAGE_TYPE_URL: &str = "/ibc.mock.ClientMessage";
pub const MOCK_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.mock.ConsensusState";

pub const TENDERMINT_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.ClientState";
pub const TENDERMINT_CLIENT_MESSAGE_TYPE_URL: &str =
	"/ibc.lightclients.tendermint.v1.ClientMessage";
pub const TENDERMINT_CONSENSUS_STATE_TYPE_URL: &str =
	"/ibc.lightclients.tendermint.v1.ConsensusState";

#[derive(Clone, Debug, PartialEq, Eq, ClientDef)]
pub enum AnyClient {
	Mock(MockClient),
	Tendermint(TendermintClient<Crypto>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnyUpgradeOptions {
	Mock(()),
	Tendermint(TendermintUpgradeOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, ClientState, Protobuf)]
pub enum AnyClientState {
	#[ibc(proto_url = "MOCK_CLIENT_STATE_TYPE_URL")]
	Mock(MockClientState),
	#[ibc(proto_url = "TENDERMINT_CLIENT_STATE_TYPE_URL")]
	Tendermint(TendermintClientState<Crypto>),
}
#[derive(Clone, Debug, PartialEq, Eq, ClientMessage)]
#[allow(clippy::large_enum_variant)]
pub enum AnyClientMessage {
	#[ibc(proto_url = "MOCK_CLIENT_MESSAGE_TYPE_URL")]
	Mock(MockClientMessage),
	#[ibc(proto_url = "TENDERMINT_CLIENT_MESSAGE_TYPE_URL")]
	Tendermint(ClientMessage),
}

impl Protobuf<Any> for AnyClientMessage {}

impl TryFrom<Any> for AnyClientMessage {
	type Error = ics02_client::error::Error;

	fn try_from(value: Any) -> Result<Self, Self::Error> {
		match value.type_url.as_str() {
			MOCK_CLIENT_MESSAGE_TYPE_URL =>
				Ok(Self::Mock(panic!("MockClientMessage doesn't implement Protobuf"))),
			TENDERMINT_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Tendermint(
				ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			_ => Err(ics02_client::error::Error::unknown_consensus_state_type(value.type_url)),
		}
	}
}

impl From<AnyClientMessage> for Any {
	fn from(client_msg: AnyClientMessage) -> Self {
		match client_msg {
			AnyClientMessage::Mock(_mock) => {
				panic!("MockClientMessage doesn't implement Protobuf");
			},
			AnyClientMessage::Tendermint(msg) => Any {
				type_url: TENDERMINT_CLIENT_MESSAGE_TYPE_URL.to_string(),
				value: msg.encode_vec(),
			},
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, ConsensusState, Protobuf)]
pub enum AnyConsensusState {
	#[ibc(proto_url = "TENDERMINT_CONSENSUS_STATE_TYPE_URL")]
	Tendermint(TendermintConsensusState),
	#[ibc(proto_url = "MOCK_CONSENSUS_STATE_TYPE_URL")]
	Mock(MockConsensusState),
}

impl From<MockConsensusState> for AnyConsensusState {
	fn from(mcs: MockConsensusState) -> Self {
		Self::Mock(mcs)
	}
}

impl From<MockClientState> for AnyClientState {
	fn from(mcs: MockClientState) -> Self {
		Self::Mock(mcs)
	}
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct MockClientTypes;
impl ClientTypes for MockClientTypes {
	type AnyClientMessage = AnyClientMessage;
	type AnyClientState = AnyClientState;
	type AnyConsensusState = AnyConsensusState;
	type ClientDef = AnyClient;
	type HostBlock = MockHostBlock;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Crypto;

impl ics23::HostFunctionsProvider for Crypto {
	fn sha2_256(_message: &[u8]) -> [u8; 32] {
		unimplemented!()
	}

	fn sha2_512(_message: &[u8]) -> [u8; 64] {
		unimplemented!()
	}

	fn sha2_512_truncated(_message: &[u8]) -> [u8; 32] {
		unimplemented!()
	}

	fn sha3_512(_message: &[u8]) -> [u8; 64] {
		unimplemented!()
	}

	fn ripemd160(_message: &[u8]) -> [u8; 20] {
		unimplemented!()
	}
}

impl TendermintHostFunctionsProvider for Crypto {
	fn sha2_256(_preimage: &[u8]) -> [u8; 32] {
		unimplemented!()
	}

	fn ed25519_verify(_sig: &[u8], _msg: &[u8], _pub_key: &[u8]) -> bool {
		unimplemented!()
	}

	fn secp256k1_verify(_sig: &[u8], _message: &[u8], _public: &[u8]) -> bool {
		unimplemented!()
	}
}

impl HostFunctionsProvider for Crypto {}
