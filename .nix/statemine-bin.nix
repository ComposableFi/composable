{ pkgs, rust-nightly, crane-nightly }:
with pkgs;
let
  branch = "polkadot-v0.9.27";
  paritytech-cumulus = fetchFromGitHub {
    repo = "cumulus";
    owner = "paritytech";
    rev = branch;
    hash = "sha256-nbHdXv/93F6vHXWr/r9+AqvBBa5f9L6tmoIs8EEqiKM=";
  };
  substrate-build-attrs = {
    buildInputs = [ openssl zstd ];
    nativeBuildInputs = [ clang openssl pkg-config ]
      ++ lib.optional stdenv.isDarwin
      (with darwin.apple_sdk.frameworks; [ Security SystemConfiguration ]);
    LD_LIBRARY_PATH = lib.strings.makeLibraryPath [
      stdenv.cc.cc.lib
      llvmPackages.libclang.lib
    ];
    LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
    PROTOC = "${protobuf}/bin/protoc";
    ROCKSDB_LIB_DIR = "${rocksdb}/lib";
  };
in with pkgs;
# rust-nightly.buildPackage (substrate-build-attrs // rec {
#   __noChroot = true;
#   name = "statemine-v${version}";
#   version = "0.9.27";
#   src = paritytech-cumulus;
#   cargoSha256 = "sha256-s3i+XqQH+FHCqG/L/gI9BWNKDMxxLwZ/tbeFD68hNew=";
#   doCheck = false;
#   meta = { mainProgram = "polkadot-parachain"; };
#   cargoBuildCommand = "cargo build --release --locked --bin polkadot-parachain";
#   installPhase = ''
#     mkdir -p $out/bin
#     cp target/release/polkadot-parachain $out/bin/polkadot-parachain
#   '';
# })
# rustPlatform.buildRustPackage (rec {
#   name = "cumulus-v${version}";
#   version = "0.9.27";
#   pname = "polkadot-parachain";
#   src = paritytech-cumulus;
#   cargoDepsName = pname;
#   cargoHash = "sha256-hL+cIQJXPzZjvIxoL0EJkrss9Q+NUBlys1cnH9x7DE0=";
#   meta = { mainProgram = "polkadot-parachain"; };
#   buildNoDefaultFeatures = true;
#   doCheck = false;
#   __noChroot = true;
#   buildInputs = [ openssl zstd ];
#   # configurePhase = ''
#   #   	mkdir home
#   #     export HOME=$PWD/home	
#   #     export WASM_TARGET_DIRECTORY=$PWD/home
#   # '';
#   # buildPhase = ''
#   # cargo build --release --locked --bin ${pname} --no-default-features
#   # '';
#   cargoDeps = rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };
#   nativeBuildInputs = [ rust-nightly clang pkg-config ]
#     ++ lib.optional stdenv.isDarwin
#     (with darwin.apple_sdk.frameworks; [ Security SystemConfiguration ]);
#   LD_LIBRARY_PATH =
#     lib.strings.makeLibraryPath [ stdenv.cc.cc.lib llvmPackages.libclang.lib ];
#   LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
#   PROTOC = "${protobuf}/bin/protoc";
#   ROCKSDB_LIB_DIR = "${rocksdb}/lib";
#   RUST_BACKTRACE = "full";
# })

