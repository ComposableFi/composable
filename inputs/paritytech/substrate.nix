{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, lib, ... }:
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
        CARGO_NET_RETRY = 3; # +1 on top of default
      } // debug;

      subenv = {
        doCheck = false;
        buildInputs = with pkgs; [ openssl zstd protobuf ];
        nativeBuildInputs = with pkgs;
          [ clang pkg-config self'.packages.rust-nightly ]
          ++ lib.optional stdenv.isDarwin
          (with pkgs.darwin.apple_sdk.frameworks; [
            Security
            SystemConfiguration
          ]);
        RUST_BACKTRACE = "full";
      } // subattrs;
      check-pallet = pkgs.writeShellApplication {
        name = "check-pallet";
        runtimeInputs = [ self'.packages.rust-nightly ];
        text = ''
          EXTRA_FEATURES=""
          if [[ -n "''${2-}" ]]; then
            EXTRA_FEATURES=",$2"
          fi
          cargo check --no-default-features --target wasm32-unknown-unknown --package "$1" 
          cargo check --tests --features=std,runtime-benchmarks --package "$1"
          cargo clippy --package "$1" -- --deny warnings --allow deprecated
          cargo test --features=std,runtime-benchmarks"$EXTRA_FEATURES" --package "$1"
        '';
      };
      check-runtime = check-pallet;
    in {
      _module.args.subnix = rec { inherit subenv subattrs; };
      packages = { inherit check-pallet check-runtime; };
    };
}
