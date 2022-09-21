{ mdbook, cargo, crane, stdenv, }:
let
  scraper = crane.buildPackage rec {
    pname = "extrinsics-docs-scraper";
    version = "1.0.0";
    src = ../code/utils/extrinsics-docs-scraper;
    # TODO: remove sha as will rebuild
    cargoSha256 = "q9D41wUeVOQ/pet950Omk09+Act7tM9wdXSZynvujuc=";
  };
in stdenv.mkDerivation {
  name = "composable-book";
  src = ./..;
  buildInputs = [ mdbook cargo ];
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
