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
#![allow(clippy::all)]
#![deny(missing_docs)]

//! GRANDPA prover utilities

use crate::polkadot::api::runtime_types::polkadot_parachain::primitives::Id;
use anyhow::anyhow;
pub use beefy_prover;
use beefy_prover::helpers::{
	fetch_timestamp_extrinsic_with_proof, unsafe_arc_cast, TimeStampExtWithProof,
};
use codec::{Decode, Encode};
use finality_grandpa_rpc::GrandpaApiClient;
use jsonrpsee::{async_client::Client, ws_client::WsClientBuilder};
use primitives::{
	justification::GrandpaJustification, parachain_header_storage_key, ClientState, FinalityProof,
	ParachainHeaderProofs, ParachainHeadersWithFinalityProof,
};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::traits::{Header, Zero};
use std::{collections::BTreeMap, sync::Arc};
use subxt::{
	ext::{sp_core::hexdisplay::AsBytesRef, sp_runtime::traits::One},
	Config, OnlineClient,
};

/// Host function implementation for the verifier
pub mod host_functions;
/// Subxt generated code for the parachain
pub mod parachain;
/// Subxt generated code for the relay chain
pub mod polkadot;

/// Contains methods useful for proving parachain header finality using GRANDPA
pub struct GrandpaProver<T: Config> {
	/// Subxt client for the relay chain
	pub relay_client: OnlineClient<T>,
	/// Relay chain jsonrpsee client for typed rpc requests, which subxt lacks support for.
	pub relay_ws_client: Arc<Client>,
	/// Subxt client for the parachain
	pub para_client: OnlineClient<T>,
	/// Parachain jsonrpsee client for typed rpc requests, which subxt lacks support for.
	pub para_ws_client: Arc<Client>,
	/// ParaId of the associated parachain
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
	/// Initializes the parachain and relay chain clients given the ws urls.
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
			let key = polkadot::api::storage().grandpa().current_set_id();
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
		let latest_relay_height = u32::from(*header.number());
		let finalized_para_header =
			self.query_latest_finalized_parachain_header(latest_relay_height).await?;