# 1. rustPlatform.buildRustPackage and ipetkov/crane.buildPackage fail to build things
# 2. tried variations to setup deps - did not worked.
# 3. first fails with:
# error: builder for '/nix/store/w8ly900j5cp0sgpd2zha5fiq9ylz4k0p-cumulus-v0.9.27.drv' failed with exit code 101;
#  last 10 log lines:
#  >   --- stderr
#  >   thread 'main' panicked at '`cargo metadata` can not fail on project `Cargo.toml`; qed: CargoMetadata { stderr: "    Blocking waiting for file lock on package cache\nerror: failed to get `clap` as a dependency of package `cumulus-client-cli v0.1.0 (/tmp/nix-build-cumulus-v0.9.27.drv-0/source/client/cli)`\n\nCaused by:\n  failed to load source for dependency `clap`\n\nCaused by:\n  Unable to update registry `crates-io`\n\nCaused by:\n  failed to update replaced source registry `crates-io`\n\nCaused by:\n  failed to read root of directory source: /tmp/nix-build-cumulus-v0.9.27.drv-0/cumulus-v0.9.27-vendor.tar.gz/@vendor@\n\nCaused by:\n  No such file or directory (os error 2)\n" }', /tmp/nix-build-cumulus-v0.9.27.drv-0/cumulus-v0.9.27-vendor.tar.gz/substrate-wasm-builder/src/wasm_project.rs:96:10
#  >   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
#  > error: failed to run custom build command for `polkadot-test-runtime v0.9.27 (https://github.com/paritytech/polkadot?branch=release-v0.9.27#b017bad5)`
#  >
#  > Caused by:
#  >   process didn't exit successfully: `/tmp/nix-build-cumulus-v0.9.27.drv-0/source/target/release/build/polkadot-test-runtime-a6006bdbb11b871a/build-script-build` (exit status: 101)
#  >   --- stderr
#  >   thread 'main' panicked at '`cargo metadata` can not fail on project `Cargo.toml`; qed: CargoMetadata { stderr: "error: failed to get `clap` as a dependency of package `cumulus-client-cli v0.1.0 (/tmp/nix-build-cumulus-v0.9.27.drv-0/source/client/cli)`\n\nCaused by:\n  failed to load source for dependency `clap`\n\nCaused by:\n  Unable to update registry `crates-io`\n\nCaused by:\n  failed to update replaced source registry `crates-io`\n\nCaused by:\n  failed to read root of directory source: /tmp/nix-build-cumulus-v0.9.27.drv-0/cumulus-v0.9.27-vendor.tar.gz/@vendor@\n\nCaused by:\n  No such file or directory (os error 2)\n" }', /tmp/nix-build-cumulus-v0.9.27.drv-0/cumulus-v0.9.27-vendor.tar.gz/substrate-wasm-builder/src/wasm_project.rs:96:10
#  >   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
# 4. second  like this  https://github.com/ipetkov/crane/issues/83 
# error: builder for '/nix/store/88hhfs6azap80x7klqi2891vhk1qvizy-cumulus-v0.9.27.drv' failed with exit code 101;
#        last 10 log lines:
#        > configuring
#        > no configure script, doing nothing
#        > building
#        > error: failed to get `clap` as a dependency of package `cumulus-client-cli v0.1.0 (/tmp/nix-build-cumulus-v0.9.27.drv-4/source/client/cli)`
#        >
#        > Caused by:
#        >   failed to create directory `/homeless-shelter/.cargo/registry/index/github.com-1ecc6299db9ec823`
#        >
#        > Caused by:
# 5. plauing with phases and configs gives other errors hard to test (fails only after 10 minutes of run)
stdenv.mkDerivation (rec {
  name = "cumulus-v${version}";
  version = "0.9.27";
  pname = "polkadot-parachain";
  src = paritytech-cumulus;
  doCheck = false;
  __noChroot = true;
  buildInputs = [ openssl zstd ];
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
  nativeBuildInputs = [ rust-nightly clang pkg-config ]
    ++ lib.optional stdenv.isDarwin
    (with darwin.apple_sdk.frameworks; [ Security SystemConfiguration ]);
  LD_LIBRARY_PATH =
    lib.strings.makeLibraryPath [ stdenv.cc.cc.lib llvmPackages.libclang.lib ];
  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
  PROTOC = "${protobuf}/bin/protoc";
  ROCKSDB_LIB_DIR = "${rocksdb}/lib";
  RUST_BACKTRACE = "full";
})
