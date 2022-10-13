# TODO: move to `parachains` folder
{ pkgs, rust-overlay }:

pkgs.stdenv.mkDerivation {
  name = "acala";
  src = pkgs.fetchgit {
    url = "https://github.com/AcalaNetwork/Acala.git";
    rev = "2cc84df5298611a63ddf3198f8f242a120a932f6";
    sha256 = "sha256-l0vQphfyE0FWISPbu3WvFMifM7mj072kXksntGAXS9k=";
    fetchSubmodules = true;
  };
  installPhase = ''
    mkdir --parents $out/bin && mv ./target/release/acala $out/bin
  '';
  __noChroot = true;
  doCheck = false;
  buildInputs = with pkgs; [ openssl ];
  nativeBuildInputs = with pkgs; [ clang git rust-overlay ];
  buildPhase = ''
    mkdir home
    export HOME=$PWD/home
    cargo build --locked --features with-all-runtime,rococo-native --profile release --workspace --exclude runtime-integration-tests --exclude e2e-tests --exclude test-service
  '';
  meta = { mainProgram = "acala"; };
  # TODO: moved these to some `cumulus based derivation`
  LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath
    (with pkgs; [ stdenv.cc.cc.lib llvmPackages.libclang.lib ]);
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  PROTOC = "${pkgs.protobuf}/bin/protoc";
  ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
}
