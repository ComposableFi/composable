{ pkgs, rust-nightly }:
with pkgs;
rustPlatform.buildRustPackage rec {
  # HACK: break the nix sandbox so we can build the runtimes. This
  # requires Nix to have `sandbox = relaxed` in its config.
  # We don't really care because polkadot is only used for local devnet.
  name = "polkadot-v${version}";
  version = "0.9.27";

  src = fetchFromGitHub {
    repo = "polkadot";
    owner = "paritytech";
    rev = "v${version}";
    hash = "sha256-LEz3OrVgdFTCnVwzU8C6GeEougaOl2qo7jS9qIdMqAM=";
  };

  cargoSha256 = "sha256-6y+WK2k1rhqMxMjEJhzJ26WDMKZjXQ+q3ca2hbbeLvA=";
  meta = { mainProgram = "polkadot"; };

  # substrate-attrs-node-with-attrs
  __noChroot = true;
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
}
