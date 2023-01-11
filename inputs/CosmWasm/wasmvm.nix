{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, lib, ... }: {
    packages = {
      libwasmvm = pkgs.rustPlatform.buildRustPackage {
        name = "libwasmvm";
        src = pkgs.fetchFromGitHub {
          owner = "CosmWasm";
          repo = "wasmvm";
          rev = "1afba37bfd0eda626d11ec760f51b16cb4254167";
          sha256 = "sha256-/wS+kZFu4RTO7Ump21dM9DpQBxTQ87BlCblE0JYMdiY=";
        };
        cargoHash = "sha256-WMfYsGtzOCxbhyoRRLtHg9H8ckPCByjsBSZCXimj/80=";
        sourceRoot = "source/libwasmvm";
        doCheck = false;
      };
    };
  };
}
