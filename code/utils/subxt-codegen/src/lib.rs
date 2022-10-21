#![allow(clippy::all)]

use codec::{Decode, Input};
use frame_metadata::RuntimeMetadataPrefixed;
use jsonrpsee::{
	async_client::ClientBuilder,
	client_transport::ws::{Uri, WsTransportClientBuilder},
	core::{client::ClientT, Error},
	rpc_params,
};
use std::{env, fs, path::Path};
use subxt_codegen::{CratePath, DerivesRegistry};

async fn fetch_metadata_ws(url: String) -> color_eyre::Result<Vec<u8>> {
	let (sender, receiver) = WsTransportClientBuilder::default()
		.build(url.parse::<Uri>().unwrap())
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
	let crate_path = CratePath::default();
	// add any derives you want here:
	let p = Vec::<String>::new()
		.iter()
		.map(|raw| syn::parse_str(raw))
		.collect::<Result<Vec<_>, _>>()?;
	let mut derives = DerivesRegistry::new(&crate_path);
	derives.extend_for_all(p.into_iter());

	let runtime_api = generator.generate_runtime(item_mod, derives, crate_path);
	Ok(format!("{}", runtime_api))
}

/// This will generate the relevant subxt code for the given rpc url and write it to
/// $OUT_DIR/filename.
pub async fn build_script(url: String, file_name: String) -> color_eyre::Result<()> {
	let metadata = fetch_metadata_ws(url).await?;
	let code = codegen(&mut &metadata[..])?;
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join(format!("{file_name}.rs"));
	fs::write(&dest_path, &code)?;
	Ok(())
}
