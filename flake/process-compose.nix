{ ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, ... }: {
    process-compose.devnet-xc =
      {
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
              depends_on."centauri".condition = "process_started_successfully";
            };
            picasso = {
              command = self'.packages.zombienet-rococo-local-picasso-dev;
            };
            hyperspace = {
              command = ''
                sleep 20
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
              depends_on."centauri".condition = "process_started_successfully";
              depends_on."picasso".condition = "process_started_successfully";
            };
          };
        };
      };
  };
}
