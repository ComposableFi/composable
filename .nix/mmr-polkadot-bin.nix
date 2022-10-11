{ pkgs, rust-nightly }:
with pkgs;
rustPlatform.buildRustPackage rec {
  # HACK: break the nix sandbox so we can build the runtimes. This
  # requires Nix to have `sandbox = relaxed` in its config.
  # We don't really care because polkadot is only used for local devnet.
  __noChroot = true;
  name = "mmr-polkadot-v${version}";
  version = "0.9.27";
  src = fetchFromGitHub {
    repo = "polkadot";
    owner = "ComposableFi";
    rev = "0898082540c42fb241c01fe500715369a33a80de";
    hash = "sha256-dymuSVQXzdZe8iiMm4ykVXPIjIZd2ZcAOK7TLDGOWcU=";
  };
  cargoSha256 = "sha256-u/hFRxt3OTMDwONGoJ5l7whC4atgpgIQx+pthe2CJXo=";
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
  meta = { mainProgram = "polkadot"; };
}
