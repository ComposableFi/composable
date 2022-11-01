{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      allDirectoriesAndFiles = pkgs.stdenv.mkDerivation {
        name = "allDirectoriesAndFiles";
        src =
          builtins.filterSource (path: _type: baseNameOf path != ".git") ./.;
        dontUnpack = true;
        installPhase = ''
          mkdir $out/
          cp -r $src/. $out/
        '';
      };

      filesWithExtension = extension:
        pkgs.stdenv.mkDerivation {
          name = "files-with-extension-${extension}";
          src = builtins.filterSource (path: type:
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
    in {
      packages = {
        fmt = pkgs.writeShellApplication {
          name = "fmt-composable";

          runtimeInputs = with pkgs;
            [ nixfmt coreutils taplo nodePackages.prettier ]
            ++ [ self'.packages.rust-nightly ];

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

        hadolint-check = pkgs.stdenv.mkDerivation {
          name = "hadolint-check";
          dontUnpack = true;
          buildInputs = [ allDirectoriesAndFiles pkgs.hadolint ];
          installPhase = ''
            mkdir -p $out

            hadolint --version
            total_exit_code=0
            for file in $(find ${allDirectoriesAndFiles} -name "Dockerfile" -or -name "*.dockerfile"); do
              echo "=== $file ==="
              hadolint --config ${allDirectoriesAndFiles}/.hadolint.yaml $file || total_exit_code=$?
              echo ""
            done
            exit $total_exit_code
          '';
        };

        spell-check = pkgs.stdenv.mkDerivation {
          name = "cspell-check";
          dontUnpack = true;
          buildInputs = [ allDirectoriesAndFiles pkgs.nodePackages.cspell ];
          installPhase = ''
            mkdir $out
            echo "cspell version: $(cspell --version)"
            cd ${allDirectoriesAndFiles}
            cspell lint --config cspell.yaml --no-progress "**"
          '';
        };

      };
    };
}
