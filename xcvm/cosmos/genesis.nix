{ pkgs } :
  pkgs.stdenv.mkDerivation {
    name = "genesis";
    src = ./.junod;
    installPhase = ''
    ls $src
    mkdir --parents $out/data/.junod && cp -r $src/.junod $out/data/.junod
    '';
}