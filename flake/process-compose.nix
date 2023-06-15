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
          };
          hyperspace = {
            command = ''
              COMPOSABLE_DATA=/tmp/composable-devnet/
              HYPERSPACE_DATA="$COMPOSABLE_DATA/hyperspace"
              RUST_LOG="trace,jsonrpsee_client_transport::ws=trace,soketto=trace,tracing::span=trace,mio::poll=trace,trie=trace,jsonrpsee_core::client::async_client=trace"
              export RUST_LOG
              mkdir --parents "$COMPOSABLE_DATA"
              mkdir --parents "$HYPERSPACE_DATA"

              cp -f ${self'.packages.hyperspace-config-chain-2} $HYPERSPACE_DATA/config-chain-2.toml  
              cp -f ${self'.packages.hyperspace-config-chain-3} $HYPERSPACE_DATA/config-chain-3.toml  
              cp -f ${self'.packages.hyperspace-config-core} $HYPERSPACE_DATA/config-core.toml                
              ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-clients --config-a $HYPERSPACE_DATA/config-chain-3.toml --config-b $HYPERSPACE_DATA/config-chain-2.toml --config-core $HYPERSPACE_DATA/config-core.toml --delay-period 10
            '';
            log_location = "/tmp/composable-devnet/hyperspace/log.txt";
            depends_on = {
              "centauri-init".condition = "process_completed_successfully";
              "centauri".condition = "process_healthy";
            };
            availability = { restart = "on_failure"; };
          };
        };
      };
    };
  };
}
