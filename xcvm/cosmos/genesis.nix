# `junod` lacks ability of non interactive keys import/export/recover 
# (requires either phrase or passwords inputs), 
# neither has well defined way to prepare genesis state with some contracts built it.
# So we solve it by having some dummy folder copy paste after local run of client.
{ pkgs, junod }:
pkgs.stdenv.mkDerivation {
  name = "genesis";
  src = ./junod/.juno/keyring-file;
  installPhase = "true";
  buildPhase = ''
    # NOTE: consider use wrapper like for hasura
    mkdir --parents $out/data/.juno/ && cp -r $src/* $out/data/.juno/
    '';
}
