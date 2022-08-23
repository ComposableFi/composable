{ pkgs } :
  pkgs.stdenv.mkDerivation {
    name = "genesis";
    src = ./junod/.juno/keyring-file;
    installPhase = "true";
   buildPhase = ''    
    mkdir --parents $out/data/.juno/keyring-file && cp -r $src $out/data/.juno/keyring-file
    '';
}