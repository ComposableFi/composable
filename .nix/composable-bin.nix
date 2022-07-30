{ pkgs, composable}:
pkgs.stdenv.mkDerivation rec {
    name = "composable-${composable.name}-${composable.version}";
    version = composable.version;
    src = pkgs.fetchurl {
      url = "https://storage.googleapis.com/composable-binaries/community-releases/${composable.name}/${name}.tar.gz";
      sha256 = composable.hash;
    };
    nativeBuildInputs = [
      pkgs.autoPatchelfHook
    ];
    buildInputs = [ pkgs.stdenv.cc.cc pkgs.zlib ];
    installPhase = ''
      tar -xvf $src
      mkdir -p $out/bin
      mv release/composable $out/bin
      mv doc $out
    '';
  }