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
    sha256 = "sha256-eRz1MtgGaJ3PZ6rNddAkE0OuGyP4TaUH+d29bO8x6Sg=";
  };
  cargoLock = {
    lockFile = lockFilePath;
    outputHashes = { "beefy-gadget-4.0.0-dev" = "sha256-roQ1fAHT9pdzeaLjedStg+C8voDnj8gbo/R0zloXZlo="; };

  };
  postPatch = ''
    echo ${lockFilePath}
    cp ${lockFilePath} Cargo.lock
  '';
  cargoHash = "sha256-roQ1fAHT9pdzeaLjedStg+C8voDnj8gbo/R0zloXZlo=";
  cargoDepsName = pname;
}
