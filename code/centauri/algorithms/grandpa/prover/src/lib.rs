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

use crate::runtime::api::runtime_types::polkadot_parachain::primitives::Id;
use anyhow::anyhow;
pub use beefy_prover;
use beefy_prover::helpers::{
	fetch_timestamp_extrinsic_with_proof, unsafe_arc_cast, TimeStampExtWithProof,
};
use codec::{Decode, Encode};
use finality_grandpa_rpc::GrandpaApiClient;
use jsonrpsee::{async_client::Client, ws_client::WsClientBuilder};
use primitives::{
	parachain_header_storage_key, ClientState, FinalityProof, ParachainHeaderProofs,
	ParachainHeadersWithFinalityProof,
};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::traits::{Header, Zero};
use std::{collections::BTreeMap, sync::Arc};
use subxt::{
	ext::{sp_core::hexdisplay::AsBytesRef, sp_runtime::traits::One},
	Config, OnlineClient,
};

pub mod host_functions;
pub mod runtime;

pub struct GrandpaProver<T: Config> {
	pub relay_client: OnlineClient<T>,
	pub relay_ws_client: Arc<Client>,
	pub para_client: OnlineClient<T>,
	pub para_ws_client: Arc<Client>,
	pub para_id: u32,
}

/// An encoded justification proving that the given header has been finalized
#[derive(Clone, Serialize, Deserialize)]
pub struct JustificationNotification(pub sp_core::Bytes);

