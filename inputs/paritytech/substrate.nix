{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, lib, ... }:
    let
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
        LD_LIBRARY_PATH = lib.strings.makeLibraryPath [
          pkgs.stdenv.cc.cc.lib
          pkgs.llvmPackages.libclang.lib
        ];
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        PROTOC = "${pkgs.protobuf}/bin/protoc";
        ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
        RUST_BACKTRACE = "full";
      };
    in { _module.args.subTools = rec { inherit subenv; }; };
}
