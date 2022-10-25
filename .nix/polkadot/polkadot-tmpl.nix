{ pkgs, rust-nightly, name, version, repo, owner, rev, hash, cargoSha256 }:
with pkgs;
rustPlatform.buildRustPackage rec {
  # HACK: break the nix sandbox so we can build the runtimes. This
  # requires Nix to have `sandbox = relaxed` in its config.
  # We don't really care because polkadot is only used for local devnet.
  inherit name version cargoSha256;

  src = fetchFromGitHub { inherit repo owner rev hash; };

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
