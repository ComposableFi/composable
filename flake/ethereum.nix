{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, ... }: {
      packages = rec {
        forge = pkgs.stdenv.mkDerivation rec {
          name = "forge";
          src = self.inputs.ethereum.packages.${system}.foundry;
          installPhase = ''
            mkdir --parents $out/bin
            cp $src/bin/forge $out/bin/forge
          '';
        };

        eth-gen = pkgs.writeShellApplication {
          name = "eth-gen";
          text = ''
            BASE_DIR=/tmp/composable-devnet
            DATADIR="$BASE_DIR/eth"
            rm --recursive --force "$DATADIR"
            mkdir --parents "$BASE_DIR"
            cp --dereference --no-preserve=mode,ownership --recursive --force "${self.inputs.eth-pos-devnet-src}/" "$DATADIR"
          '';
        };

        eth-consensus-gen = pkgs.writeShellApplication {
          name = "eth-consensus-gen";
          runtimeInputs = [ self.inputs.ethereum.packages.${system}.prysm ];
          text = ''
            BASE_DIR=/tmp/composable-devnet/eth
            prysmctl \
            testnet \
            generate-genesis \
            --fork=capella \
            --num-validators=64 \
            --output-ssz="$BASE_DIR/consensus/genesis.ssz" \
            --chain-config-file="$BASE_DIR/consensus/config.yml" \
            --geth-genesis-json-in="$BASE_DIR/execution/genesis.json" \
            --geth-genesis-json-out="$BASE_DIR/execution/genesis.json"
          '';
        };

        eth-executor-gen = pkgs.writeShellApplication {
          name = "eth-executor-gen";
          runtimeInputs = [ self.inputs.ethereum.packages.${system}.geth ];
          text = ''
            BASE_DIR=/tmp/composable-devnet/eth
            DATADIR="$BASE_DIR/execution"
            geth --datadir="$DATADIR" init "$DATADIR/genesis.json"
          '';
        };
        eth-executor = pkgs.writeShellApplication {
          name = "eth-executor";
          runtimeInputs = [ self.inputs.ethereum.packages.${system}.geth ];
          text = ''
            BASE_DIR=/tmp/composable-devnet/eth
            DATADIR="$BASE_DIR/execution"          
                        
            geth --http --http.api=eth --http.addr=0.0.0.0 --authrpc.vhosts=* --authrpc.addr=0.0.0.0 --authrpc.jwtsecret="$BASE_DIR/jwtsecret" --datadir="$DATADIR" --allow-insecure-unlock --unlock=0x123463a4b065722e99115d6c222f267d9cabb524 --password="$DATADIR/geth_password.txt" --nodiscover --syncmode=full
          '';
        };

        eth-consensus = pkgs.writeShellApplication {
          name = "eth-consensus";
          runtimeInputs = [ self.inputs.ethereum.packages.${system}.prysm ];
          text = ''
            BASE_DIR=/tmp/composable-devnet/eth
            DATADIR="$BASE_DIR/consensus"          

            beacon-chain \
              --datadir="$DATADIR/beacondata" \
              --min-sync-peers=0 \
              --genesis-state="$DATADIR/genesis.ssz" \
              --interop-eth1data-votes \
              --bootstrap-node= \
              --chain-config-file="$DATADIR/config.yml" \
              --chain-id=32382 \
              --rpc-host=0.0.0.0 \
              --grpc-gateway-host=0.0.0.0 \
              --execution-endpoint=http://localhost:8551 \
              --accept-terms-of-use \
              --jwt-secret="$BASE_DIR/jwtsecret" \
              --suggested-fee-recipient=0x123463a4b065722e99115d6c222f267d9cabb524          
          '';
        };

        eth-validator = pkgs.writeShellApplication {
          name = "eth-validator";
          runtimeInputs = [ self.inputs.ethereum.packages.${system}.prysm ];
          text = ''
            BASE_DIR=/tmp/composable-devnet/eth
            DATADIR="$BASE_DIR/consensus/validatordata"                   
            mkdir --parents "$DATADIR"

            validator \
              --force-clear-db \
              --accept-terms-of-use \
              --interop-num-validators=64 \
              --interop-start-index=0 \
              --chain-config-file="$BASE_DIR/consensus/config.yml"
          '';
        };
      };
    };
}
