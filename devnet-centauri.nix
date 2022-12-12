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
      devnet-dali = (pkgs.callPackage devnetTools.mk-devnet {
        inherit (packages) polkadot-launch composable-node polkadot-node;
        chain-spec = "dali-dev";
      }).script;

      devnet-dali-2 = (pkgs.callPackage devnetTools.mk-devnet {
        inherit (packages) polkadot-launch composable-node polkadot-node;
        network-config-path =
          ./scripts/polkadot-launch/rococo-local-dali-dev-2.nix;
        chain-spec = "dali-dev";
      }).script;

      devnet-centauri = pkgs.composable.mkDevnetProgram "devnet-centauri"
        (import ./.nix/devnet-specs/centauri.nix {
          inherit pkgs;
          devnet-1 = devnet-dali;
          devnet-2 = devnet-dali-2;
        });
    };
  };
}
