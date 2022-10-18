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

use beefy_light_client_primitives::{
	error::BeefyClientError, MmrUpdateProof, ParachainsUpdateProof, SignatureWithAuthorityIndex,
	SignedCommitment,
};
use beefy_primitives::{
	known_payload_ids::MMR_ROOT_ID,
	mmr::{BeefyNextAuthoritySet, MmrLeaf},
	Payload,
};
use beefy_prover::{Crypto, Prover};
use futures::stream::StreamExt;
use pallet_mmr_primitives::Proof;
use serde_json::Value;
use sp_core::bytes::to_hex;
use subxt::{
	rpc::{rpc_params, Subscription},
	PolkadotConfig,
};

#[tokio::test]
async fn test_verify_mmr_with_proof() {
	let url = std::env::var("NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9944".to_string());
	let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(url).await.unwrap();
	let para_url = std::env::var("PARA_NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9188".to_string());
	let para_client =
		subxt::client::OnlineClient::<PolkadotConfig>::from_url(para_url).await.unwrap();

	let mut client_state = Prover::get_initial_client_state(Some(&client)).await;
	let subscription: Subscription<String> = client
		.rpc()
		.subscribe(
			"beefy_subscribeJustifications",
			rpc_params![],
			"beefy_unsubscribeJustifications",
		)
		.await
		.unwrap();

	let parachain_client =
		Prover { relay_client: client, para_client, beefy_activation_block: 0, para_id: 2000 };

	let mut subscription_stream = subscription.enumerate().take(100);
	while let Some((count, Ok(commitment))) = subscription_stream.next().await {
		let recv_commitment: sp_core::Bytes =
			serde_json::from_value(Value::String(commitment)).unwrap();
		let signed_commitment: beefy_primitives::SignedCommitment<
			u32,
			beefy_primitives::crypto::Signature,
		> = codec::Decode::decode(&mut &*recv_commitment).unwrap();

		match signed_commitment.commitment.validator_set_id {
			id if id < client_state.current_authorities.id => {
				// If validator set id of signed commitment is less than current validator set id we
				// have Then commitment is outdated and we skip it.
				println!(
                    "Skipping outdated commitment \n Received signed commitmment with validator_set_id: {:?}\n Current authority set id: {:?}\n Next authority set id: {:?}\n",
                    signed_commitment.commitment.validator_set_id, client_state.current_authorities.id, client_state.next_authorities.id
                );
				continue
			},
			_ => {},
		}

		println!("Received commitmment #{count} for: \n{:?}", signed_commitment.commitment);

		let mmr_update = parachain_client
			.fetch_mmr_update_proof_for(signed_commitment.clone())
			.await
			.unwrap();

		client_state =
			crate::verify_mmr_root_with_proof::<Crypto>(client_state.clone(), mmr_update.clone())
				.unwrap();

		let mmr_root_hash = signed_commitment.commitment.payload.get_raw(&MMR_ROOT_ID).unwrap();

		assert_eq!(client_state.mmr_root_hash.as_bytes(), &mmr_root_hash[..]);

		assert_eq!(client_state.latest_beefy_height, signed_commitment.commitment.block_number);

		assert_eq!(
			client_state.next_authorities,
			mmr_update.latest_mmr_leaf.beefy_next_authority_set
		);

		println!(
			"\nSuccessfully verifyed mmr for block number: {}\nmmr_root_hash: {}\n",
			client_state.latest_beefy_height,
			to_hex(&client_state.mmr_root_hash[..], false)
		);
	}
}

#[tokio::test]
async fn should_fail_with_incomplete_signature_threshold() {
	let mmr_update = MmrUpdateProof {
		signed_commitment: SignedCommitment {
			commitment: beefy_primitives::Commitment {
				payload: Payload::new(MMR_ROOT_ID, vec![0u8; 32]),
				block_number: Default::default(),
				validator_set_id: 3,
			},
			signatures: vec![SignatureWithAuthorityIndex { index: 0, signature: [0u8; 65] }; 2],
		},
		latest_mmr_leaf: MmrLeaf {
			version: Default::default(),
			parent_number_and_hash: (Default::default(), Default::default()),
			beefy_next_authority_set: BeefyNextAuthoritySet {
				id: 0,
				len: 0,
				root: Default::default(),
			},
			leaf_extra: Default::default(),
		},
		mmr_proof: Proof { leaf_index: 0, leaf_count: 0, items: vec![] },
		authority_proof: vec![],
	};

	let res = crate::verify_mmr_root_with_proof::<Crypto>(
		Prover::<PolkadotConfig>::get_initial_client_state(None).await,
		mmr_update,
	);

	match res {
		Err(BeefyClientError::IncompleteSignatureThreshold) => {},
		Err(err) =>
			panic!("Expected {:?}  found {:?}", BeefyClientError::IncompleteSignatureThreshold, err),
		Ok(val) =>
			panic!("Expected {:?}  found {:?}", BeefyClientError::IncompleteSignatureThreshold, val),
	}
}

