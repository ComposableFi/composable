# outputs /lib/wasmswap.wasm
{ crane, fetchFromGitHub }:
crane.buildPackage {
  name = "wasmswap";
  src = fetchFromGitHub {
    owner = "Wasmswap";
    repo = "wasmswap-contracts";
    rev = "cbd85f3a0a3636a273a1db136eacd26c6c50b7c8";
    sha256 = "sha256-CD0NHCXnM/9f8FiSFp9VXPfNuDMx3sYP4i6vdCOd6aE=";
  };
  cargoHash = "sha256-zj6QCY0KdZkDnhFZrrOus7L/0MeXxIrP6i7ZnpsaEC0=";
  doCheck = false;
  cargoExtraArgs = "--target wasm32-unknown-unknown";
  cargoCheckCommand = "true";
}
