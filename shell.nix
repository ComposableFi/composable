let
  mozillaOverlay =
    import (builtins.fetchGit {
      url = "https://github.com/mozilla/nixpkgs-mozilla.git";
      rev = "e1f7540fc0a8b989fb8cf701dc4fd7fc76bcf168";
    });
  nixpkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
  rust-nightly = with nixpkgs; ((rustChannelOf { date = "2022-04-18"; channel = "nightly"; }).rust.override {
    extensions = [ "rust-src" ];
    targets = [ "wasm32-unknown-unknown" ];
  });
in
with nixpkgs; pkgs.mkShell {
  buildInputs = [
    clang
    openssl.dev
    pkg-config
    rust-nightly
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
  ];

  RUST_SRC_PATH = "${rust-nightly}/lib/rustlib/src/rust/src";
  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
  PROTOC = "${protobuf}/bin/protoc";
  ROCKSDB_LIB_DIR = "${rocksdb}/lib";
}
