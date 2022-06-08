{ pkgs }:
let 
  scraper = pkgs.rustPlatform.buildRustPackage rec {
    pname = "extrinsics-docs-scraper";
    version = "1.0.0";
    src = ../utils/extrinsics-docs-scraper;
    cargoSha256 = "q9D41wUeVOQ/pet950Omk09+Act7tM9wdXSZynvujuc=";
  };
in
pkgs.stdenv.mkDerivation {
  name = "composable-book";
  src = ./..;
  buildInputs = [ 
    pkgs.mdbook 
    pkgs.cargo
  ];
  dontUnpack = true;
  installPhase = ''
    echo pre_cargo
    cd $src
  	${scraper}/bin/extrinsics-docs-scraper --config-file-path=scraper.toml
    echo post_cargo
    mkdir -p $out/book
    cd $src/book
    mdbook build --dest-dir $out/book
  '';
}
