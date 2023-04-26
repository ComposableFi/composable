{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    apps = {
      composable-follow = {
        type = "app";
        program = pkgs.writeShellApplication {
          name = "composable-follow";
          runtimeInputs = [ self'.packages.composable-node ];
          text = ''
              composable --chain=composable --listen-addr=/ip4/0.0.0.0/tcp/30334 --prometheus-port 9615 --base-path data --execution=wasm --state-pruning=256 --blocks-pruning=256 --ws-external --rpc-external --rpc-cors=all --unsafe-rpc-external --rpc-methods=unsafe --ws-port 9988 --unsafe-ws-external --rpc-port 39988 --in-peers 1000 --out-peers 1000 --ws-max-connections 10000  --sync=warp -- --execution=wasm --listen-addr=/ip4/0.0.0.0/tcp/30333 --sync=warp 
           '';
        };
      };
      run-in-docker = {
        type = "app";
        program = pkgs.writeShellApplication {
          name = "run-in-docker";
          runtimeInputs = [ ];
          text = ''
            docker run --rm --volume /var/run/docker.sock:/var/run/docker.sock --volume nix:/nix -it nixos/nix bash -c "nix run composable#''${1-} --print-build-logs --extra-experimental-features nix-command --extra-experimental-features flakes --option sandbox relaxed --show-trace --accept-flake-config" 
          '';
        };
      };
    };   
  };
}
