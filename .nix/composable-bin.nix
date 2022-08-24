# TODO: move to `parachains` folder
{ pkgs, composable }:
pkgs.stdenv.mkDerivation rec {
  name = "composable-${composable.name}-${composable.version}";
  version = composable.version;
  src = pkgs.fetchurl {
    # TODO: remove - use cachix for builds - or pure buildsfrom repo
    url =
      "https://storage.googleapis.com/composable-binaries/community-releases/${composable.name}/${name}.tar.gz";
    sha256 = composable.hash;
  };
  nativeBuildInputs = [ pkgs.autoPatchelfHook ];
  autoPatchelfIgnoreMissingDeps = [ "*" ];
  buildInputs = [ pkgs.stdenv.cc.cc pkgs.zlib pkgs.rocksdb ];
  installPhase = ''
    tar -xvf $src
    mkdir -p $out/bin
    mv release/composable $out/bin
    mv doc $out
  '';
  ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
  LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath [
    pkgs.stdenv.cc.cc.lib
    pkgs.llvmPackages.libclang.lib
    pkgs.rocksdb
  ];
}
