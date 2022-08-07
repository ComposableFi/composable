{ pkgs }:
pkgs.stdenv.mkDerivation {
  name = "junod";
  src = pkgs.fetchurl {
    url = "https://github.com/CosmosContracts/juno/releases/download/v8.0.0/junod";
    sha256 = "sha256-nJAYLXt2oXc9sr3dmZOgPEMZWfSXZ5T5sWjyej1Gr+Q=";
  };
  phases = [ "installPhase" "patchPhase" ];
  installPhase = ''
    mkdir --parents $out/bin
    cp $src $out/bin/junod
    chmod +x $out/bin/junod                                  
  '';
}