impl<T> GrandpaProver<T>
where
	T: Config,
	T::BlockNumber: Ord + Zero,
	u32: From<T::BlockNumber>,
{
	pub async fn new(
		relay_ws_url: &str,
		para_ws_url: &str,
		para_id: u32,
	) -> Result<Self, anyhow::Error> {
		let relay_ws_client = Arc::new(WsClientBuilder::default().build(relay_ws_url).await?);
		let relay_client = OnlineClient::<T>::from_rpc_client(relay_ws_client.clone()).await?;
		let para_ws_client = Arc::new(WsClientBuilder::default().build(para_ws_url).await?);
		let para_client = OnlineClient::<T>::from_rpc_client(para_ws_client.clone()).await?;

		Ok(Self { relay_ws_client, relay_client, para_ws_client, para_client, para_id })
	}

	/// Construct the inital client state.
	pub async fn initialize_client_state(&self) -> Result<ClientState<T::Hash>, anyhow::Error> {
		use sp_finality_grandpa::AuthorityList;

		let current_set_id = {
			let key = runtime::api::storage().grandpa().current_set_id();
			self.relay_client
				.storage()
				.fetch(&key, None)
				.await
				.unwrap()
				.expect("Failed to fetch current set id")
		};

		let current_authorities = {
			let bytes = self
				.relay_client
				.rpc()
				.request::<String>(
					"state_call",
					subxt::rpc_params!("GrandpaApi_grandpa_authorities", "0x"),
				)
				.await
				.map(|res| hex::decode(&res[2..]))??;

			AuthorityList::decode(&mut &bytes[..]).expect("Failed to scale decode authorities")
		};

		let latest_relay_hash = self.relay_client.rpc().finalized_head().await?;

		let header = self
			.relay_client
			.rpc()
			.header(Some(latest_relay_hash))
			.await?
			.ok_or_else(|| anyhow!("Header not found for hash: {latest_relay_hash:?}"))?;

		Ok(ClientState {
			current_authorities,
			current_set_id,
			latest_relay_height: u32::from(*header.number()),
			latest_relay_hash,
			para_id: self.para_id,
		})
	}

	/// Returns the finalized parachain headers in between the given relay chain hashes.
	pub async fn query_finalized_parachain_headers_between(
		&self,
		latest_finalized_height: u32,
		previous_finalized_height: u32,
	) -> Result<Option<Vec<T::Header>>, anyhow::Error> {
		let latest_finalized_hash = self
			.relay_client
			.rpc()
			.block_hash(Some(latest_finalized_height.into()))
			.await?
			.ok_or_else(|| anyhow!("Block hash not found for number: {latest_finalized_height}"))?;
		let start_height = previous_finalized_height + 1;
		let previous_finalized_hash = self
			.relay_client
			.rpc()
			.block_hash(Some(start_height.into()))
			.await?
			.ok_or_else(|| anyhow!("Block hash not found for number: {start_height}"))?;
		let change_set = self
			.relay_client
			.rpc()
			.query_storage(
				// we are interested only in the blocks where our parachain header changes.
				vec![parachain_header_storage_key(self.para_id).as_bytes_ref()],
				previous_finalized_hash,
				Some(latest_finalized_hash),
			)
			.await?;

		if change_set.len() == 1 {
			return Ok(None)
		}

		let mut headers = vec![];
		for changes in change_set {
			let header =
				self.relay_client.rpc().header(Some(changes.block)).await?.ok_or_else(|| {
					anyhow!("[get_parachain_headers] block not found {:?}", changes.block)
				})?;

			let head = {
				let key = runtime::api::storage().paras().heads(&Id(self.para_id));
				self.relay_client
					.storage()
					.fetch(&key, Some(header.hash()))
					.await?
					.expect("Header exists in its own changeset; qed")
			};

			let para_header = T::Header::decode(&mut &head.0[..])
				.map_err(|_| anyhow!("Failed to decode header"))?;
			headers.push(para_header);
		}

		Ok(Some(headers))
	}

	/// Returns the finality proof for the given parachain header numbers in between the given relay
	/// chain hashes.
	pub async fn query_finalized_parachain_headers_with_proof<H>(
		&self,
		latest_finalized_height: u32,
		previous_finalized_height: u32,
		header_numbers: Vec<T::BlockNumber>,
	) -> Result<Option<ParachainHeadersWithFinalityProof<H>>, anyhow::Error>
	where
		H: Header,
		H::Hash: From<T::Hash>,
		T::BlockNumber: One,
	{
		let latest_finalized_hash = self
			.relay_client
			.rpc()
			.block_hash(Some(latest_finalized_height.into()))
			.await?
			.ok_or_else(|| anyhow!("Block hash not found for number: {latest_finalized_height}"))?;
		let previous_finalized_hash = self
			.relay_client
			.rpc()
			.block_hash(Some((previous_finalized_height).into()))
			.await?
			.ok_or_else(|| {
				anyhow!("Failed to fetch block has for height {previous_finalized_height}")
			})?;
		let latest_finalized_header = self
			.relay_client
			.rpc()
			.header(Some(latest_finalized_hash))
			.await?
			.ok_or_else(|| anyhow!("Header not found!"))?;

		let encoded = GrandpaApiClient::<JustificationNotification, H256, u32>::prove_finality(
			// we cast between the same type but different crate versions.
			&*unsafe {
				unsafe_arc_cast::<_, jsonrpsee_ws_client::WsClient>(self.relay_ws_client.clone())
			},
			u32::from(*latest_finalized_header.number()),
		)
		.await?
		.ok_or_else(|| {
			anyhow!("No justification found for block: {:?}", latest_finalized_header.hash())
		})?
		.0;
		let mut finality_proof = FinalityProof::<H>::decode(&mut &encoded[..])?;
		finality_proof.unknown_headers = {
			let mut unknown_headers = vec![H::decode(&mut &latest_finalized_header.encode()[..])?];
			let mut current = *latest_finalized_header.parent_hash();
			while current != previous_finalized_hash {
				let header = self
					.relay_client
					.rpc()
					.header(Some(current))
					.await?
					.ok_or_else(|| anyhow!("Header with hash: {current:?} not found!"))?;
				unknown_headers.push(H::decode(&mut &header.encode()[..])?);
				current = *header.parent_hash();
			}
			unknown_headers
		};

		// we are interested only in the blocks where our parachain header changes.
		let para_storage_key = parachain_header_storage_key(self.para_id);
		let keys = vec![para_storage_key.as_bytes_ref()];
		let start = self
			.relay_client
			.rpc()
			.block_hash(Some((previous_finalized_height + 1).into()))
			.await?
			.ok_or_else(|| anyhow!("Failed to fetch previous finalized hash + 1"))?;
		let change_set = self
			.relay_client
			.rpc()
			.query_storage(keys.clone(), start, Some(latest_finalized_header.hash()))
			.await?;

		let mut parachain_headers = BTreeMap::<H::Hash, ParachainHeaderProofs>::default();

		// no new parachain headers have been finalized
		if change_set.len() == 1 {
			return Ok(None)
		}

		for changes in change_set {
			let header = self
				.relay_client
				.rpc()
				.header(Some(changes.block))
				.await?
				.ok_or_else(|| anyhow!("block not found {:?}", changes.block))?;

			let parachain_header_bytes = {
				let key = runtime::api::storage().paras().heads(&Id(self.para_id));
				self.relay_client
					.storage()
					.fetch(&key, Some(header.hash()))
					.await?
					.expect("Header exists in its own changeset; qed")
					.0
			};

			let para_header: T::Header = Decode::decode(&mut &parachain_header_bytes[..])?;
			let para_block_number = *para_header.number();
			// skip genesis header or any unknown headers
			if para_block_number == Zero::zero() || !header_numbers.contains(&para_block_number) {
				continue
			}

			let state_proof = self
				.relay_client
				.rpc()
				.read_proof(keys.clone(), Some(header.hash()))
				.await?
				.proof
				.into_iter()
				.map(|p| p.0)
				.collect();

			let TimeStampExtWithProof { ext: extrinsic, proof: extrinsic_proof } =
				fetch_timestamp_extrinsic_with_proof(&self.para_client, Some(para_header.hash()))
					.await
					.map_err(|err| anyhow!("Error fetching timestamp with proof: {err:?}"))?;
			let proofs = ParachainHeaderProofs { state_proof, extrinsic, extrinsic_proof };
			parachain_headers.insert(header.hash().into(), proofs);
		}

		Ok(Some(ParachainHeadersWithFinalityProof { finality_proof, parachain_headers }))
	}
}
