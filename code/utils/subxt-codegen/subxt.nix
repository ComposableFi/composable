{ pkgs, lockFilePath }:
let
  cargo-toml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
  subxt-dep = cargo-toml.dependencies.subxt-codegen;
in pkgs.rustPlatform.buildRustPackage rec {

  name = "subxt-cli";
  pname = "subxt-cli";

  src = pkgs.fetchgit {
    name = "subxt-src";
    url = subxt-dep.git;
    rev = subxt-dep.rev;
    sha256 = "sha256-C5BYvXA6jRx3Dhwp14se/LcqzfvADhg4pU6Ysmz37Sw=";
  };

  cargoHash = "sha256-roQ1fAHT9pdzeaLjedStg+C8voDnj8gbo/R0zloXZlo=";
  cargoDepsName = pname;
}
