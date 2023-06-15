{ ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, ... }: {
    process-compose.devnet-xc =
      {
        settings = {
          processes = {
            sleep = {
              command = ''
                sleep 20
              '';
            };
            centauri = {
              command = self'.packages.centaurid-gen;
            };
            picasso = {
              command = self'.packages.zombienet-rococo-local-picasso-dev;
            };
            hyperspace = {
              command = ''
                COMPOSABLE_DATA=/tmp/composable-devnet/
                HYPERSPACE_DATA="$COMPOSABLE_DATA/hyperspace"
                RUST_LOG="trace,jsonrpsee_client_transport::ws=debug,soketto=debug,tracing::span=debug,mio::poll=debug,trie=debug,jsonrpsee_core::client::async_client=debug"
                export RUST_LOG
                mkdir --parents "$COMPOSABLE_DATA"
                mkdir --parents "$HYPERSPACE_DATA"
                
                cp -f ${self'.packages.hyperspace-config-chain-a} $HYPERSPACE_DATA/config-chain-a.toml  
                cp -f ${self'.packages.hyperspace-config-chain-b} $HYPERSPACE_DATA/config-chain-b.toml  
                cp -f ${self'.packages.hyperspace-config-chain-2} $HYPERSPACE_DATA/config-chain-2.toml  
                cp -f ${self'.packages.hyperspace-config-core} $HYPERSPACE_DATA/config-core.toml                
                ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-clients --config-a $HYPERSPACE_DATA/config-chain-a.toml --config-b $HYPERSPACE_DATA/config-chain-2.toml --config-core $HYPERSPACE_DATA/config-core.toml --delay-period 10
              '';
            };
          };
        };
      };
  };
}
