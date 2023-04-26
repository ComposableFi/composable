{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    apps = {
      composable-follow = {
        type = "app";
        program = pkgs.writeShellApplication {
          name = "composable-follow";
          runtimeInputs = [ self'.packages.composable-node ];
          text = ''
            nixcomposable  -- --chain=composable --listen-addr=/ip4/0.0.0.0/tcp/30334 --prometheus-port 9615 --base-path data/composable-follow --execution=wasm --ws-external --state-pruning=1024 --blocks-pruning=1024 --rpc-external --rpc-cors=all --unsafe-rpc-external --rpc-methods=unsafe --ws-port 9988 --unsafe-ws-external --rpc-port 39988 --in-peers 1000 --out-peers 1000 --ws-max-connections 10000  --sync=fast-unsafe -- --execution=wasm --listen-addr=/ip4/0.0.0.0/tcp/30333 --sync=fast-unsafe  --state-pruning=1024 --blocks-pruning=1024
          '';
        };
      };
    };
  };
}
