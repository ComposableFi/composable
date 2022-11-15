{ pkgs }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "subxt-cli";
  version = "0.24.0";

  src = pkgs.fetchCrate {
    inherit pname version;
    sha256 = "sha256-Dqbrv2rVZWXxWe+UJ10yZWhmaCVCU6QpfLjGu3xKE90=";
  };

  cargoHash = "sha256-biQEE8SRIRHPtpRyActrT4UXK81ObsrZjW9Vqd15dr8=";
  cargoDepsName = pname;
}
