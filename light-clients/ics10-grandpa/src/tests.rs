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
	client_message::{ClientMessage, Header, RelayChainHeader},
	client_state::ClientState,
	consensus_state::ConsensusState,
	mock::{
		AnyClientMessage, AnyClientState, AnyConsensusState, HostFunctionsManager, MockClientTypes,
	},
};
use beefy_prover::helpers::{fetch_timestamp_extrinsic_with_proof, TimeStampExtWithProof};
use codec::Decode;
use finality_grandpa_rpc::GrandpaApiClient;
use futures::stream::StreamExt;
use grandpa_client::justification::GrandpaJustification;
use grandpa_client_primitives::{parachain_header_storage_key, ParachainHeaderProofs};
use grandpa_prover::{runtime, GrandpaProver, JustificationNotification};
use ibc::{
	core::{
		ics02_client::{
			client_state::ClientState as _,
			context::{ClientKeeper, ClientReader},
			handler::{dispatch, ClientResult::Update},
			msgs::{
				create_client::MsgCreateAnyClient, update_client::MsgUpdateAnyClient, ClientMsg,
			},
		},
		ics24_host::identifier::{ChainId, ClientId},
	},
	events::IbcEvent,
	handler::HandlerOutput,
	mock::{context::MockContext, host::MockHostType},
	test_utils::get_dummy_account_id,
	Height,
};
use primitive_types::H256;
use sp_finality_grandpa::AuthorityList;
use std::{mem::size_of_val, time::Duration};
use subxt::rpc::{rpc_params, ClientT};

pub type Justification = GrandpaJustification<RelayChainHeader>;

