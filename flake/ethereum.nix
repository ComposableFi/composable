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
            DATADIR=/tmp/composable-devnet/eth
            rm --recursive --force "$DATADIR"
            mkdir --parents "$DATADIR"
            cp --dereference --no-preserve=mode,ownership --recursive --force "${self.inputs.eth-pos-devnet-src}/" "$DATADIR"  
          '';
        };

        eth-consensus-gen = pkgs.writeShellApplication {
          name = "eth-consensus-gen";
          runtimeInputs = [ self.inputs.ethereum.packages.${system}.prysm self.inputs.eth-pos-devnet-src ];
          text = ''
            mkdir --parents /tmp/composable-devnet
            DATADIR=./flake/eth-pos-devnet/consensus
            BASE_DIR=./flake/eth-pos-devnet
            prysmctl \
            testnet \
            generate-genesis \
            --fork=bellatrix \
            --num-validators=64 \
            --output-ssz="$DATADIR/genesis.ssz" \
            --chain-config-file="$DATADIR/config.yml" \
            --geth-genesis-json-in="$BASE_DIR/execution/genesis.json" \
            --geth-genesis-json-out="$BASE_DIR/execution/consensus-genesis.json"
          '';
        };
        eth-executor-gen = pkgs.writeShellApplication {
          name = "eth-executor-gen";
          runtimeInputs = [ self.inputs.ethereum.packages.${system}.geth ];
          text = ''    
            DATADIR=./flake/eth-pos-devnet/execution/
            geth --datadir="$DATADIR" init "$DATADIR/genesis.json"
          '';
        };
        eth-executor = pkgs.writeShellApplication {
          name = "eth-executor";
          runtimeInputs = [ self.inputs.ethereum.packages.${system}.geth ];
          text = ''
            BASE_DIR=./flake/eth-pos-devnet/
            DATADIR=./flake/eth-pos-devnet/execution/
            
            geth --http --http.api=eth --http.addr=0.0.0.0 --authrpc.vhosts=* --authrpc.addr=0.0.0.0 --authrpc.jwtsecret="$BASE_DIR/jwtsecret" --datadir="$DATADIR" --allow-insecure-unlock --unlock=0x123463a4b065722e99115d6c222f267d9cabb524 --password="$DATADIR/geth_password.txt" --nodiscover --syncmode=full
          '';
        };

      };
    };
}
