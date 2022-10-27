{ pkgs, }:
let
  cargo-toml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
  subxt-dep = cargo-toml.dependencies.subxt-codegen;
in pkgs.rustPlatform.buildRustPackage rec {

  name = "subxt-cli";
  pname = "subxt-cli";
  cargoBuildHook = ''
    cargo build --package subxt-cli
  '';
  src = pkgs.fetchgit {
    name = "subxt-src";
    url = subxt-dep.git;
    rev = subxt-dep.rev;
    sha256 = "sha256-C5BYvXA6jRx3Dhwp14se/LcqzfvADhg4pU6Ysmz37Sw=";
  };
  doCheck = false;
  cargoHash = "sha256-2PYLB59fI6gFrmj7UQetpu0f98C3IuFyGwWssN1E7q4=";
  cargoDepsName = pname;
  checkPhase = "true";
}
