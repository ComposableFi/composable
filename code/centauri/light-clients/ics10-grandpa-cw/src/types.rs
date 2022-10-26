use std::ops::Deref;

use ibc_derive::{ClientDef, ClientMessage, ClientState, ConsensusState, Protobuf};

use ics10_grandpa::client_def::GrandpaClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::host_functions::HostFunctionsManager;


#[derive(Clone, Debug, PartialEq, Eq, ClientDef)]
pub enum AnyClient {
	Grandpa(GrandpaClient<HostFunctionsManager>),
	#[cfg(test)]
	Mock(ibc::mock::client_def::MockClient),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Protobuf, ConsensusState)]
#[serde(tag = "type")]
pub enum AnyConsensusState {
	#[ibc(proto_url = "GRANDPA_CONSENSUS_STATE_TYPE_URL")]
	Grandpa(ConsensusState),
	#[ibc(proto_url = "MOCK_CONSENSUS_STATE_TYPE_URL")]
	Mock(MockConsensusState),
}


#[derive(Debug,Clone, Deserialize, Serialize, JsonSchema, PartialEq)]
// #[cw_serde]
pub struct Height {
	/// Previously known as "epoch"
	pub revision_number: u64,

	/// The height of a block
	pub revision_height: u64,
}

impl Deref for Height {
	type Target = ibc::Height;
	fn deref(&self) -> &Self::Target {
		&ibc::Height {
			revision_number: self.revision_number,
			revision_height: self.revision_height,
		}
	}
}
