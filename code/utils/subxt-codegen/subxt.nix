{ pkgs }:
let 
  cargo-toml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
  subxt-dep = cargo-toml.dependencies.subxt-codegen;
in pkgs.rustPlatform.buildRustPackage rec {
  pname = "subxt-cli";

  src = pkgs.fetchgit {
    url = subxt-dep.git;
    rev = subxt-dep.rev;
    sha256 = "sha256-iESFu1rpHVORyFV+g53eVADqUt6x6vB6rCuxEUq/MiM=";
  };

  cargoHash = "sha256-roQ1fAHT9pdzeaLjedStg+C8voDnj8gbo/R0zloXZlo=";
  cargoDepsName = pname;
}
