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
	justification::GrandpaJustification, verify_parachain_headers_with_grandpa_finality_proof,
};
use codec::Decode;
use finality_grandpa_rpc::GrandpaApiClient;
use futures::StreamExt;
use grandpa_prover::{
	beefy_prover::helpers::unsafe_cast_to_jsonrpsee_client, host_functions::HostFunctionsProvider,
	runtime, GrandpaProver,
};
use jsonrpsee_core::client::Client;
use polkadot_core_primitives::Header;
use primitives::ClientState;
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_finality_grandpa::AuthorityList;
use std::{mem::size_of_val, sync::Arc};
use subxt::{ext::sp_runtime::traits::Header as _, rpc::rpc_params, PolkadotConfig};

pub type Justification = GrandpaJustification<Header>;

/// An encoded justification proving that the given header has been finalized
#[derive(Clone, Serialize, Deserialize)]
pub struct JustificationNotification(sp_core::Bytes);

#[tokio::test]
async fn follow_grandpa_justifications() {
	let relay_client = {
		let url = std::env::var("NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9944".to_string());
		subxt::client::OnlineClient::<PolkadotConfig>::from_url(url).await.unwrap()
	};

	let para_client = {
		let para_url =
			std::env::var("PARA_NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9188".to_string());
		subxt::client::OnlineClient::<PolkadotConfig>::from_url(para_url).await.unwrap()
	};

	println!("Waiting for grandpa proofs to become available");
	relay_client
		.rpc()
		.subscribe_blocks()
		.await
		.unwrap()
		.filter_map(|result| futures::future::ready(result.ok()))
		.skip_while(|h| futures::future::ready(*h.number() < 210))
		.take(1)
		.collect::<Vec<_>>()
		.await;
	println!("Grandpa proofs are now available");
	let client: Arc<Client> = unsafe { unsafe_cast_to_jsonrpsee_client(&relay_client) };
	let subscription =
		GrandpaApiClient::<JustificationNotification, H256, u32>::subscribe_justifications(
			&*client,
		)
		.await
		.expect("Failed to subscribe to grandpa justifications");

	let current_set_id = {
		let key = runtime::api::storage().grandpa().current_set_id();
		relay_client
			.storage()
			.fetch(&key, None)
			.await
			.unwrap()
			.expect("Failed to fetch current set id")
	};

	let current_authorities = {
		let bytes = relay_client
			.rpc()
			.request::<String>("state_call", rpc_params!("GrandpaApi_grandpa_authorities", "0x"))
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

	let prover = GrandpaProver { relay_client, para_client, para_id: 2000 };

	let mut client_state =
		ClientState { current_authorities, current_set_id, latest_relay_hash, para_id: 2000 };
	let mut subscription_stream = subscription.take(100);
	while let Some(Ok(JustificationNotification(sp_core::Bytes(justification)))) =
		subscription_stream.next().await
	{
		println!("========= New Justification =========");
		println!("justification size: {}kb", size_of_val(&*justification) / 1000);
		println!("current_set_id: {}", client_state.current_set_id);

		let justification =
			Justification::decode(&mut &justification[..]).expect("Failed to decode justification");
		println!(
			"For relay chain header: Hash({:?}), Number({})",
			justification.commit.target_hash, justification.commit.target_number
		);

		let headers = prover
			.query_finalized_parachain_headers_between(
				justification.commit.target_hash,
				client_state.latest_relay_hash,
			)
			.await
			.expect("Failed to fetch finalized parachain headers");

		let header_numbers = headers.iter().map(|h| *h.number()).collect();
		let maybe_proof = prover
			.query_finalized_parachain_headers_with_proof(
				justification.commit.target_hash,
				client_state.latest_relay_hash,
				header_numbers,
			)
			.await
			.expect("Failed to fetch finalized parachain headers with proof");

		if let Some(proof) = maybe_proof {
			client_state = verify_parachain_headers_with_grandpa_finality_proof::<
				Header,
				HostFunctionsProvider,
			>(client_state, proof)
			.expect("Failed to verify parachain headers with grandpa finality_proof");
			println!("========= Successfully verified grandpa justification =========");
		}
	}
}
