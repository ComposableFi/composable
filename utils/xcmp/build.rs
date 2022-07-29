use std::{env, fs, path::Path, process::Command};

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();

	println!("TODO: call here subxt-cli as soon as got build of docker ci with it");
	// TODO: decide what to do with offline chains? should they block the build?
	fs::create_dir_all(Path::new(&out_dir).join("src/generated/")).unwrap();

	// let rococo_relay_chain = Command::new("subxt")
	// 	.args(&["codegen", "--url", "https://rococo-rpc.polkadot.io"])
	// 	.output()
	// 	.unwrap();
	// let dest_path = Path::new(&out_dir).join("src/generated/rococo_relay_chain.rs");
	// fs::write(dest_path, rococo_relay_chain.stdout).unwrap();

	// let dali_parachain = Command::new("subxt")
	// 	.args(&["codegen", "--url", "https://dali.devnets.composablefinance.ninja"])
	// 	.output()
	// 	.unwrap();
	// let dest_path = Path::new(&out_dir).join("src/generated/dali_parachain.rs");
	// fs::write(dest_path, dali_parachain.stdout).unwrap();

	let dest_path = Path::new(&out_dir).join("src/generated/mod.rs");
	fs::write(&dest_path, "pub mod rococo_relay_chain;pub mod dali_parachain;").unwrap();
	println!("cargo:rerun-if-changed=build.rs");
}
