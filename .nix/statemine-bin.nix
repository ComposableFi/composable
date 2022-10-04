{ pkgs, rust-nightly }:
with pkgs;
rustPlatform.buildRustPackage rec {
  __noChroot = true;
  name = "statemine-v${version}";
  version = "polkadot-v0.9.27";
  src = fetchFromGitHub {
    repo = "cumulus";
    owner = "paritytech";
    rev = "${version}";
    hash = "sha256-nbHdXv/93F6vHXWr/r9+AqvBBa5f9L6tmoIs8EEqiKM=";
  };
  cargoSha256 = "sha256-s5i+XqQH+FHCqG/L/gI9BWNKDMxxLwZ/tbeFD68hNew=";
  doCheck = false;
  buildInputs = [ openssl zstd ];
  nativeBuildInputs = [ rust-nightly clang pkg-config ]
    ++ lib.optional stdenv.isDarwin
    (with darwin.apple_sdk.frameworks; [ Security SystemConfiguration ]);
  LD_LIBRARY_PATH =
    lib.strings.makeLibraryPath [ stdenv.cc.cc.lib llvmPackages.libclang.lib ];
  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
  PROTOC = "${protobuf}/bin/protoc";
  ROCKSDB_LIB_DIR = "${rocksdb}/lib";
  meta = { mainProgram = "polkadot-parachain"; };
}
