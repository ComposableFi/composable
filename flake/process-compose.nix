{ ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, ... }: {
    process-compose.devnet-xc = {
      settings = {
        processes = {
          centauri = {
            command = self'.packages.centaurid-gen;
            readiness_probe.http_get = {
              host = "127.0.0.1";
              port = 26657;
            };
          };
          centauri-init = {
            command = self'.packages.centaurid-init;
            depends_on."centauri".condition = "process_healthy";
          };
          picasso = {
            command = self'.packages.zombienet-rococo-local-picasso-dev;
            availability = { restart = "on_failure"; };
            log_location = "/tmp/composable-devnet/zombienet.log";
          };
          hyperspace-client = {
            command = ''
              sleep 20
              COMPOSABLE_DATA=/tmp/composable-devnet/
              HYPERSPACE_DATA="$COMPOSABLE_DATA/hyperspace"
              RUST_LOG="hyperspace=trace,hyperspace_parachain=trace,hyperspace_cosmos=trace"
              export RUST_LOG
              mkdir --parents "$COMPOSABLE_DATA"
              mkdir --parents "$HYPERSPACE_DATA"

              cp --dereference --no-preserve=mode,ownership --force ${self'.packages.hyperspace-config-chain-2} $HYPERSPACE_DATA/config-chain-2.toml  
              cp --dereference --no-preserve=mode,ownership --force ${self'.packages.hyperspace-config-chain-3} $HYPERSPACE_DATA/config-chain-3.toml  
              cp --dereference --no-preserve=mode,ownership --force ${self'.packages.hyperspace-config-core} $HYPERSPACE_DATA/config-core.toml                
              CODE_ID=$(cat /tmp/centauri-dev/code_id)
              sed -i "s/wasm_code_id = \"0000000000000000000000000000000000000000000000000000000000000000\"/wasm_code_id = \"$CODE_ID\"/" "$HYPERSPACE_DATA/config-chain-2.toml"
              ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-clients --config-a $HYPERSPACE_DATA/config-chain-3.toml --config-b $HYPERSPACE_DATA/config-chain-2.toml --config-core $HYPERSPACE_DATA/config-core.toml --delay-period 10
            '';
            log_location = "/tmp/composable-devnet/hyperspace/clients.log";
            depends_on = {
              "centauri-init".condition = "process_completed_successfully";
              "centauri".condition = "process_healthy";
            };
            availability = { restart = "on_failure"; };
          };
          hyperspace-connection = {
            command = ''
              COMPOSABLE_DATA=/tmp/composable-devnet/
              HYPERSPACE_DATA="$COMPOSABLE_DATA/hyperspace"
              RUST_LOG="hyperspace=trace,hyperspace_parachain=trace,hyperspace_cosmos=trace"
              export RUST_LOG      
              ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-connection --config-a $HYPERSPACE_DATA/config-chain-3.toml --config-b $HYPERSPACE_DATA/config-chain-2.toml --config-core $HYPERSPACE_DATA/config-core.toml --delay-period 10
            '';
            log_location = "/tmp/composable-devnet/hyperspace/connection.log";
            depends_on = {
              "hyperspace-client".condition = "process_completed_successfully";
            };
            availability = { restart = "on_failure"; };
          };
          hyperspace-channels = {
            command = ''
              COMPOSABLE_DATA=/tmp/composable-devnet/
              HYPERSPACE_DATA="$COMPOSABLE_DATA/hyperspace"
              RUST_LOG="hyperspace=trace,hyperspace_parachain=trace,hyperspace_cosmos=trace"
              export RUST_LOG
              ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-channel --config-a $HYPERSPACE_DATA/config-chain-3.toml --config-b $HYPERSPACE_DATA/config-chain-2.toml --config-core $HYPERSPACE_DATA/config-core.toml --delay-period 10 --port-id transfer --version ics20-1 --order unordered
            '';
            log_location = "/tmp/composable-devnet/hyperspace/channels.log";
            depends_on = {
              "hyperspace-connection".condition =
                "process_completed_successfully";
            };
            availability = { restart = "on_failure"; };
          };
          hyperspace-relay = {
            command = ''
              COMPOSABLE_DATA=/tmp/composable-devnet/
              HYPERSPACE_DATA="$COMPOSABLE_DATA/hyperspace"
              RUST_LOG="hyperspace=trace,hyperspace_parachain=trace,hyperspace_cosmos=trace"
              export RUST_LOG
              ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace relay --config-a $HYPERSPACE_DATA/config-chain-3.toml --config-b $HYPERSPACE_DATA/config-chain-2.toml --config-core $HYPERSPACE_DATA/config-core.toml --delay-period 10
            '';
            log_location = "/tmp/composable-devnet/hyperspace/relay.log";
            depends_on = {
              "hyperspace-channels".condition =
                "process_completed_successfully";
            };
            availability = { restart = "on_failure"; };
          };
        };
      };
    };
  };
}
