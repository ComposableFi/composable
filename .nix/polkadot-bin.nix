{pkgs, polkadot}:
pkgs.stdenv.mkDerivation {
    name = "polkadot-${polkadot.version}";
    version = polkadot.version;
    src = fetchurl {
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
  }