#[tokio::test]
async fn should_fail_with_invalid_validator_set_id() {
	let mmr_update = MmrUpdateProof {
		signed_commitment: SignedCommitment {
			commitment: beefy_primitives::Commitment {
				payload: Payload::new(MMR_ROOT_ID, vec![0u8; 32]),
				block_number: Default::default(),
				validator_set_id: 3,
			},
			signatures: vec![SignatureWithAuthorityIndex { index: 0, signature: [0u8; 65] }; 5],
		},
		latest_mmr_leaf: MmrLeaf {
			version: Default::default(),
			parent_number_and_hash: (Default::default(), Default::default()),
			beefy_next_authority_set: BeefyNextAuthoritySet {
				id: 0,
				len: 0,
				root: Default::default(),
			},
			leaf_extra: Default::default(),
		},
		mmr_proof: Proof { leaf_index: 0, leaf_count: 0, items: vec![] },
		authority_proof: vec![],
	};

	let res = crate::verify_mmr_root_with_proof::<Crypto>(
		Prover::<PolkadotConfig>::get_initial_client_state(None).await,
		mmr_update,
	);
	match res {
		Err(BeefyClientError::AuthoritySetMismatch {
			current_set_id,
			next_set_id,
			commitment_set_id,
		}) if current_set_id == 0 && next_set_id == 1 && commitment_set_id == 3 => {},
		Err(err) => panic!(
			"Expected {:?}  found {:?}",
			BeefyClientError::AuthoritySetMismatch {
				current_set_id: 0,
				next_set_id: 1,
				commitment_set_id: 3
			},
			err
		),
		Ok(val) => panic!("Found {:?}", val),
	}
}

#[tokio::test]
async fn verify_parachain_headers() {
	let url = std::env::var("NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9944".to_string());
	let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(url).await.unwrap();
	let para_url = std::env::var("PARA_NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9188".to_string());
	let para_client =
		subxt::client::OnlineClient::<PolkadotConfig>::from_url(para_url).await.unwrap();

	let mut client_state = Prover::get_initial_client_state(Some(&client)).await;
	let subscription: Subscription<String> = client
		.rpc()
		.subscribe(
			"beefy_subscribeJustifications",
			rpc_params![],
			"beefy_unsubscribeJustifications",
		)
		.await
		.unwrap();

	println!("Waiting for parachain to start producing blocks");
	let block_sub = para_client.rpc().subscribe_blocks().await.unwrap();
	block_sub.take(2).collect::<Vec<_>>().await;
	println!("Parachain has started producing blocks");

	let parachain_client =
		Prover { relay_client: client, para_client, beefy_activation_block: 0, para_id: 2000 };

	let mut subscription_stream = subscription.enumerate().take(100);
	while let Some((count, Ok(commitment))) = subscription_stream.next().await {
		let recv_commitment: sp_core::Bytes =
			serde_json::from_value(Value::String(commitment)).unwrap();
		let signed_commitment: beefy_primitives::SignedCommitment<
			u32,
			beefy_primitives::crypto::Signature,
		> = codec::Decode::decode(&mut &*recv_commitment).unwrap();

		match signed_commitment.commitment.validator_set_id {
			id if id < client_state.current_authorities.id => {
				// If validator set id of signed commitment is less than current validator set id we
				// have Then commitment is outdated and we skip it.
				println!(
                    "Skipping outdated commitment \n Received signed commitmment with validator_set_id: {:?}\n Current authority set id: {:?}\n Next authority set id: {:?}\n",
                    signed_commitment.commitment.validator_set_id, client_state.current_authorities.id, client_state.next_authorities.id
                );
				continue
			},
			_ => {},
		}

		println!("Received commitmment #{count}: \n{:?}", signed_commitment.commitment);

		let block_number = signed_commitment.commitment.block_number;

		let headers = parachain_client
			.query_finalized_parachain_headers_at(block_number, client_state.latest_beefy_height)
			.await
			.unwrap();
		let (parachain_headers, batch_proof) = parachain_client
			.query_finalized_parachain_headers_with_proof(
				block_number,
				client_state.latest_beefy_height,
				headers.iter().map(|h| h.number).collect(),
			)
			.await
			.unwrap();

		let parachain_update_proof =
			ParachainsUpdateProof { parachain_headers, mmr_proof: batch_proof };

		let mmr_update =
			parachain_client.fetch_mmr_update_proof_for(signed_commitment).await.unwrap();

		client_state = crate::verify_mmr_root_with_proof::<Crypto>(client_state, mmr_update)
			.expect("verify_mmr_root_with_proof should not panic!");

		crate::verify_parachain_headers::<Crypto>(client_state.clone(), parachain_update_proof)
			.expect("verify_parachain_headers should not panic!");

		println!(
			"\nSuccessfully verified parachain headers for block number: {}\n",
			client_state.latest_beefy_height,
		);
	}
}
