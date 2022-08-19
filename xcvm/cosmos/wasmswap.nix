{ buildGoModule, rustPlatform, fetchFromGitHub, patchelf, system, lib }:
  rustPlatform.buildRustPackage {
    name = "wasmswap";
    src = fetchFromGitHub {
      owner = "Wasmswap";
      repo = "wasmswap-contracts";
      rev = "1afba37bfd0eda626d11ec710f51b16cb4254167";
      sha256 = "sha256-/wS+kZFu4RTO1Ump21dM9DpQBxTQ87BlCblE0JYMdiY=";
    };
    cargoHash = "sha256-WMfYsGtzOCxbhyoRRLtHg9H8ckPCByjsBSZCXimj/80=";
    doCheck = false;
  }