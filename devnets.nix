{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, devnetTools, ... }: {
    packages = let
      packages = self'.packages;

      # for containers which are intended for testing, debug and development (including running isolated runtime)
      docker-in-docker = with pkgs; [ docker docker-buildx docker-compose ];
      containers-tools-minimal = with pkgs; [ acl direnv home-manager cachix ];
      container-tools = with pkgs;
        [
          bash
          bottom
          coreutils
          findutils
          gawk
          gnugrep
          less
          nettools
          nix
          procps
        ] ++ containers-tools-minimal;
    in rec {
      # Dali devnet
      devnet-dali-centauri-1 = (pkgs.callPackage devnetTools.mk-devnet {
        inherit (packages) polkadot-launch composable-node polkadot-node;
        chain-spec = "dali-dev";
      }).script;

      devnet-dali-centauri-2 = (pkgs.callPackage devnetTools.mk-devnet {
        inherit (packages) polkadot-launch composable-node polkadot-node;
        network-config-path =
          ./scripts/polkadot-launch/rococo-local-dali-dev-2.nix;
        chain-spec = "dali-dev";
      }).script;

      # Centauri Persistent Devnet
      devnet-centauri = pkgs.composable.mkDevnetProgram "devnet-centauri"
        (import ./.nix/devnet-specs/centauri.nix {
          inherit pkgs;
          devnet-1 = devnet-dali-centauri-1;
          devnet-2 = devnet-dali-centauri-2;
        });

      # Dali bridge devnet
      bridge-devnet-dali = (devnetTools.mk-bridge-devnet {
        inherit packages;
        inherit (packages) polkadot-launch composable-node polkadot-node;
      }).script;

      # Dali bridge devnet with mmr-polkadot
      bridge-mmr-devnet-dali = (devnetTools.mk-bridge-devnet {
        inherit packages;
        inherit (packages) polkadot-launch composable-node;
        polkadot-node = packages.mmr-polkadot-node;
      }).script;

      # Picasso devnet
      devnet-picasso = (pkgs.callPackage devnetTools.mk-devnet {
        inherit (packages) polkadot-launch composable-node polkadot-node;
        chain-spec = "picasso-dev";
      }).script;

      devnet-container = devnetTools.mk-devnet-container {
        inherit container-tools;
        containerName = "composable-devnet-container";
        devNet = packages.zombienet-rococo-local-dali-dev;
      };

      # Dali Bridge devnet container
      bridge-devnet-dali-container = devnetTools.mk-devnet-container {
        inherit container-tools;
        containerName = "composable-bridge-devnet-container";
        devNet = packages.bridge-devnet-dali;
      };

      # Dali Bridge devnet container with mmr-polkadot
      bridge-mmr-devnet-dali-container = devnetTools.mk-devnet-container {
        inherit container-tools;
        containerName = "composable-bridge-mmr-devnet-container";
        devNet = packages.bridge-mmr-devnet-dali;
      };

      devnet-rococo-dali-karura = let
        config = (pkgs.callPackage
          ./scripts/polkadot-launch/kusama-local-dali-dev-karura-dev.nix {
            polkadot-bin = packages.polkadot-node;
            composable-bin = packages.composable-node;
            acala-bin = packages.acala-node;
          }).result;
        config-file = pkgs.writeTextFile {
          name = "kusama-local-dali-dev-karura-dev.json";
          text = "${builtins.toJSON config}";
        };
      in pkgs.writeShellApplication {
        name = "run-rococo-dali-karura";
        text = ''
          cat ${config-file}
          rm -rf /tmp/polkadot-launch
          ${packages.polkadot-launch}/bin/polkadot-launch ${config-file} --verbose
        '';
      };

      devnet-picasso-complete = let
        config = (pkgs.callPackage ./scripts/polkadot-launch/all-dev-local.nix {
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
        config = (pkgs.callPackage ./scripts/polkadot-launch/all-dev-local.nix {
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
        (import ./.nix/devnet-specs/default.nix {
          inherit pkgs;
          price-feed = packages.price-feed;
          devnet = packages.devnet-dali-complete;
          frontend = packages.frontend-static;
        });

      devnet-xcvm = pkgs.composable.mkDevnetProgram "devnet-xcvm"
        (import ./.nix/devnet-specs/xcvm.nix {
          inherit pkgs;
          devnet-dali = packages.zombienet-rococo-local-dali-dev;
        });

      devnet-dali-persistent =
        pkgs.composable.mkDevnetProgram "devnet-dali-persistent"
        (import ./.nix/devnet-specs/default.nix {
          inherit pkgs;
          price-feed = packages.price-feed;
          devnet = packages.devnet-dali-complete;
          frontend = packages.frontend-static-persistent;
        });

      devnet-picasso-persistent =
        pkgs.composable.mkDevnetProgram "devnet-picasso-persistent"
        (import ./.nix/devnet-specs/default.nix {
          inherit pkgs;
          price-feed = packages.price-feed;
          devnet = packages.devnet-picasso-complete;
          frontend = packages.frontend-static-picasso-persistent;
        });

      kusama-picasso-karura-devnet = let
        config = (pkgs.callPackage
          ./scripts/polkadot-launch/kusama-local-picasso-dev-karura-dev.nix {
            polkadot-bin = packages.polkadot-node;
            composable-bin = packages.composable-node;
            acala-bin = packages.acala-node;
          }).result;
        config-file = pkgs.writeTextFile {
          name = "kusama-local-picasso-dev-karura-dev.json";
          text = "${builtins.toJSON config}";
        };
      in pkgs.writeShellApplication {
        name = "kusama-picasso-karura";
        text = ''
          cat ${config-file}
          rm -rf /tmp/polkadot-launch
          ${packages.polkadot-launch}/bin/polkadot-launch ${config-file} --verbose
        '';
      };
    };
  };
}