#[tokio::test]
async fn test_continuous_update_of_grandpa_client() {
	let client_id = ClientId::new(ClientState::<HostFunctionsManager>::client_type(), 0).unwrap();

	let chain_start_height = Height::new(1, 11);

	let mut ctx = MockContext::<MockClientTypes>::new(
		ChainId::new("mockgaiaA".to_string(), 1),
		MockHostType::Mock,
		5,
		chain_start_height,
	);
	ctx.block_time = Duration::from_secs(600);

	let signer = get_dummy_account_id();

	let url = std::env::var("NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9944".to_string());
	let relay_client = subxt::ClientBuilder::new()
		.set_url(url)
		.build::<subxt::DefaultConfig>()
		.await
		.unwrap();

	let para_url = std::env::var("NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9988".to_string());
	let para_client = subxt::ClientBuilder::new()
		.set_url(para_url)
		.build::<subxt::DefaultConfig>()
		.await
		.unwrap();
	let grandpa_prover = GrandpaProver {
		relay_client: relay_client.clone(),
		para_client: para_client.clone(),
		para_id: 2001,
	};

	println!("Waiting for grandpa proofs to become available");
	relay_client
		.rpc()
		.subscribe_blocks()
		.await
		.unwrap()
		.filter_map(|result| futures::future::ready(result.ok()))
		.skip_while(|h| futures::future::ready(h.number < 210))
		.take(1)
		.collect::<Vec<_>>()
		.await;
	println!("Grandpa proofs are now available");
	let api =
		grandpa_prover.relay_client.clone().to_runtime_api::<runtime::api::RuntimeApi<
			subxt::DefaultConfig,
			subxt::PolkadotExtrinsicParams<_>,
		>>();

	let (client_state, consensus_state) = loop {
		let current_set_id = api
			.storage()
			.grandpa()
			.current_set_id(None)
			.await
			.expect("Failed to fetch current set id");

		let current_authorities = {
			let bytes = relay_client
				.rpc()
				.client
				.request::<String>(
					"state_call",
					rpc_params!("GrandpaApi_grandpa_authorities", "0x"),
				)
				.await
				.map(|res| hex::decode(&res[2..]))
				.expect("Failed to fetch authorities")
				.expect("Failed to hex decode authorities");

			AuthorityList::decode(&mut &bytes[..]).expect("Failed to scale decode authorities")
		};

		let latest_relay_hash = relay_client
			.rpc()
			.finalized_head()
			.await
			.expect("Failed to fetch finalized header");
		let latest_relay_header = relay_client
			.rpc()
			.header(Some(latest_relay_hash))
			.await
			.expect("Failed to fetch finalized header")
			.expect("Failed to fetch finalized header");

		let head_data = api
			.storage()
			.paras()
			.heads(
				&runtime::api::runtime_types::polkadot_parachain::primitives::Id(
					grandpa_prover.para_id,
				),
				Some(latest_relay_hash),
			)
			.await
			.unwrap()
			.unwrap();
		let decoded_para_head = frame_support::sp_runtime::generic::Header::<
			u32,
			sp_runtime::traits::BlakeTwo256,
		>::decode(&mut &*head_data.0)
		.expect("Failed to decode parachain header");
		// we can't use the genesis block to construct the initial state.
		if decoded_para_head.number == 0 {
			continue
		}
		let client_state = ClientState {
			relay_chain: Default::default(),
			latest_relay_hash,
			frozen_height: None,
			latest_para_height: decoded_para_head.number,
			para_id: grandpa_prover.para_id,
			current_set_id,
			current_authorities,
			_phantom: Default::default(),
		};
		let subxt_block_number: subxt::BlockNumber = decoded_para_head.number.into();
		let block_hash = grandpa_prover
			.para_client
			.rpc()
			.block_hash(Some(subxt_block_number))
			.await
			.unwrap();

		let TimeStampExtWithProof { ext: timestamp_extrinsic, proof: extrinsic_proof } =
			fetch_timestamp_extrinsic_with_proof(&grandpa_prover.para_client, block_hash)
				.await
				.unwrap();
		let state_proof = grandpa_prover
			.relay_client
			.rpc()
			.read_proof(
				vec![parachain_header_storage_key(grandpa_prover.para_id)],
				Some(latest_relay_hash),
			)
			.await
			.expect("Failed to fetch state proof!")
			.proof
			.into_iter()
			.map(|bytes| bytes.0)
			.collect();

		let header_proof =
			ParachainHeaderProofs { state_proof, extrinsic: timestamp_extrinsic, extrinsic_proof };

		let (_, consensus_state) = ConsensusState::from_header::<HostFunctionsManager>(
			header_proof,
			grandpa_prover.para_id,
			latest_relay_header.state_root,
		)
		.unwrap();

		break (AnyClientState::Grandpa(client_state), AnyConsensusState::Grandpa(consensus_state))
	};

	let create_client =
		MsgCreateAnyClient { client_state, consensus_state, signer: signer.clone() };

	// Create the client
	let res = dispatch(&ctx, ClientMsg::CreateClient(create_client)).unwrap();
	ctx.store_client_result(res.result).unwrap();
	let subscription =
		GrandpaApiClient::<JustificationNotification, H256, u32>::subscribe_justifications(
			&*relay_client.rpc().client,
		)
		.await
		.expect("Failed to subscribe to grandpa justifications");
	let mut subscription = subscription.take(100);

	while let Some(Ok(JustificationNotification(sp_core::Bytes(justification_bytes)))) =
		subscription.next().await
	{
		println!("========= New Justification =========");
		println!("justification size: {}kb", size_of_val(&*justification_bytes) / 1000);
		let client_state: ClientState<HostFunctionsManager> =
			match ctx.client_state(&client_id).unwrap() {
				AnyClientState::Grandpa(client_state) => client_state,
				_ => panic!("unexpected client state"),
			};
		println!("current_set_id: {}", client_state.current_set_id);

		let justification = Justification::decode(&mut &justification_bytes[..])
			.expect("Failed to decode justification");
		println!(
			"For header: Hash({:?}), Number({})",
			justification.commit.target_hash, justification.commit.target_number
		);

		let headers = grandpa_prover
			.query_finalized_parachain_headers_between(
				justification.commit.target_hash,
				client_state.latest_relay_hash,
			)
			.await
			.expect("Failed to fetch finalized parachain headers");

		let header_numbers = headers.iter().map(|h| h.number).collect();
		let proof = grandpa_prover
			.query_finalized_parachain_headers_with_proof(
				justification.commit.target_hash,
				client_state.latest_relay_hash,
				header_numbers,
			)
			.await
			.expect("Failed to fetch finalized parachain headers with proof");

		let header = Header {
			finality_proof: proof.finality_proof,
			parachain_headers: proof.parachain_headers,
		};
		let msg = MsgUpdateAnyClient {
			client_id: client_id.clone(),
			client_message: AnyClientMessage::Grandpa(ClientMessage::Header(header)),
			signer: signer.clone(),
		};

		// advance the chain
		ctx.advance_host_chain_height();
		let res = dispatch(&ctx, ClientMsg::UpdateClient(msg.clone()));

		match res {
			Ok(HandlerOutput { result, mut events, log }) => {
				assert_eq!(events.len(), 1);
				let event = events.pop().unwrap();
				assert!(
					matches!(event, IbcEvent::UpdateClient(ref e) if e.client_id() == &msg.client_id)
				);
				assert_eq!(event.height(), ctx.host_height());
				assert!(log.is_empty());
				ctx.store_client_result(result.clone()).unwrap();
				match result {
					Update(upd_res) => {
						assert_eq!(upd_res.client_id, client_id);
						assert!(!upd_res.client_state.is_frozen());
						assert_eq!(
							upd_res.client_state,
							ctx.latest_client_states(&client_id).clone()
						);
						// todo: assert the specific heights for new consensus states
						println!(
							"======== Successfully verified parachain headers for block number: {} ========",
							upd_res.client_state.latest_height(),
						);
					},
					_ => unreachable!("update handler result has incorrect type"),
				}
			},
			Err(e) => panic!("Unexpected error {:?}", e),
		}
	}
}
