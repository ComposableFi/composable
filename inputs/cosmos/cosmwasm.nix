{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = rec {
      cosmwasm-check = pkgs.rustPlatform.buildRustPackage rec {
        pname = "cosmwasm-check";
        version = "0948715950151579aaba487944b630332d83e215";

        src = pkgs.fetchFromGitHub {
          owner = "CosmWasm";
          repo = "cosmwasm";
          rev = version;
          sha256 = "sha256-/Bsq+QG/teLuAwCqpP1uAeMyhyCcy+2aJZ0OxZsUQt4=";
        };
        cargoSha256 = "sha256-5ga1XQxy6I6plBDHbv3v9oZ3eBV3ue1HLQJrTpzZBTs=";
        doCheck = false;
        cargoCheckCommand = "true";
      };
    };
    devShells = {
      cosmwasm = pkgs.mkShell {
        buildInputs = with self'.packages; [
          subwasm
          wasm-optimizer
          cosmwasm-check
          junod
        ];
        shellHook = ''
          echo "junod alice key:"
          echo "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose" | junod keys add alice --recover --keyring-backend test || true
        '';
      };
    };
  };
}
