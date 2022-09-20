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

use codec::{Decode, Input};
use frame_metadata::RuntimeMetadataPrefixed;
use jsonrpsee::{
	async_client::ClientBuilder,
	client_transport::ws::{Uri, WsTransportClientBuilder},
	core::{client::ClientT, Error},
	rpc_params,
};
use std::{env, fs, path::Path};
use subxt_codegen::DerivesRegistry;

// We need this build script to rebuild the runtime metadata from a live node
// Since we have exported functions that depends on the having the latest relay chain metadata
// Build is disabled by default and only enabled when the env variable PROVER_ENABLED is set to 1

async fn fetch_metadata_ws() -> color_eyre::Result<Vec<u8>> {
	let node_var = env::var("NODE_ENDPOINT").unwrap_or("ws://127.0.0.1:9944".to_string());
	let (sender, receiver) = WsTransportClientBuilder::default()
		.build(node_var.parse::<Uri>().unwrap())
		.await
		.map_err(|e| Error::Transport(e.into()))?;

	let client = ClientBuilder::default()
		.max_notifs_per_subscription(4096)
		.build_with_tokio(sender, receiver);

	let metadata: String = client.request("state_getMetadata", rpc_params![]).await?;
	Ok(hex::decode(metadata.trim_start_matches("0x"))?)
}

fn codegen<I: Input>(encoded: &mut I) -> color_eyre::Result<String> {
	let metadata = <RuntimeMetadataPrefixed as Decode>::decode(encoded)?;
	let generator = subxt_codegen::RuntimeGenerator::new(metadata);
	let item_mod = syn::parse_quote!(
		pub mod api {}
	);

	// add any derives you want here:
	let p = Vec::<String>::new()
		.iter()
		.map(|raw| syn::parse_str(raw))
		.collect::<Result<Vec<_>, _>>()?;
	let mut derives = DerivesRegistry::default();
	derives.extend_for_all(p.into_iter());

	let runtime_api = generator.generate_runtime(item_mod, derives);
	Ok(format!("{}", runtime_api))
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	let metadata = fetch_metadata_ws().await?;
	let code = codegen(&mut &metadata[..])?;
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("runtime.rs");
	fs::write(&dest_path, &code)?;

	Ok(())
}
