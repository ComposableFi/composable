{ pkgs }:
pkgs.stdenv.mkDerivation {
  name = "composable-book";
  src = ./.;
  buildInputs = [ pkgs.mdbook ];
  dontUnpack = true;
  installPhase = ''
    mkdir -p $out/book
    cd $src
    mdbook build --dest-dir $out/book
  '';
}
