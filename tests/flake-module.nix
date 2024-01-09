{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, bashTools, ... }: {
    packages = rec {
      mantis-tests = pkgs.writeShellApplication {
        runtimeInputs = with pkgs; [ bun ];
        name = "tests";
        text = ''
          bun install
          ${bashTools.export pkgs.networksLib.devnet.mnemonics}
          bun tests.ts -- --mnemonic "$APPLICATION1" --outpost-contract-address "$(cat ${pkgs.networksLib.pica.devnet.CHAIN_DATA}/outpost_contract_address)"
        '';
      };
      mantis-e2e = pkgs.writeShellApplication {
        runtimeInputs = with pkgs;
          with self'.packages; [
            curl
            dasel
            nodejs
            coreutils
            process-compose
            centaurid
            osmosisd
            pckill
            devnet-prune
            devnet-cosmos-fresh-background
          ];
        name = "mantis-e2e";
        text = ''
          devnet-prune         
          devnet-cosmos-fresh-background 2>devnet-cosmos-fresh-background.log 1>devnet-cosmos-fresh-background.log &
          TRIES=0
          SUCCESS_CODE=1
          while test $TRIES -le 64; do
            if [[ -e "${pkgs.networksLib.pica.devnet.CHAIN_DATA}/outpost_contract_address" ]]; then
              # test
              echo "mantis::tests:: need to sleep loong until IBC is alive, and little bit on top when CVM"
              sleep 128 # move waiting to separate tests
              sleep 16 # move waiting to separate tests
              echo "mantis::tests:: launching tests "
              (
                ${bashTools.export pkgs.networksLib.devnet.mnemonics}
                cd tests
                nix run .#mantis-tests -- --mnemonic "$APPLICATION1" --outpost-contract-address "$(cat ${pkgs.networksLib.pica.devnet.CHAIN_DATA}/outpost_contract_address)"
              )
              SUCCESS_CODE=0
              pckill
              break
            fi
            if [[ $TRIES -gt 32 ]]; then
              echo "mantis::tests:: failed to start devnet"
              pckill              
              exit 17
            fi
            ((TRIES=TRIES+1))
            echo "mantis::tests:: Waiting for devnet to start and CVM init one more time $TRIES"
            sleep 8
          done
          pckill
          exit $SUCCESS_CODE
        '';
      };
    };
    apps = {
      mantis-e2e =
        self.inputs.flake-utils.lib.mkApp { drv = self'.packages.mantis-e2e; };
    };
  };
}
