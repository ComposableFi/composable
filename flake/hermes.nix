{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, ... }:
    let
      devnet-root-directory = "/tmp/composable-devnet";
      validator-key = "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj";
    in {
      packages = rec {
        hermes = self.inputs.cosmos.packages.${system}.hermes_1_5_1;
        osmosis-centauri-hermes-init = pkgs.writeShellApplication {
          runtimeInputs = [ hermes ];
          name = "osmosis-centauri-hermes-init";
          text = ''
            HOME=$(realpath .)
            MNEMONIC_FILE="$HOME/.hermes/mnemonics/relayer.txt"
            export HOME
            mkdir --parents "$HOME/.hermes/mnemonics/"

            echo "black frequent sponsor nice claim rally hunt suit parent size stumble expire forest avocado mistake agree trend witness lounge shiver image smoke stool chicken" > "$MNEMONIC_FILE"
            hermes keys add --chain centauri-dev --mnemonic-file "$MNEMONIC_FILE" --key-name centauri-dev --overwrite
            hermes keys add --chain osmosis-dev --mnemonic-file "$MNEMONIC_FILE" --key-name osmosis-dev --overwrite
            RUST_LOG=info
            export RUST_LOG
            hermes create channel --a-chain centauri-dev --b-chain osmosis-dev --a-port transfer --b-port transfer --new-client-connection --yes
          '';
        };
        osmosis-centauri-hermes-relay = pkgs.writeShellApplication {
          runtimeInputs = [ hermes ];
          name = "osmosis-centauri-hermes-relay";
          text = ''
            HOME=$(realpath .)
            export HOME
            RUST_LOG=info
            export RUST_LOG
            hermes start
          '';
        };
      };
    };
}
