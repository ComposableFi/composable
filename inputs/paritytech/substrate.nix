{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, lib, ... }:
    let

      subattrs = {
        LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath [
          pkgs.stdenv.cc.cc.lib
          pkgs.llvmPackages.libclang.lib
        ];
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        PROTOC = "${pkgs.protobuf}/bin/protoc";
        ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
      };

      subenv = {
        doCheck = false;
        buildInputs = with pkgs; [ openssl zstd ];
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
          cargo check --tests --features=std,runtime-benchmarks --package "$1"
          cargo check --no-default-features --target wasm32-unknown-unknown --package "$1"
          cargo clippy --package "$1" -- --deny warnings --allow deprecated
        '';
      };
    in {
      _module.args.subnix = rec { inherit subenv subattrs; };
      packages = { inherit check-pallet; };
    };
}
