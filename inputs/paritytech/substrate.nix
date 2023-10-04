{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, lib, systemCommonRust, ... }:
    let
      debug = {
        # CARGO_LOG = "debug";
        # CARGO_NET_GIT_FETCH_WITH_CLI = "true";
        # CARGO_NET_RETRY = "true";
        # CARGO_HTTP_MULTIPLEXING = "false";
        # CARGO_HTTP_DEBUG = "true";
        # RUST_LOG = "debug";
      };
      subattrs = {
        LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath [
          pkgs.stdenv.cc.cc.lib
          pkgs.llvmPackages.libclang.lib
        ];
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        PROTOC = "${pkgs.protobuf}/bin/protoc";
        ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
        # forces Rust to use exact same git as CI runner/Nix fetcher/other tools
        CARGO_NET_GIT_FETCH_WITH_CLI = "true";
        CARGO_NET_RETRY = 3; # +1 on top of defaultw
      } // debug;

      subenv = {
        doCheck = false;
        buildInputs = with pkgs; [ openssl zstd protobuf zlib ];
        nativeBuildInputs = with pkgs;
        # yes, all these are in general needed, git not alwasy, but substrate checks git revision
          [ clang pkg-config self'.packages.rust-nightly git ]
          ++ systemCommonRust.darwin-deps;
        RUST_BACKTRACE = "full";
      } // subattrs;

      check-pallet = pkgs.writeShellApplication {
        name = "check-pallet";
        runtimeInputs = [ self'.packages.rust-nightly pkgs.protobuf ];
        text = ''
          cargo check --no-default-features --target wasm32-unknown-unknown --package "$1" 
          cargo check --no-default-features --target wasm32-unknown-unknown --package "$1" --features runtime-benchmarks
          cargo clippy --package "$1" -- --deny warnings --allow deprecated
        '';
      };
      check-std-wasm = pkgs.writeShellApplication {
        name = "check-std-wasm";
        runtimeInputs = [ self'.packages.rust-nightly pkgs.protobuf ];
        text = ''
          # we cannot use `check` because it does not not validates linker like `build`, errors happen there too
          FEATURES=""
          if [[ -n "''${2-}" ]]; then
            FEATURES="--features=$2"
          fi          
          # shellcheck disable=SC2086
          cargo build --no-default-features --target wasm32-unknown-unknown --package "$1" $FEATURES
          # shellcheck disable=SC2086
          cargo clippy --package "$1" $FEATURES -- --deny warnings --allow deprecated

          # shellcheck disable=SC2086
          cargo test --package "$1" $FEATURES
        '';
      };

      check-no-std = pkgs.writeShellApplication {
        name = "check-no-std";
        runtimeInputs = [ self'.packages.rust-nightly ];
        text = ''
          FEATURES=""
          if [[ -n "''${2-}" ]]; then
            FEATURES="--features=$2"
          fi          
          # shellcheck disable=SC2086
          cargo build --no-default-features --target thumbv7em-none-eabi --package "$1" $FEATURES
        '';
      };

      check-runtime = check-pallet;
    in {
      _module.args.subnix = rec { inherit subenv subattrs; };
      packages = {
        inherit check-pallet check-runtime check-std-wasm check-no-std;
      };
    };
}
