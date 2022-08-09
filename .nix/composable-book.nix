{ pkgs, composable }:
pkgs.stdenv.mkDerivation {
  name = "composable-book";
  src = pkgs.fetchFromGitHub {
    owner = "ComposableFi";
    repo = "composable";
    rev = composable.version;
    sha256 = composable.revhash;
  };
  buildInputs = [ pkgs.mdbook ];
  phases = [ "installPhase" ];
  installPhase = ''
    mkdir -p $out/book
    cd $src/book
    mdbook build --dest-dir $out/book
  '';
}
