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
        eth-executor = pkgs.writeShellApplication {
          name = "eth-executor";
          runtimeInputs = [ self.inputs.ethereum.packages.${system}.geth ];
          text = ''
            DATADIR=./flake/eth-pos-devnet/execution/
            
            geth --http --http.api=eth --http.addr=0.0.0.0 --authrpc.vhosts=* --authrpc.addr=0.0.0.0 --authrpc.jwtsecret=./eth-pos-devnet/jwtsecret --datadir="$DATADIR" --allow-insecure-unlock --unlock=0x123463a4b065722e99115d6c222f267d9cabb524 --password="$DATADIR" --nodiscover --syncmode=full
          '';
        };
        eth-executor = pkgs.writeShellApplication {
          name = "eth-executor";
          runtimeInputs = [ self.inputs.ethereum.packages.${system}.geth ];
          text = ''
            DATADIR=./flake/eth-pos-devnet/execution/
            
            geth --http --http.api=eth --http.addr=0.0.0.0 --authrpc.vhosts=* --authrpc.addr=0.0.0.0 --authrpc.jwtsecret=./eth-pos-devnet/jwtsecret --datadir="$DATADIR" --allow-insecure-unlock --unlock=0x123463a4b065722e99115d6c222f267d9cabb524 --password="$DATADIR" --nodiscover --syncmode=full
          '';
        };

      };
    };
}
