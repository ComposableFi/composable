// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
	client_def::BeefyClient,
	client_state::{
		ClientState as BeefyClientState, UpgradeOptions as BeefyUpgradeOptions,
		BEEFY_CLIENT_STATE_TYPE_URL,
	},
	consensus_state::{ConsensusState as BeefyConsensusState, BEEFY_CONSENSUS_STATE_TYPE_URL},
	error::Error,
	header::{BeefyHeader, BEEFY_HEADER_TYPE_URL},
};
use beefy_client_primitives::error::BeefyClientError;
use ibc::{
	core::ics02_client::{
		client_consensus::ConsensusState, client_def::ClientDef, client_state::ClientState,
		header::Header, misbehaviour::Misbehaviour,
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
};
use ibc_derive::{ClientDef, ClientState, ConsensusState, Header, Misbehaviour, Protobuf};
use primitive_types::H256;
use serde::{Deserialize, Serialize};
use sp_storage::ChildInfo;
use sp_trie::StorageProof;

pub const MOCK_CLIENT_STATE_TYPE_URL: &str = "/ibc.mock.ClientState";
pub const MOCK_HEADER_TYPE_URL: &str = "/ibc.mock.Header";
pub const MOCK_MISBEHAVIOUR_TYPE_URL: &str = "/ibc.mock.Misbehavior";
pub const MOCK_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.mock.ConsensusState";

#[derive(Clone, Default, Debug, Eq)]
pub struct HostFunctionsManager;

impl beefy_client_primitives::HostFunctions for HostFunctionsManager {
	fn keccak_256(input: &[u8]) -> [u8; 32] {
		beefy_prover::Crypto::keccak_256(input)
	}

	fn secp256k1_ecdsa_recover_compressed(
		signature: &[u8; 65],
		value: &[u8; 32],
	) -> Option<Vec<u8>> {
		beefy_prover::Crypto::secp256k1_ecdsa_recover_compressed(signature, value)
	}

	fn verify_timestamp_extrinsic(
		root: H256,
		proof: &[Vec<u8>],
		value: &[u8],
	) -> Result<(), BeefyClientError> {
		beefy_prover::Crypto::verify_timestamp_extrinsic(root, proof, value)
	}
}

impl light_client_common::HostFunctions for HostFunctionsManager {
	fn verify_child_trie_proof<I>(root: &[u8; 32], proof: &[Vec<u8>], items: I) -> Result<(), Error>
	where
		I: IntoIterator<Item = (Vec<u8>, Option<Vec<u8>>)>,
	{
		let proof = StorageProof::new(proof);
		let child_info = ChildInfo::new_default(b"ibc/");
		sp_state_machine::read_child_proof_check(root.into(), proof, &child_info, items)
			.map_err(|err| Error::Custom(format!("Failed to verify child trie proof: {err:?}")))?;
		Ok(())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, ClientDef)]
pub enum AnyClient {
	Mock(MockClient),
	Beefy(BeefyClient<HostFunctionsManager>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnyUpgradeOptions {
	Mock(()),
	Beefy(BeefyUpgradeOptions),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, ClientState, Protobuf)]
#[serde(tag = "type")]
pub enum AnyClientState {
	#[ibc(proto_url = "MOCK_CLIENT_STATE_TYPE_URL")]
	Mock(MockClientState),
	#[serde(skip)]
	#[ibc(proto_url = "BEEFY_CLIENT_STATE_TYPE_URL")]
	Beefy(BeefyClientState<HostFunctionsManager>),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Header, Protobuf)]
#[allow(clippy::large_enum_variant)]
pub enum AnyHeader {
	#[ibc(proto_url = "MOCK_HEADER_TYPE_URL")]
	Mock(MockHeader),
	#[serde(skip)]
	#[ibc(proto_url = "BEEFY_HEADER_TYPE_URL")]
	Beefy(BeefyHeader),
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
	#[ibc(proto_url = "BEEFY_CONSENSUS_STATE_TYPE_URL")]
	Beefy(BeefyConsensusState),
	#[ibc(proto_url = "MOCK_CONSENSUS_STATE_TYPE_URL")]
	Mock(MockConsensusState),
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct MockClientTypes;

impl ClientTypes for MockClientTypes {
	type AnyHeader = AnyHeader;
	type AnyClientState = AnyClientState;
	type AnyConsensusState = AnyConsensusState;
	type AnyMisbehaviour = AnyMisbehaviour;
	type HostFunctions = HostFunctionsManager;
	type ClientDef = AnyClient;
	type HostBlock = ();
}