		Ok(ClientState {
			current_authorities,
			current_set_id,
			latest_relay_height,
			latest_relay_hash,
			para_id: self.para_id,
			// we'll set this below
			latest_para_height: u32::from(*finalized_para_header.number()),
		})
	}

	/// Returns the latest finalized parachain header at the given finalized relay chain height.
	pub async fn query_latest_finalized_parachain_header(
		&self,
		latest_finalized_height: u32,
	) -> Result<T::Header, anyhow::Error> {
		let latest_finalized_hash = self
			.relay_client
			.rpc()
			.block_hash(Some(latest_finalized_height.into()))
			.await?
			.ok_or_else(|| anyhow!("Block hash not found for number: {latest_finalized_height}"))?;
		let key = polkadot::api::storage().paras().heads(&Id(self.para_id));
		let header = self
			.relay_client
			.storage()
			.fetch(&key, Some(latest_finalized_hash))
			.await?
			.ok_or_else(|| anyhow!("parachain header not found for para id: {}", self.para_id))?;
		let header = T::Header::decode(&mut &header.0[..])
			.map_err(|_| anyhow!("Failed to decode header"))?;

		Ok(header)
	}

	/// Returns the finality proof for the given parachain header numbers finalized by the given
	/// relay chain height.
	pub async fn query_finalized_parachain_headers_with_proof<H>(
		&self,
		client_state: &ClientState<T::Hash>,
		mut latest_finalized_height: u32,
		header_numbers: Vec<T::BlockNumber>,
	) -> Result<ParachainHeadersWithFinalityProof<H>, anyhow::Error>
	where
		H: Header,
		u32: From<H::Number>,
		H::Hash: From<T::Hash>,
		T::Hash: From<H::Hash>,
		T::BlockNumber: One,
	{
		let previous_para_hash = self
			.para_client
			.rpc()
			.block_hash(Some((client_state.latest_para_height + 1).into()))
			.await?
			.ok_or_else(|| anyhow!("Failed to fetch previous finalized parachain + 1 hash"))?;
		let address = parachain::api::storage().parachain_system().validation_data();
		let validation_data = self
			.para_client
			.storage()
			.fetch(&address, Some(previous_para_hash))
			.await
			.unwrap()
			.unwrap();
        dbg!(&validation_data.relay_parent_number);
		let previous_finalized_height =
			validation_data.relay_parent_number.min(client_state.latest_relay_height);

		let session_end = self.session_end_for_block(previous_finalized_height).await?;

        if client_state.latest_relay_height != session_end && latest_finalized_height > session_end {
			latest_finalized_height = session_end
		}

		let encoded = GrandpaApiClient::<JustificationNotification, H256, u32>::prove_finality(
			// we cast between the same type but different crate versions.
			&*unsafe {
				unsafe_arc_cast::<_, jsonrpsee_ws_client::WsClient>(self.relay_ws_client.clone())
			},
			latest_finalized_height,
		)
		.await?
		.ok_or_else(|| anyhow!("No justification found for block: {:?}", latest_finalized_height))?
		.0;
		let mut finality_proof = FinalityProof::<H>::decode(&mut &encoded[..])?;
		let mut justification =
			GrandpaJustification::<H>::decode(&mut &finality_proof.justification[..])?;
        justification.commit.precommits.drain(..);

        dbg!(&justification.commit);

		// sometimes we might get a justification for latest_finalized_height - 1, sigh
		let latest_finalized_height = u32::from(justification.commit.target_number);
        finality_proof.block = justification.commit.target_hash;

		let start = self
			.relay_client
			.rpc()
			.block_hash(Some(previous_finalized_height.into()))
			.await?
			.ok_or_else(|| anyhow!("Failed to fetch previous finalized hash + 1"))?;

		let latest_finalized_hash = self
			.relay_client
			.rpc()
			.block_hash(Some(latest_finalized_height.into()))
			.await?
			.ok_or_else(|| anyhow!("Failed to fetch previous finalized hash + 1"))?;

		let mut unknown_headers = vec![];
		for height in previous_finalized_height..=latest_finalized_height {
			let hash = self.relay_client.rpc().block_hash(Some(height.into())).await?.ok_or_else(
				|| anyhow!("Failed to fetch block has for height {previous_finalized_height}"),
			)?;

			let header = self
				.relay_client
				.rpc()
				.header(Some(hash))
				.await?
				.ok_or_else(|| anyhow!("Header with hash: {hash:?} not found!"))?;

			unknown_headers.push(H::decode(&mut &header.encode()[..])?);
		}

		// overwrite unknown headers
		finality_proof.unknown_headers = unknown_headers;

		// we are interested only in the blocks where our parachain header changes.
		let para_storage_key = parachain_header_storage_key(self.para_id);
		let keys = vec![para_storage_key.as_bytes_ref()];

		let change_set = self
			.relay_client
			.rpc()
			.query_storage(keys.clone(), start, Some(latest_finalized_hash))
			.await?;

		let mut parachain_headers = BTreeMap::<H::Hash, ParachainHeaderProofs>::default();

		for changes in change_set {
			let header = self
				.relay_client
				.rpc()
				.header(Some(changes.block))
				.await?
				.ok_or_else(|| anyhow!("block not found {:?}", changes.block))?;

			let parachain_header_bytes = {
				let key = polkadot::api::storage().paras().heads(&Id(self.para_id));
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

		Ok(ParachainHeadersWithFinalityProof { finality_proof, parachain_headers })
	}

	// Queries the block at which the epoch for the given block belongs to ends.
	async fn session_end_for_block(&self, block: u32) -> Result<u32, anyhow::Error> {
		let epoch_addr = polkadot::api::storage().babe().epoch_start();
		let block_hash = self.relay_client.rpc().block_hash(Some(block.into())).await?;
		let (previous_epoch_start, current_epoch_start) = self
			.relay_client
			.storage()
			.fetch(&epoch_addr, block_hash)
			.await?
			.ok_or_else(|| anyhow!("Failed to fetch epoch information"))?;
		Ok(current_epoch_start + (current_epoch_start - previous_epoch_start))
	}
}
