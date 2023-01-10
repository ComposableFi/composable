{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, devnetTools, ... }: {
    packages = let packages = self'.packages;
    in rec {
      # Centauri Persistent Devnet
      devnet-centauri = pkgs.composable.mkDevnetProgram "devnet-centauri"
        (import ./specs/centauri.nix {
          inherit pkgs devnetTools;
          devnet-a = packages.zombienet-dali-centauri-a;
          devnet-b = packages.zombienet-dali-centauri-b;
        });

      # Picasso devnet
      devnet-picasso = (pkgs.callPackage devnetTools.mk-devnet {
        inherit (packages) polkadot-launch composable-node polkadot-node;
        chain-spec = "picasso-dev";
      }).script;

      devnet-container = devnetTools.mk-devnet-container {
        container-tools = devnetTools.withDevNetContainerTools;
        containerName = "composable-devnet-container";
        devNet = packages.zombienet-rococo-local-dali-dev;
      };

      devnet-picasso-complete = let
        config =
          (pkgs.callPackage ../scripts/polkadot-launch/all-dev-local.nix {
            chainspec = "picasso-dev";
            polkadot-bin = packages.polkadot-node;
            composable-bin = packages.composable-node;
            statemine-bin = packages.statemine-node;
            acala-bin = packages.acala-node;
          }).result;
        config-file = pkgs.writeTextFile {
          name = "all-dev-local.json";
          text = "${builtins.toJSON config}";
        };
      in pkgs.writeShellApplication {
        name = "devnet-picasso-complete";
        text = ''
          cat ${config-file}
          rm -rf /tmp/polkadot-launch
          ${packages.polkadot-launch}/bin/polkadot-launch ${config-file} --verbose
        '';
      };

      devnet-dali-complete = let
        config =
          (pkgs.callPackage ../scripts/polkadot-launch/all-dev-local.nix {
            chainspec = "dali-dev";
            polkadot-bin = packages.polkadot-node;
            composable-bin = packages.composable-node;
            statemine-bin = packages.statemine-node;
            acala-bin = packages.acala-node;
          }).result;
        config-file = pkgs.writeTextFile {
          name = "all-dev-local.json";
          text = "${builtins.toJSON config}";
        };
      in pkgs.writeShellApplication {
        name = "devnet-dali-complete";
        text = ''
          cat ${config-file}
          rm -rf /tmp/polkadot-launch
          ${packages.polkadot-launch}/bin/polkadot-launch ${config-file} --verbose
        '';
      };

      devnet-initialize-script-local = devnetTools.mkDevnetInitializeScript {
        polkadotUrl = "ws://localhost:9944";
        composableUrl = "ws://localhost:9988";
        parachainIds = [ 1000 2000 2087 ];
      };

      devnet-initialize-script-dali-persistent =
        devnetTools.mkDevnetInitializeScript {
          polkadotUrl =
            "wss://persistent.devnets.composablefinance.ninja/chain/rococo";
          composableUrl =
            "wss://persistent.devnets.composablefinance.ninja/chain/dali";
          parachainIds = [ 1000 2000 2087 ];
        };

      devnet-initialize-script-picasso-persistent =
        devnetTools.mkDevnetInitializeScript {
          polkadotUrl =
            "wss://persistent.picasso.devnets.composablefinance.ninja/chain/rococo";
          composableUrl =
            "wss://persistent.picasso.devnets.composablefinance.ninja/chain/picasso";
          parachainIds = [ 1000 2000 2087 ];
        };

      devnet = pkgs.composable.mkDevnetProgram "devnet-default"
        (import ./specs/default.nix {
          inherit pkgs devnetTools;
          price-feed = packages.price-feed;
          devnet = packages.devnet-dali-complete;
          frontend = packages.frontend-static;
        });

      devnet-xcvm = pkgs.composable.mkDevnetProgram "devnet-xcvm"
        (import ./specs/xcvm.nix {
          inherit pkgs devnetTools;
          devnet-dali = packages.zombienet-rococo-local-dali-dev;
        });

      devnet-dali-persistent =
        pkgs.composable.mkDevnetProgram "devnet-dali-persistent"
        (import ./specs/default.nix {
          inherit pkgs devnetTools;
          price-feed = packages.price-feed;
          devnet = packages.devnet-dali-complete;
          frontend = packages.frontend-static-persistent;
        });

      devnet-picasso-persistent =
        pkgs.composable.mkDevnetProgram "devnet-picasso-persistent"
        (import ./specs/default.nix {
          inherit pkgs;
          price-feed = packages.price-feed;
          devnet = packages.devnet-picasso-complete;
          frontend = packages.frontend-static-picasso-persistent;
        });
    };
  };
}
