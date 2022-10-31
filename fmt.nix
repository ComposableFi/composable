{ self, ... }:
{
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      rust-nightly = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      filesWithExtension = extension:
        pkgs.stdenv.mkDerivation {
          name = "files-with-extension-${extension}";
          src = builtins.filterSource
            (path: type:
              (type == "directory" && baseNameOf path != ".git") || (type
              == "regular" && pkgs.lib.strings.hasSuffix ".${extension}" path))
            ./.;
          dontUnpack = true;
          installPhase = ''
            mkdir $out/
            cp -r $src/. $out/
          '';
        };

      allNixFiles = filesWithExtension "nix";
      allTomlFiles = filesWithExtension "toml";
    in
    {
      packages = {
        fmt = pkgs.writeShellApplication {
          name = "fmt-composable";

          runtimeInputs = with pkgs; [
            nixfmt
            coreutils
            taplo
            nodePackages.prettier
          ] ++ [
            rust-nightly
          ];

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

        nixfmt-check = pkgs.stdenv.mkDerivation {
          name = "nixfmt-check";
          dontUnpack = true;

          buildInputs = [ allNixFiles pkgs.nixfmt ];
          installPhase = ''
            mkdir $out
            nixfmt --version
            SRC=$(find ${allNixFiles} -name "*.nix" -type f | tr "\n" " ")
            echo $SRC
            nixfmt --check $SRC
          '';
        };

        deadnix-check = pkgs.stdenv.mkDerivation {
          name = "deadnix-check";
          dontUnpack = true;

          buildInputs = [ allNixFiles pkgs.deadnix ];
          installPhase = ''
            mkdir $out
            deadnix --version
            SRC=$(find ${allNixFiles} -name "*.nix" -type f | tr "\n" " ")
            echo $SRC
            deadnix $SRC
          '';
        };

        taplo-cli-check = pkgs.stdenv.mkDerivation {
          name = "taplo-cli-check";
          dontUnpack = true;
          buildInputs = [ allTomlFiles pkgs.taplo-cli ];
          installPhase = ''
            mkdir $out
            cd ${allTomlFiles}
            taplo check --verbose
          '';
        };
      };
    };
}
