{pkgs, polkadot}:
pkgs.stdenv.mkDerivation {
    name = "polkadot-${polkadot.version}";
    version = polkadot.version;
    src = pkgs.fetchurl {
      url = "https://github.com/paritytech/polkadot/releases/download/v${polkadot.version}/polkadot";
      sha256 = polkadot.hash;
    };
    nativeBuildInputs = [
      pkgs.autoPatchelfHook
    ];
    buildInputs = [ pkgs.stdenv.cc.cc ];
    dontUnpack = true;
    installPhase = ''
      mkdir -p $out/bin
      cp $src $out/bin/polkadot
      chmod +x $out/bin/polkadot
    '';
    ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
    LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath [
      pkgs.stdenv.cc.cc.lib
      pkgs.llvmPackages.libclang.lib
    ];        
  }