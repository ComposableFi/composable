{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }: {
      packages = let packages = self'.packages;
      in rec {
        generated-release-body = let
          subwasm-call = runtime:
            builtins.readFile (pkgs.runCommand "subwasm-info" { } ''
              ${packages.subwasm}/bin/subwasm info ${runtime}/lib/runtime.optimized.wasm | tail -n+2 | head -c -1 > $out
            '');
          flake-url =
            "github:ComposableFi/composable/v${packages.composable-node.version}";
        in pkgs.writeTextFile {
          name = "release.txt";
          text = ''
            ## Runtimes
            ### Dali
            ```
            ${subwasm-call packages.dali-runtime}```
            ### Picasso
            ```
            ${subwasm-call packages.picasso-runtime}
            ```
            ### Composable
            ```
            ${subwasm-call packages.composable-runtime}
            ```
            ## Nix
            ```bash
            # Generate the Wasm runtimes
            nix build ${flake-url}#dali-runtime
            nix build ${flake-url}#picasso-runtime
            nix build ${flake-url}#composable-runtime

            # Run the Composable node (release mode) alone
            nix run ${flake-url}#composable-node-release

            # Spin up a local devnet
            nix run ${flake-url}#devnet

            # Spin up a local XCVM devnet
            nix run ${flake-url}#devnet-xcvm

            # Show all possible apps, shells and packages
            nix flake show ${flake-url} --allow-import-from-derivation
            ```
          '';
        };

        generate-release-artifacts = pkgs.writeShellApplication {
          name = "generate-release-artifacts";
          text = let
            make-bundle = type: package:
              inputs'.bundlers.bundlers."${system}"."${type}" package;
            subwasm-version = runtime:
              builtins.readFile (pkgs.runCommand "subwasm-version" { } ''
                ${packages.subwasm}/bin/subwasm version ${runtime}/lib/runtime.optimized.wasm | grep specifications | cut -d ":" -f2 | cut -d " " -f3 | head -c -1 > $out
              '');
          in ''
            mkdir -p release-artifacts/to-upload/

            # Generate release body
            cp ${generated-release-body} release-artifacts/release.txt

            # Generate wasm runtimes
            cp ${packages.dali-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/dali_runtime_${
              subwasm-version packages.dali-runtime
            }.wasm
            cp ${packages.picasso-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/picasso_runtime_${
              subwasm-version packages.picasso-runtime
            }.wasm
            cp ${packages.composable-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/composable_runtime_${
              subwasm-version packages.composable-runtime
            }.wasm

            # Generate packaged binaries
            # RPM Name convention: https://docs.oracle.com/en/database/oracle/oracle-database/19/ladbi/rpm-packages-naming-convention.html
            cp ${
              make-bundle "toRPM" packages.composable-node-release
            }/*.rpm release-artifacts/to-upload/composable-node-${packages.composable-node-release.version}-1.x86_64.rpm
            # DEB Name convention: https://askubuntu.com/questions/330018/what-is-the-standard-for-naming-deb-file-name
            cp ${
              make-bundle "toDEB" packages.composable-node-release
            }/*.deb release-artifacts/to-upload/composable-node_${packages.composable-node-release.version}-1_amd64.deb
            cp ${
              make-bundle "toDockerImage" packages.composable-node-release
            } release-artifacts/composable-docker-image

            # Checksum everything
            cd release-artifacts/to-upload
            sha256sum ./* > checksums.txt
          '';
        };

      };

    };
}
