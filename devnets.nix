{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, devnetTools, ... }: {
    packages = let
      packages = self'.packages;
      docker-in-docker = with pkgs; [ docker docker-buildx docker-compose ];
      baseContainerTools = with pkgs; [ bash coreutils procps ];
      userContainerTools = with pkgs; [ acl direnv home-manager cachix ];
      container-tools = with pkgs;
        [ bottom findutils gawk gnugrep less nettools nix ]
        ++ userContainerTools ++ baseContainerTools;
    in rec {
      devnet-centauri = pkgs.composable.mkDevnetProgram "devnet-centauri"
        (import ./.nix/devnet-specs/centauri.nix {
          inherit pkgs;
          devnet-1 = packages.devnet-dali-centauri-a;
          devnet-2 = packages.devnet-dali-centauri-b;
        });

      devnet-container = devnetTools.mk-devnet-container {
        inherit container-tools;
        containerName = "composable-devnet-container";
        devNet = packages.zombienet-rococo-local-dali-dev;
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
    };
  };
}
