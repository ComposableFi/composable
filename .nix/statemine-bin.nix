{ pkgs, rust-nightly }:
with pkgs;
let
  branch = "polkadot-v0.9.27";
  paritytech-cumulus = fetchFromGitHub {
    repo = "cumulus";
    owner = "paritytech";
    rev = branch;
    hash = "sha256-nbHdXv/93F6vHXWr/r9+AqvBBa5f9L6tmoIs8EEqiKM=";
  };
in pkgs.stdenv.mkDerivation (rec {
  name = "cumulus-v${version}";
  version = "0.9.27";
  pname = "polkadot-parachain";
  src = paritytech-cumulus;
  doCheck = false;
  __noChroot = true;
  buildInputs = with pkgs; [ openssl zstd ];
  configurePhase = ''
    	mkdir home
      export HOME=$PWD/home
      export WASM_TARGET_DIRECTORY=$PWD/home
  '';
  buildPhase = ''
    cargo build --release --locked --bin polkadot-parachain --no-default-features
  '';
  installPhase = ''
    mkdir --parents $out/bin && mv ./target/release/polkadot-parachain $out/bin
  '';
  # substrate-attrs-node-with-attrs
  nativeBuildInputs = with pkgs;
    [ rust-nightly clang pkg-config ] ++ lib.optional stdenv.isDarwin
    (with darwin.apple_sdk.frameworks; [ Security SystemConfiguration ]);
  LD_LIBRARY_PATH = lib.strings.makeLibraryPath
    (with pkgs; [ stdenv.cc.cc.lib llvmPackages.libclang.lib ]);
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  PROTOC = "${pkgs.protobuf}/bin/protoc";
  ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
  RUST_BACKTRACE = "full";
})
