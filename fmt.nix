{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      rust-nightly =
        pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in {
      packages = {
        fmt = pkgs.writeShellApplication {
          name = "fmt-composable";

          runtimeInputs = with pkgs;
            [ nixfmt coreutils taplo nodePackages.prettier ]
            ++ [ rust-nightly ];

          text = ''
              # .nix
            	find . -name "*.nix" -type f -print0 | xargs -0 nixfmt;

              # .toml
              taplo fmt

              # .rs
            	find . -path ./code/target -prune -o -name "*.rs" -type f -print0 | xargs -0 rustfmt --edition 2021;

              # .js .ts .tsx
              prettier \
                --config="./code/integration-tests/runtime-tests/.prettierrc" \
                --write \
                --ignore-path="./code/integration-tests/runtime-tests/.prettierignore" \
                ./code/integration-tests/runtime-tests/
          '';
        };
      };
    };
}
