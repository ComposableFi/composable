{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = rec {
      devnet-integration-tests = pkgs.writeShellApplication {
        runtimeInputs = with pkgs;
          with self'.packages; [
            curl
            dasel
            nodejs
            coreutils
            process-compose
            centaurid
            osmosisd
          ];
        name = "devnet-integration-tests";
        text = ''
          # shellcheck disable=SC2069
          ( ${
            pkgs.lib.meta.getExe self'.packages.devnet-xc-fresh-background
          } 2>&1 & ) | tee devnet-xc.log &

          PATH=$PATH:$(pwd)
          export PATH
          cd code/integration-tests/runtime-tests
          npm install
          npm run generate-types
          npm run test:cosmos
          sleep 8
          set +o errexit
          pkill -SIGKILL process-compose
          set -o errexit
          exit 0
        '';
      };
    };
    apps = {
      devnet-integration-tests = self.inputs.flake-utils.lib.mkApp {
        drv = self'.packages.devnet-integration-tests;
      };
    };
  };
}
