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
	client_message::{
		BeefyHeader, ClientMessage, ParachainHeader as BeefyParachainHeader,
		ParachainHeadersWithProof,
	},
	client_state::{ClientState as BeefyClientState, ClientState},
	consensus_state::ConsensusState,
	mock::{
		AnyClientMessage, AnyClientState, AnyConsensusState, HostFunctionsManager, MockClientTypes,
	},
};
use beefy_light_client_primitives::{NodesUtils, PartialMmrLeaf};
use beefy_prover::{
	helpers::{fetch_timestamp_extrinsic_with_proof, TimeStampExtWithProof},
	runtime, Prover,
};
use codec::{Decode, Encode};
use futures::stream::StreamExt;
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
use json::Value;
use std::time::Duration;
use subxt::{
	rpc::{rpc_params, Subscription},
	PolkadotConfig,
};

#[tokio::test]
async fn test_continuous_update_of_beefy_client() {
	let client_id = ClientId::new(&ClientState::<HostFunctionsManager>::client_type(), 0).unwrap();

	let chain_start_height = Height::new(1, 11);

	let mut ctx = MockContext::<MockClientTypes>::new(
		ChainId::new("mockgaiaA".to_string(), 1),
		MockHostType::Mock,
		5,
		chain_start_height,
	);
	ctx.block_time = Duration::from_secs(600);

	let signer = get_dummy_account_id();

	let relay_client = {
		let url = std::env::var("NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9944".to_string());
		subxt::client::OnlineClient::<PolkadotConfig>::from_url(url).await.unwrap()
	};
	let para_client = {
		let para_url =
			std::env::var("PARA_NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9188".to_string());
		subxt::client::OnlineClient::<PolkadotConfig>::from_url(para_url).await.unwrap()
	};
	let client_wrapper = Prover {
		relay_client: relay_client.clone(),
		para_client: para_client.clone(),
		beefy_activation_block: 0,
		para_id: 2000,
	};

	println!("Waiting for parachain to start producing blocks");
	let block_sub = para_client.rpc().subscribe_blocks().await.unwrap();
	block_sub.take(2).collect::<Vec<_>>().await;
	println!("Parachain has started producing blocks");

	let (client_state, consensus_state) = loop {
		let beefy_state = client_wrapper.construct_beefy_client_state(0).await.unwrap();
		let subxt_block_number: subxt::rpc::BlockNumber = beefy_state.latest_beefy_height.into();
		let block_hash = client_wrapper
			.relay_client
			.rpc()
			.block_hash(Some(subxt_block_number))
			.await
			.unwrap();
		let head_data = {
			let key = runtime::api::storage().paras().heads(
				&runtime::api::runtime_types::polkadot_parachain::primitives::Id(
					client_wrapper.para_id,
				),
			);
			relay_client.storage().fetch(&key, block_hash).await.unwrap().unwrap()
		};
		let decoded_para_head = frame_support::sp_runtime::generic::Header::<
			u32,
			frame_support::sp_runtime::traits::BlakeTwo256,
		>::decode(&mut &*head_data.0)
		.unwrap();
		let block_number = decoded_para_head.number;
		let client_state = BeefyClientState {
			chain_id: ChainId::new("relay-chain".to_string(), 0),
			relay_chain: Default::default(),
			mmr_root_hash: beefy_state.mmr_root_hash,
			latest_beefy_height: beefy_state.latest_beefy_height,
			frozen_height: None,
			beefy_activation_block: beefy_state.beefy_activation_block,
			latest_para_height: block_number,
			para_id: client_wrapper.para_id,
			authority: beefy_state.current_authorities,
			next_authority_set: beefy_state.next_authorities,
			..Default::default()
		};
		// we can't use the genesis block to construct the initial state.
		if block_number == 0 {
			continue
		}
		let subxt_block_number: subxt::rpc::BlockNumber = block_number.into();
		let block_hash = client_wrapper
			.para_client
			.rpc()
			.block_hash(Some(subxt_block_number))
			.await
			.unwrap();

		let TimeStampExtWithProof { ext: timestamp_extrinsic, proof: extrinsic_proof } =
			fetch_timestamp_extrinsic_with_proof(&client_wrapper.para_client, block_hash)
				.await
				.unwrap();
		let parachain_header = BeefyParachainHeader {
			parachain_header: decoded_para_head,
			partial_mmr_leaf: PartialMmrLeaf {
				version: Default::default(),
				parent_number_and_hash: Default::default(),
				beefy_next_authority_set: Default::default(),
			},
			parachain_heads_proof: vec![],
			heads_leaf_index: 0,
			heads_total_count: 0,
			extrinsic_proof,
			timestamp_extrinsic,
		};

		let consensus_state = ConsensusState::from_header(parachain_header).unwrap();

		break (AnyClientState::Beefy(client_state), AnyConsensusState::Beefy(consensus_state))
	};

	let create_client =
		MsgCreateAnyClient { client_state, consensus_state, signer: signer.clone() };

	// Create the client
	let res = dispatch(&ctx, ClientMsg::CreateClient(create_client)).unwrap();
	ctx.store_client_result(res.result).unwrap();
	let subscription: Subscription<String> = relay_client
		.rpc()
		.subscribe(
			"beefy_subscribeJustifications",
			rpc_params![],
			"beefy_unsubscribeJustifications",
		)
		.await
		.unwrap();
	let mut subscription = subscription.take(100);

	while let Some(Ok(commitment)) = subscription.next().await {
		let recv_commitment: sp_core::Bytes = json::from_value(Value::String(commitment)).unwrap();
		let signed_commitment: beefy_primitives::SignedCommitment<
			u32,
			beefy_primitives::crypto::Signature,
		> = codec::Decode::decode(&mut &*recv_commitment).unwrap();
		let client_state: BeefyClientState<HostFunctionsManager> =
			match ctx.client_state(&client_id).unwrap() {
				AnyClientState::Beefy(client_state) => client_state,
				_ => panic!("unexpected client state"),
			};
		match signed_commitment.commitment.validator_set_id {
			id if id < client_state.authority.id => {
				// If validator set id of signed commitment is less than current validator set id we
				// have Then commitment is outdated and we skip it.
				println!(
                    "Skipping outdated commitment \n Received signed commitmment with validator_set_id: {:?}\n Current authority set id: {:?}\n Next authority set id: {:?}\n",
                    signed_commitment.commitment.validator_set_id, client_state.authority.id, client_state.next_authority_set.id
                );
				continue
			},
			_ => {},
		}

		println!(
			"Received signed commitmment for: {:?}",
			signed_commitment.commitment.block_number
		);

		let block_number = signed_commitment.commitment.block_number;
		let headers = client_wrapper
			.query_finalized_parachain_headers_at(block_number, client_state.latest_beefy_height)
			.await
			.unwrap();
		let (parachain_headers, batch_proof) = client_wrapper
			.query_finalized_parachain_headers_with_proof(
				block_number,
				client_state.latest_beefy_height,
				headers.iter().map(|h| h.number).collect(),
			)
			.await
			.unwrap();

		let mmr_update = client_wrapper
			.fetch_mmr_update_proof_for(signed_commitment.clone())
			.await
			.unwrap();

		let mmr_size = NodesUtils::new(batch_proof.leaf_count).size();

		let header = BeefyHeader {
			headers_with_proof: Some(ParachainHeadersWithProof {
				headers: parachain_headers
					.into_iter()
					.map(|header| BeefyParachainHeader {
						parachain_header: Decode::decode(&mut &*header.parachain_header.as_slice())
							.unwrap(),
						partial_mmr_leaf: header.partial_mmr_leaf,
						parachain_heads_proof: header.parachain_heads_proof,
						heads_leaf_index: header.heads_leaf_index,
						heads_total_count: header.heads_total_count,
						extrinsic_proof: header.extrinsic_proof,
						timestamp_extrinsic: header.timestamp_extrinsic,
					})
					.collect(),
				mmr_proofs: batch_proof.items.into_iter().map(|item| item.encode()).collect(),
				mmr_size,
			}),
			mmr_update_proof: Some(mmr_update),
		};

		let msg = MsgUpdateAnyClient {
			client_id: client_id.clone(),
			client_message: AnyClientMessage::Beefy(ClientMessage::Header(header)),
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
