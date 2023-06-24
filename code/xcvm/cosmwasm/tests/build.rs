use sha2::Digest;
use std::io::Read;

/// Location of the compiled cw20-base contract.
const CW20_URL: &str =
	"https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm";

/// SHA256 of the compiled cw20-base contract.
const CW20_HASH: &[u8; 32] =
	b"\x9c\x29\x5a\x93\xd5\x03\x3c\xb7\x40\x2d\x59\xcd\xed\xef\x72\x54\xa6\x9f\x9c\xa5\x0e\xed\x10\x18\x0c\x53\xbb\xb3\x1c\x00\x5e\x92";

/// Downloads `cw20_base.wasm` contract and saves it in `$OUT_DIR`.
///
/// Panics if file cannot be downloaded or its SHA256 hash doesn’t match
/// expected hash.
fn main() {
	// When building on CI inside of NIX, don’t download the contract file
	// since HTTP is blocked.
	if std::env::var_os("NIX_BUILD_RS_OUT_DIR").is_some() {
		return
	}

	// Otherwise, download the file, verify it’s what we expect and store it in
	// out directory.
	let out_dir = std::env::var_os("OUT_DIR").unwrap();
	let mut out_file = std::path::PathBuf::from(out_dir);
	out_file.push("cw20_base.wasm");

	let resp = match ureq::get(CW20_URL).call() {
		Ok(x) => x,
		Err(err) => panic!("{CW20_URL}: {err}"),
	};
	let mut data = Vec::with_capacity(500_000);
	if let Err(err) = resp.into_reader().take(500_000).read_to_end(&mut data) {
		panic!("{CW20_URL}: {err}");
	}

	if CW20_HASH[..] != sha2::Sha256::digest(data.as_slice())[..] {
		panic!("{CW20_URL}: content integrity check failed")
	}

	if let Err(err) = std::fs::write(out_file.as_path(), data.as_slice()) {
		panic!("{}: {err}", out_file.display());
	}
}
