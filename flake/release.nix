{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , devnetTools, centauri, osmosis, bashTools, ... }: {
      packages = let
        nix-config = ''
          --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed'';
        packages = self'.packages;
        make-bundle = type: package:
          self.inputs.bundlers.bundlers."${system}"."${type}" package;
        subwasm-version = runtime:
          builtins.readFile (pkgs.runCommand "subwasm-version" { } ''
            ${pkgs.subwasm}/bin/subwasm version ${runtime}/lib/runtime.optimized.wasm | grep specifications | cut -d ":" -f2 | cut -d " " -f3 | head -c -1 > $out
          '');

      in rec {
        generated-release-body = let
          subwasm-call = runtime:
            builtins.readFile (pkgs.runCommand "subwasm-info" { } ''
              ${pkgs.subwasm}/bin/subwasm info ${runtime}/lib/runtime.optimized.wasm | tail -n+2 | head -c -1 > $out
            '');
          flake-url =
            "github:ComposableFi/composable/release-v${packages.composable-node.version}";
        in pkgs.writeTextFile {
          name = "release.txt";
          text = ''
            ## Runtimes
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
            nix build ${flake-url}#picasso-runtime ${nix-config}
            nix build ${flake-url}#composable-runtime ${nix-config}

            # Run the Composable node (release mode) alone
            nix run ${flake-url}#composable-node ${nix-config}

            # Spin up a local devnet
            nix run ${flake-url}#devnet-picasso ${nix-config}
            nix run ${flake-url}#devnet-composable ${nix-config}

            # CW CLI tool
            nix run ${flake-url}#ccw ${nix-config}

            # Spin up a local XC(Inter chain) devnet
            nix run ${flake-url}#devnet-xc-fresh ${nix-config}
            ```
          '';
        };

        tag-release = pkgs.writeShellApplication {
          name = "tag-release";
          runtimeInputs = [ pkgs.git pkgs.yq ];
          text = ''
            git tag --sign "release-v$1" --message "RC" && git push origin "release-v$1" --force
          '';
        };

        delete-release-tag-unsafe = pkgs.writeShellApplication {
          name = "delete-release-tag-unsafe";
          runtimeInputs = [ pkgs.git ];
          text = ''
            # shellcheck disable=SC2015
            git tag --delete "release-v$1" || true && git push --delete origin "release-v$1"
          '';
        };

        generate-release-artifacts = pkgs.writeShellApplication {
          name = "generate-release-artifacts";
          runtimeInputs = devnetTools.withBuildTools;
          text = ''
            mkdir -p release-artifacts/to-upload/

            echo "Generate release body"
            cp ${generated-release-body} release-artifacts/release.txt

            echo "Generate wasm runtimes"
            cp ${packages.picasso-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/picasso_runtime_${
              subwasm-version packages.picasso-runtime
            }.wasm
            cp ${packages.composable-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/composable_runtime_${
              subwasm-version packages.composable-runtime
            }.wasm

            cp ${packages.picasso-testfast-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/picasso_testfast_runtime_${
              subwasm-version packages.picasso-testfast-runtime
            }.wasm

            cp ${packages.composable-testfast-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/composable_testfast_runtime_${
              subwasm-version packages.composable-testfast-runtime
            }.wasm

            echo "Generate node packages"
            cp ${
              make-bundle "toRPM" packages.composable-node
            }/*.rpm release-artifacts/to-upload/composable-node-${packages.composable-node.version}-1.x86_64.rpm
            cp ${
              make-bundle "toDEB" packages.composable-node
            }/*.deb release-artifacts/to-upload/composable-node_${packages.composable-node.version}-1_amd64.deb
            cp ${packages.composable-node-image} release-artifacts/composable-image

            cp ${
              make-bundle "toRPM" packages.composable-testfast-node
            }/*.rpm release-artifacts/to-upload/composable-testfast-node-${packages.composable-testfast-node.version}-1.x86_64.rpm
            cp ${
              make-bundle "toDEB" packages.composable-testfast-node
            }/*.deb release-artifacts/to-upload/composable-testfast-node_${packages.composable-testfast-node.version}-1_amd64.deb
            cp ${
              make-bundle "toDockerImage" packages.composable-testfast-node
            } release-artifacts/composable-testfast-node-docker-image

            echo "Devnet"
            cp ${packages.devnet-image} release-artifacts/devnet-image

            echo "Bridge"
            cp ${packages.hyperspace-composable-polkadot-picasso-kusama-image} release-artifacts/hyperspace-composable-polkadot-picasso-kusama-image

            # Checksum everything
            cd release-artifacts/to-upload
            sha256sum ./* > checksums.txt
          '';
        };

        gov-prod-cvm = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ packages.osmosisd pkgs.jq ];
          name = "gov-prod-cvm";
          text = ''
             if [[ -f .secret/CI_COSMOS_MNEMONIC ]]; then
               CI_COSMOS_MNEMONIC="$(cat .secret/CI_COSMOS_MNEMONIC)"
             fi
             CI_COSMOS_MNEMONIC="''${1-$CI_COSMOS_MNEMONIC}"

             ${bashTools.export osmosis.env.mainnet}

             rm --force --recursive .secret/$DIR 
             mkdir --parents .secret/$DIR

            DEPOSIT=400000000$FEE
            echo "$CI_COSMOS_MNEMONIC" | "$BINARY" keys add CI_COSMOS_MNEMONIC --recover --keyring-backend test --home .secret/$DIR --output json
            ADDRESS=$("$BINARY" keys show CI_COSMOS_MNEMONIC --keyring-backend test --home .secret/$DIR --output json | jq -r '.address')
            echo "$ADDRESS" > .secret/$DIR/ADDRESS

            INTERPRETER_WASM_FILE="${packages.cw-cvm-executor}/lib/cw_cvm_executor.wasm"
            INTERPRETER_WASM_CODE_HASH=$(sha256sum "$INTERPRETER_WASM_FILE"  | head -c 64)
            DESCRIPTION=$(cat ${./release-gov-osmosis-proposal-cvm-upload.md})

             "$BINARY" tx gov submit-proposal wasm-store "$INTERPRETER_WASM_FILE" --title "Upload Composable cross-chain Virtual Machine interpreter contract" \
               --description "$DESCRIPTION" --run-as "$ADDRESS"  \
               --deposit="$DEPOSIT" \
               --code-source-url 'https://github.com/ComposableFi/composable/tree/d4d01f19d8fbe4eafa81f9f2dfd0fd4899998ce6/code/cvm/cosmwasm/contracts/interpreter' \
               --builder "composablefi/devnet:v9.10037.1" \
               --code-hash "$INTERPRETER_WASM_CODE_HASH" \
               --from "$ADDRESS" --keyring-backend test --chain-id $CHAIN_ID --yes --broadcast-mode block \
               --gas 25000000 --gas-prices 0.025$FEE --node "$NODE" --home .secret/$DIR |
                tee .secret/$DIR/INTERPRETER_PROPOSAL

             GATEWAY_WASM_FILE="${packages.cw-cvm-gateway}/lib/cw_cvm_gateway.wasm"
             GATEWAY_WASM_CODE_HASH=$(sha256sum "$GATEWAY_WASM_FILE"  | head -c 64)

             sleep "$BLOCK_SECONDS" 
             "$BINARY" tx gov submit-proposal wasm-store "$GATEWAY_WASM_FILE" --title "Upload Composable cross-chain Virtual Machine gateway contract" \
               --description "$DESCRIPTION" --run-as "$ADDRESS"  \
               --code-source-url 'https://github.com/ComposableFi/composable/tree/d4d01f19d8fbe4eafa81f9f2dfd0fd4899998ce6/code/cvm/cosmwasm/contracts/gateway' \
               --builder "composablefi/devnet:v9.10037.1" \
               --deposit="$DEPOSIT" \
               --code-hash "$GATEWAY_WASM_CODE_HASH" \
               --from "$ADDRESS" --keyring-backend test --chain-id $CHAIN_ID --yes --broadcast-mode block \
               --gas 25000000 --gas-prices 0.025$FEE --node "$NODE" --home .secret/$DIR |
               tee .secret/$DIR/GATEWAY_PROPOSAL
          '';
        };

        release-mainnet-cvm = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ packages.centaurid pkgs.jq ];
          name = "release-mainnet-cvm";
          text = ''
            if [[ -f .secret/CI_COSMOS_MNEMONIC ]]; then
              CI_COSMOS_MNEMONIC="$(cat .secret/CI_COSMOS_MNEMONIC)"
            fi
            CI_COSMOS_MNEMONIC="''${1-$CI_COSMOS_MNEMONIC}"

            ${bashTools.export centauri.env.mainnet}

            rm --force --recursive .secret/$DIR 
            mkdir --parents .secret/$DIR

            EXECUTOR="${packages.cw-cvm-executor}/lib/cw_cvm_executor.wasm"
            GATEWAY="${packages.cw-cvm-gateway}/lib/cw_cvm_gateway.wasm"

            echo "$CI_COSMOS_MNEMONIC" | "$BINARY" keys add CI_COSMOS_MNEMONIC --recover --keyring-backend test --home .secret/$DIR --output json
            ADDRESS=$("$BINARY" keys show CI_COSMOS_MNEMONIC --keyring-backend test --home .secret/$DIR --output json | jq -r '.address')
            echo "$ADDRESS" > .secret/$DIR/ADDRESS

            GATEWAY_TX=$("$BINARY" tx wasm store "$GATEWAY" --keyring-backend test --home .secret/$DIR --output json --node "$NODE" --from CI_COSMOS_MNEMONIC --gas-prices 0.1$FEE --gas auto --gas-adjustment 1.3 --chain-id "$CHAIN_ID" --yes --broadcast-mode sync)
            echo "$GATEWAY_TX"
            GATEWAY_HASH=$(sha256sum < "$GATEWAY" | head -c 64 | tr "[:lower:]" "[:upper:]")

            EXECUTOR_HASH=$(sha256sum < "$EXECUTOR" | head -c 64 | tr "[:lower:]" "[:upper:]")

            sleep $BLOCK_TIME
            echo "$GATEWAY_HASH"
            CENTAURI_GATEWAY_CODE_ID=$("$BINARY" query wasm list-code --home .secret/$DIR --output json --node "$NODE" | jq -r ".code_infos[] | select(.data_hash == \"$GATEWAY_HASH\" and .creator == \"$ADDRESS\" ) | .code_id " | tail --lines 1)
            echo "$CENTAURI_GATEWAY_CODE_ID" > .secret/$DIR/GATEWAY_CODE_ID


            EXECUTOR_TX=$("$BINARY" tx wasm store "$EXECUTOR" --keyring-backend test --home .secret/$DIR --output json --node "$NODE" --from CI_COSMOS_MNEMONIC --gas-prices 0.1$FEE --gas auto --gas-adjustment 1.3 --chain-id "$CHAIN_ID" --yes --broadcast-mode sync)
            echo "$EXECUTOR_TX"

            echo "$EXECUTOR_HASH"
            sleep $BLOCK_TIME
            EXECUTOR_CODE_ID=$("$BINARY" query wasm list-code --home .secret/$DIR --output json --node "$NODE" | jq -r ".code_infos[] | select(.data_hash == \"$EXECUTOR_HASH\" and .creator == \"$ADDRESS\" ) | .code_id " | tail --lines 1)
            echo "$EXECUTOR_CODE_ID" > .secret/$DIR/EXECUTOR_CODE_ID

            INSTANTIATE=$(cat << EOF
                {
                    "admin" : "$ADDRESS", 
                    "network_id" : $NETWORK_ID
                }                                 
            EOF
            )

            INSTANTIATE=$("$BINARY" tx wasm instantiate "$CENTAURI_GATEWAY_CODE_ID" "$INSTANTIATE" --label "cvm-gateway-4" --keyring-backend test --home .secret/$DIR --output json --node "$NODE" --from CI_COSMOS_MNEMONIC --gas-prices 0.1$FEE --gas auto --gas-adjustment 1.3 --chain-id "$CHAIN_ID" --yes --broadcast-mode sync --admin "$ADDRESS")
            echo "$INSTANTIATE"
            sleep $BLOCK_TIME
            GATEWAY_CONTRACT_ADDRESS=$("$BINARY" query wasm list-contract-by-code "$CENTAURI_GATEWAY_CODE_ID" --home .secret/$DIR --output json --node "$NODE"  | jq -r ".contracts | .[-1]")
            echo "$GATEWAY_CONTRACT_ADDRESS" > .secret/$DIR/GATEWAY_CONTRACT_ADDRESS
          '';
        };
      };
    };

}
