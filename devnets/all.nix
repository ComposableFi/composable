{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, devnetTools, ... }: {
    packages = let packages = self'.packages;
    in rec {
      centauri-configure-and-run = pkgs.writeShellApplication rec {
        name = "centauri-configure-and-run";
        text = ''
          cp --force ${self'.packages.hyperspace-config} /tmp/config.toml  
          ${pkgs.lib.meta.getExe devnet-centauri}       
        '';
      };

      devnet-centauri = pkgs.composable.mkDevnetProgram "devnet-centauri"
        (import ./specs/centauri.nix {
          inherit pkgs devnetTools packages;
          devnet-a = packages.zombienet-dali-centauri-a;
          devnet-b = packages.zombienet-dali-centauri-b;
        });

      devnet-dali-image = devnetTools.buildDevnetImage {
        name = "devnet-dali";
        container-tools = devnetTools.withDevNetContainerTools;
        devNet = packages.zombienet-rococo-local-dali-dev;
      };

      devnet-picasso-complete = packages.zombienet-picasso-complete;
      devnet-dali-complete = packages.zombienet-dali-complete;
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
          inherit pkgs devnetTools;
          price-feed = packages.price-feed;
          devnet = packages.devnet-picasso-complete;
          frontend = packages.frontend-static-picasso-persistent;
        });
    };
  };
}
