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
use beefy_prover::helpers::{
	fetch_timestamp_extrinsic_with_proof, unsafe_arc_cast, TimeStampExtWithProof,
};
use codec::Decode;
use finality_grandpa_rpc::GrandpaApiClient;
use futures::stream::StreamExt;
use grandpa_client_primitives::{
	justification::GrandpaJustification, parachain_header_storage_key, ParachainHeaderProofs,
};
use grandpa_prover::{polkadot, GrandpaProver, JustificationNotification};
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
use std::{mem::size_of_val, time::Duration};
use subxt::{ext::sp_core::hexdisplay::AsBytesRef, PolkadotConfig};

pub type Justification = GrandpaJustification<RelayChainHeader>;

#[tokio::test]
async fn test_continuous_update_of_grandpa_client() {
    env_logger::builder()
    .filter_module("grandpa", log::LevelFilter::Trace)
    .format_module_path(false)
    .init();
    
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

	let relay_ws_url = std::env::var("NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9944".to_string());
	let para_ws_url =
		std::env::var("PARA_NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9188".to_string());

	let prover = GrandpaProver::<PolkadotConfig>::new(&relay_ws_url, &para_ws_url, 2000)
		.await
		.unwrap();

	println!("Waiting for grandpa proofs to become available");
	prover
		.relay_client
		.rpc()
		.subscribe_blocks()
		.await
		.unwrap()
		.filter_map(|result| futures::future::ready(result.ok()))
		.skip_while(|h| futures::future::ready(h.number < 90))
		.take(1)
		.collect::<Vec<_>>()
		.await;
	println!("Grandpa proofs are now available");

	let (client_state, consensus_state) = loop {
		let client_state = prover.initialize_client_state().await.unwrap();

		let latest_relay_header = prover
			.relay_client
			.rpc()
			.header(Some(client_state.latest_relay_hash))
			.await
			.expect("Failed to fetch finalized header")
			.expect("Failed to fetch finalized header");

		let head_data = {
			let key = polkadot::api::storage().paras().heads(
				&polkadot::api::runtime_types::polkadot_parachain::primitives::Id(prover.para_id),
			);
			prover
				.relay_client
				.storage()
				.fetch(&key, Some(client_state.latest_relay_hash))
				.await
				.unwrap()
				.unwrap()
		};
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
			latest_relay_hash: client_state.latest_relay_hash,
			latest_relay_height: latest_relay_header.number,
			frozen_height: None,
			latest_para_height: decoded_para_head.number,
			para_id: prover.para_id,
			current_set_id: client_state.current_set_id,
			current_authorities: client_state.current_authorities,
			_phantom: Default::default(),
		};
		let subxt_block_number: subxt::rpc::BlockNumber = decoded_para_head.number.into();
		let block_hash =
			prover.para_client.rpc().block_hash(Some(subxt_block_number)).await.unwrap();

		let TimeStampExtWithProof { ext: timestamp_extrinsic, proof: extrinsic_proof } =
			fetch_timestamp_extrinsic_with_proof(&prover.para_client, block_hash)
				.await
				.unwrap();
		let state_proof = prover
			.relay_client
			.rpc()
			.read_proof(
				vec![parachain_header_storage_key(prover.para_id).as_bytes_ref()],
				Some(client_state.latest_relay_hash),
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
			prover.para_id,
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
			&*unsafe {
				unsafe_arc_cast::<_, jsonrpsee_ws_client::WsClient>(prover.relay_ws_client.clone())
			},
		)
		.await
		.expect("Failed to subscribe to grandpa justifications");
	let mut subscription = subscription.take(100);

	while let Some(Ok(JustificationNotification(sp_core::Bytes(justification_bytes)))) =
		subscription.next().await
	{
		let client_state: ClientState<HostFunctionsManager> =
			match ctx.client_state(&client_id).unwrap() {
				AnyClientState::Grandpa(client_state) => client_state,
				_ => panic!("unexpected client state"),
			};

		let justification = Justification::decode(&mut &justification_bytes[..])
			.expect("Failed to decode justification");

		if justification.commit.target_number <= client_state.latest_relay_height {
			println!(
				"skipping outdated commit: {}, with latest relay height: {}",
				justification.commit.target_number, client_state.latest_relay_height
			);
			continue
		}

		let finalized_para_header = prover
			.query_latest_finalized_parachain_header(justification.commit.target_number)
			.await
			.expect("Failed to fetch finalized parachain headers");
		// notice the inclusive range
		let header_numbers = ((client_state.latest_para_height + 1)..=finalized_para_header.number)
			.collect::<Vec<_>>();

		if header_numbers.len() == 0 {
			continue
		}

		let proof = prover
			.query_finalized_parachain_headers_with_proof(
				&(client_state.clone().into()),
				justification.commit.target_number,
				header_numbers.clone(),
			)
			.await
			.expect("Failed to fetch finalized parachain headers with proof");
		println!("========= New Justification =========");
		println!("justification size: {}kb", size_of_val(&*justification_bytes) / 1000);
		println!("current_set_id: {}", client_state.current_set_id);
		println!(
			"For relay chain header: Hash({:?}), Number({})",
			justification.commit.target_hash, justification.commit.target_number
		);

		let header = Header {
			finality_proof: proof.finality_proof,
			parachain_headers: proof.parachain_headers.clone(),
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
						for height in header_numbers {
							let cs = ctx
								.consensus_state(
									&client_id,
									Height::new(prover.para_id as u64, height as u64),
								)
								.ok();
							dbg!((height, cs.is_some()));
						}
						println!(
							"======== Successfully updated parachain client to height: {} ========",
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
