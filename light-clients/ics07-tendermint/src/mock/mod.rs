pub mod context;
pub mod host;

use crate::{
	client_def::TendermintClient,
	client_state::{
		ClientState as TendermintClientState, UpgradeOptions as TendermintUpgradeOptions,
	},
	consensus_state::ConsensusState as TendermintConsensusState,
	header::Header as TendermintHeader,
	HostFunctionsProvider,
};

use crate::mock::host::MockHostBlock;
use core::{convert::Infallible, time::Duration};
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState,
			client_def::{ClientDef, ConsensusUpdateResult},
			client_state::{ClientState, ClientType},
			error::Error,
			header::Header,
			height::Height,
			misbehaviour::Misbehaviour,
		},
		ics03_connection::connection::ConnectionEnd,
		ics04_channel::{
			channel::ChannelEnd,
			commitment::{AcknowledgementCommitment, PacketCommitment},
			packet::Sequence,
		},
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
		ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
		ics26_routing::context::ReaderContext,
	},
	downcast,
	mock::{
		client_def::MockClient,
		client_state::{MockClientState, MockConsensusState},
		context::ClientTypes,
		header::MockHeader,
		misbehaviour::MockMisbehaviour,
	},
	prelude::*,
	timestamp::Timestamp,
};
use ibc_proto::google::protobuf::Any;
use tendermint_light_client_verifier::host_functions::HostFunctionsProvider as TendermintHostFunctionsProvider;
use tendermint_proto::Protobuf;

pub const MOCK_CLIENT_STATE_TYPE_URL: &str = "/ibc.mock.ClientState";
pub const MOCK_HEADER_TYPE_URL: &str = "/ibc.mock.Header";
pub const MOCK_MISBEHAVIOUR_TYPE_URL: &str = "/ibc.mock.Misbehavior";
pub const MOCK_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.mock.ConsensusState";

pub const TENDERMINT_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.ClientState";
pub const TENDERMINT_HEADER_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.Header";
pub const TENDERMINT_CONSENSUS_STATE_TYPE_URL: &str =
	"/ibc.lightclients.tendermint.v1.ConsensusState";

#[derive(Clone, Debug, PartialEq, Eq, ClientDef)]
pub enum AnyClient {
	Mock(MockClient),
	Tendermint(TendermintClient<Crypto>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnyUpgradeOptions {
	Mock(()),
	Tendermint(TendermintUpgradeOptions),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, ClientState, Protobuf)]
#[serde(tag = "type")]
pub enum AnyClientState {
	#[ibc(proto_url = "MOCK_CLIENT_STATE_TYPE_URL")]
	Mock(MockClientState),
	#[serde(skip)]
	#[ibc(proto_url = "TENDERMINT_CLIENT_STATE_TYPE_URL")]
	Tendermint(TendermintClientState<Crypto>),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Header, Protobuf)]
#[allow(clippy::large_enum_variant)]
pub enum AnyHeader {
	#[ibc(proto_url = "MOCK_HEADER_TYPE_URL")]
	Mock(MockHeader),
	#[serde(skip)]
	#[ibc(proto_url = "TENDERMINT_HEADER_TYPE_URL")]
	Tendermint(TendermintHeader),
}

#[derive(Clone, Debug, PartialEq, Misbehaviour, Protobuf)]
#[allow(clippy::large_enum_variant)]
pub enum AnyMisbehaviour {
	#[ibc(proto_url = "MOCK_MISBEHAVIOUR_TYPE_URL")]
	Mock(MockMisbehaviour),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, ConsensusState, Protobuf)]
#[serde(tag = "type")]
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
	type AnyHeader = AnyHeader;
	type AnyClientState = AnyClientState;
	type AnyConsensusState = AnyConsensusState;
	type AnyMisbehaviour = AnyMisbehaviour;
	type HostFunctions = Crypto;
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
