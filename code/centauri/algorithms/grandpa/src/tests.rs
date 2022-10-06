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
	beefy_prover::helpers::unsafe_arc_cast, host_functions::HostFunctionsProvider, GrandpaProver,
};
use polkadot_core_primitives::Header;
use serde::{Deserialize, Serialize};
use sp_core::H256;
use std::mem::size_of_val;
use subxt::{ext::sp_runtime::traits::Header as _, PolkadotConfig};

pub type Justification = GrandpaJustification<Header>;

/// An encoded justification proving that the given header has been finalized
#[derive(Clone, Serialize, Deserialize)]
pub struct JustificationNotification(sp_core::Bytes);

#[tokio::test]
async fn follow_grandpa_justifications() {
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
		.skip_while(|h| futures::future::ready(h.number < 210))
		.take(1)
		.collect::<Vec<_>>()
		.await;
	println!("Grandpa proofs are now available");

	let mut subscription =
		GrandpaApiClient::<JustificationNotification, H256, u32>::subscribe_justifications(
			// we cast between the same type but different crate versions.
			&*unsafe {
				unsafe_arc_cast::<_, jsonrpsee_ws_client::WsClient>(prover.relay_ws_client.clone())
			},
		)
		.await
		.expect("Failed to subscribe to grandpa justifications")
		.take(100);

	let mut client_state = prover.initialize_client_state().await.unwrap();

	while let Some(Ok(JustificationNotification(sp_core::Bytes(justification)))) =
		subscription.next().await
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
				justification.commit.target_number,
				client_state.latest_relay_height,
			)
			.await
			.expect("Failed to fetch finalized parachain headers");
		let headers = match headers {
			Some(headers) => headers,
			None => continue,
		};

		let header_numbers = headers.iter().map(|h| *h.number()).collect();
		let maybe_proof = prover
			.query_finalized_parachain_headers_with_proof(
				justification.commit.target_number,
				client_state.latest_relay_height,
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
