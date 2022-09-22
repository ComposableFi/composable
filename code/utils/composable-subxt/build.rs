use std::{env, fs, path::Path, process::Command};

use clap::{Parser, *};
use std::collections::HashMap;

#[derive(Parser, Debug)]
struct ComposableSubxt {
	#[clap(short, long, env = "COMPOSABLE_SUBXT_GENERATE")]
	generate: Option<String>,
	#[clap(short, long, env = "OUT_DIR")]
	out_dir: String,
}

fn main() {
	let urls: HashMap<&str, &str> = [
		("dali", "http:://localhost:9988"), // "https://dali.devnets.composablefinance.ninja"
		("rococo", "http:://localhost:9944"), // "https://rococo-rpc.polkadot.io"
		("picasso", "http:://localhost:9944"), // "https://picasso.devnets.composablefinance.ninja"
	]
	.into_iter()
	.collect();

	let args = ComposableSubxt::parse();
	println!("{:?}", args);

	if let Some(networks) = args.generate {
		fs::create_dir_all(Path::new(&out_dir).join("src/generated/")).unwrap();
		let networks = networks.split(",").map(|x| x.trim()).filter(|x| !x.is_empty());
		let dest_path = Path::new(&out_dir).join("src/generated/mod.rs");
		for network in networks {
			let mut network = network.split("=");
			let name = network.next().expect("!x.is_empty()");
			let url = network
				.next()
				.or_else(|| urls.get(name).map(|x| *x))
				.unwrap_or("http:://localhost:9988");
				let subxt = Command::new("subxt")
					.args(&["codegen", "--url", url])
					.output()
					.unwrap();
				let dest_path = Path::new(&out_dir).join(format!("src/generated/{}.rs", name));
				fs::write(dest_path, subxt.stdout).unwrap();			
				fs::write(&dest_path, format!("pub mod {};", name)).unwrap();
		}
	
	}

	//println!("cargo:rerun-if-changed=build.rs");
}
