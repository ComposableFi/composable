use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Composable")]
pub struct Opts {
	/// Path of the WASM file we will optimize.
	#[clap(short, long)]
	pub input: String,

	/// Output path of the optimized WASM file.
	#[clap(short, long)]
	pub output: String,
}

fn main() {
	let opts = Opts::parse();
	compact_wasm_file(Path::new(&opts.input), Path::new(&opts.output));
}

/// Compact the WASM binary using `wasm-gc` and compress it using zstd.
fn compact_wasm_file(input: &Path, output: &Path) {
	wasm_gc::garbage_collect_file(input, output).expect("Failed to compact generated WASM binary.");
	compress_wasm(output, output);
}

fn compress_wasm(input: &Path, output: &Path) {
	use sp_maybe_compressed_blob::CODE_BLOB_BOMB_LIMIT;
	let data = std::fs::read(input).expect("Failed to read WASM binary");
	match sp_maybe_compressed_blob::compress(&data, CODE_BLOB_BOMB_LIMIT) {
		Some(compressed) =>
			std::fs::write(output, &compressed[..]).expect("Failed to write WASM binary"),
		None => panic!("WASM bomb limit exceeded."),
	}
}
