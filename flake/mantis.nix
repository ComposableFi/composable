{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, devnetTools, ... }:
    let
      devnet-root-directory = "/tmp/composable-devnet";
      validator-key = "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj";
      log = "debug";
    in {
      packages = rec {
        mantis-simulate-solve = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "mantis-simulate-solve";
          text = ''
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}"            
            HOME=${devnet-root-directory}
            export HOME
            MNEMONIC_FILE="$HOME/.hermes/mnemonics/relayer.txt"
            export MNEMONIC_FILE
            echo "$HOME/.hermes/mnemonics/"
            mkdir --parents "$HOME/.hermes/mnemonics/"c
            cp --dereference --no-preserve=mode,ownership --force ${
              ./hermes.toml
            } "$HOME/.hermes/config.toml"
            echo "black frequent sponsor nice claim rally hunt suit parent size stumble expire forest avocado mistake agree trend witness lounge shiver image smoke stool chicken" > "$MNEMONIC_FILE"
            hermes keys add --chain centauri-dev --mnemonic-file "$MNEMONIC_FILE" --key-name centauri-dev --overwrite
            hermes keys add --chain osmosis-dev --mnemonic-file "$MNEMONIC_FILE" --key-name osmosis-dev --overwrite
            export RUST_LOG
            hermes create channel --a-chain centauri-dev --b-chain osmosis-dev --a-port transfer --b-port transfer --new-client-connection --yes
          '';
        };
      };
    };
}
