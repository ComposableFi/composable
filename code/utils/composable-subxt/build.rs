use std::{env, fs, path::Path, process::Command};

use clap::{Parser, *};


#[derive(Parser, Debug)]

struct ComposableSubxt {
	#[clap(short, long, env)]
    generate: String,
}

fn main() {
	
	let out_dir = env::var("OUT_DIR").unwrap();
	let args = ComposableSubxt::parse(); 
	let regenerate = env::var("COMPOSABLE_SUBXT_GENERATE").is_ok();

	
	fs::create_dir_all(Path::new(&out_dir).join("src/generated/")).unwrap();

	if regenerate {
		let rococo = Command::new("subxt")
			.args(&["codegen", "--url", "https://rococo-rpc.polkadot.io"])
			.output()
			.unwrap();
		let dest_path = Path::new(&out_dir).join("src/generated/rococo_relay_chain.rs");
		fs::write(dest_path, rococo.stdout).unwrap();

		let dali = Command::new("subxt")
			.args(&["codegen", "--url", "https://dali.devnets.composablefinance.ninja"])
			.output()
			.unwrap();
		let dest_path = Path::new(&out_dir).join("src/generated/dali.rs");
		fs::write(dest_path, dali.stdout).unwrap();
	}

	let dest_path = Path::new(&out_dir).join("src/generated/mod.rs");
	fs::write(&dest_path, "pub mod rococo;pub mod dali;").unwrap();
	println!("cargo:rerun-if-changed=build.rs");
}
