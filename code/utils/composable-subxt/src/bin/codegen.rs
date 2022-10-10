//! Can be run withing `cargo run` and `cargo build`.
//! Well known chains are included as `mod` and have some default `url`.
//! Other chains will be generated and can be `include!` as Rust code in any other package.

use std::{fs, path::Path, process::Command};

use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
struct ComposableSubxt {
	/// COMPOSABLE_SUBXT_GENERATE=dali,rococo=ws://localhost:9933
	#[clap(short, long, env = "COMPOSABLE_SUBXT_GENERATE")]
	generate: Option<String>,
	#[clap(short, long = "out-dir", env = "OUT_DIR")]
	out_dir: String,
}

// TODO: consider next flow for code generation
// 1. nix download nodes https://github.com/paritytech/subxt/blob/e48f0e3b1de130779abb3dd092c86b845de81116/examples/examples/balance_transfer_with_params.rs#L9
// 2. ask node to produce scale
// 3. generate from it
// Good: can run all always in build.rs(of course only in nix shell) or in CI (no need node to run)
// Bad: cannot generate for node you have no access in nix
fn main() {
	let urls: HashMap<&str, &str> = [
		("dali", "ws://localhost:9988"), /* "wss://dali.devnets.composablefinance.ninja", can
		                                  * split out port into json/toml/nix and parse out from
		                                  * shared location too */
		("rococo", "ws://localhost:9944"), // "wss://rococo-rpc.polkadot.io"
		("picasso", "ws://localhost:9988"), // "wss://picasso.devnets.composablefinance.ninja"
		("kusama", "wss://kusama-rpc.polkadot.io:443"), /* can eat it from polkadotjs chain
		                                    * registry json too */
	]
	.into_iter()
	.collect();

	let env = ComposableSubxt::parse();

	if let Some(networks) = env.generate {
		fs::create_dir_all(Path::new(&env.out_dir).join("src/generated/")).unwrap();
		let networks = networks.split(",").map(|x| x.trim()).filter(|x| !x.is_empty());
		let _dest_path = Path::new(&env.out_dir).join("src/generated/mod.rs");
		for network in networks {
			let mut network = network.split("=");
			let name = network.next().expect("!x.is_empty()");
			let url = network
				.next()
				.or_else(|| urls.get(name).map(|x| *x))
				.unwrap_or("ws://localhost:9988");
			let subxt = Command::new("subxt").args(&["codegen", "--url", url]).output().unwrap();
			if !subxt.stderr.is_empty() {
				panic!("{}", String::from_utf8(subxt.stderr).unwrap());
			} else if !subxt.status.success() {
				panic!("{:?}", subxt.status);
			}
			let dest_path = Path::new(&env.out_dir).join(format!("src/generated/{}.rs", name));
			fs::write(dest_path, subxt.stdout).unwrap();
		}
	}

	println!("cargo:rerun-if-changed=build.rs");
}
