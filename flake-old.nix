{
  description = "Composable Finance systems, tools and releases";
  inputs = {
    bundlers = {
      url = "github:NixOS/bundlers";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, npm-buildpackage
    , arion-src, home-manager, helix, bundlers }:
    let

      eachSystemOutputs = flake-utils.lib.eachDefaultSystem (system:
        let
          mk-xcvm-contract = name:
            crane-nightly.buildPackage (common-attrs // {
              pnameSuffix = name;
              cargoBuildCommand =
                "cargo build --target wasm32-unknown-unknown --profile cosmwasm-contracts -p ${name}";
              RUSTFLAGS = "-C link-arg=-s";
            });

        in rec {
          packages = rec {
            generated-release-body = let
              subwasm-call = runtime:
                builtins.readFile (pkgs.runCommand "subwasm-info" { } ''
                  ${subwasm}/bin/subwasm info ${runtime}/lib/runtime.optimized.wasm | tail -n+2 | head -c -1 > $out
                '');
              flake-url =
                "github:ComposableFi/composable/v${composable-node.version}";
            in pkgs.writeTextFile {
              name = "release.txt";
              text = ''
                ## Runtimes
                ### Dali
                ```
                ${subwasm-call dali-runtime}```
                ### Picasso
                ```
                ${subwasm-call picasso-runtime}
                ```
                ### Composable
                ```
                ${subwasm-call composable-runtime}
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
                  bundlers.bundlers."${system}"."${type}" package;
                subwasm-version = runtime:
                  builtins.readFile (pkgs.runCommand "subwasm-version" { } ''
                    ${subwasm}/bin/subwasm version ${runtime}/lib/runtime.optimized.wasm | grep specifications | cut -d ":" -f2 | cut -d " " -f3 | head -c -1 > $out
                  '');
              in ''
                mkdir -p release-artifacts/to-upload/

                # Generate release body
                cp ${generated-release-body} release-artifacts/release.txt

                # Generate wasm runtimes
                cp ${dali-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/dali_runtime_${
                  subwasm-version dali-runtime
                }.wasm
                cp ${picasso-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/picasso_runtime_${
                  subwasm-version picasso-runtime
                }.wasm
                cp ${composable-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/composable_runtime_${
                  subwasm-version composable-runtime
                }.wasm

                # Generate packaged binaries
                # RPM Name convention: https://docs.oracle.com/en/database/oracle/oracle-database/19/ladbi/rpm-packages-naming-convention.html
                cp ${
                  make-bundle "toRPM" composable-node-release
                }/*.rpm release-artifacts/to-upload/composable-node-${composable-node-release.version}-1.x86_64.rpm
                # DEB Name convention: https://askubuntu.com/questions/330018/what-is-the-standard-for-naming-deb-file-name
                cp ${
                  make-bundle "toDEB" composable-node-release
                }/*.deb release-artifacts/to-upload/composable-node_${composable-node-release.version}-1_amd64.deb
                cp ${
                  make-bundle "toDockerImage" composable-node-release
                } release-artifacts/composable-docker-image

                # Checksum everything
                cd release-artifacts/to-upload
                sha256sum ./* > checksums.txt
              '';
            };
            xcvm-contract-asset-registry =
              mk-xcvm-contract "xcvm-asset-registry";
            xcvm-contract-router = mk-xcvm-contract "xcvm-router";
            xcvm-contract-interpreter = mk-xcvm-contract "xcvm-interpreter";
            # TODO: inherit and provide script to run all stuff

          };


          apps = let
            makeApp = p: {
              type = "app";
              program = pkgs.lib.meta.getExe p;
            };
          in rec {
         };
        });
    in eachSystemOutputs // {};
}
