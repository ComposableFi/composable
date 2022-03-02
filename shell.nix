with import <nixpkgs> {}; mkShell {
  buildInputs = [
    stdenv.cc.cc
    openssl.dev
    clang
    pkg-config
  ];
  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
  PROTOC = "${protobuf}/bin/protoc";
  ROCKSDB_LIB_DIR = "${rocksdb}/lib";
}
