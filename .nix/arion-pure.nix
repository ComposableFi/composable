{ pkgs, packages, ... }:
pkgs.arion.build {
  modules = [
    ({ pkgs, ... }: {
      config.project.name = "Composable Finance devnet";
      config.services = {
        devnet-dali = import ./services/devnet-dali.nix {
          inherit pkgs;
          inherit packages;
        };
        subsquid-db = import ./services/subsquid-db.nix;
        subsquid-db-archive = import ./services/subsquid-db-archive.nix;
        subsquid-indexer = import ./services/subsquid-indexer.nix;
        subsquid-indexer-gateway =
          import ./services/subsquid-indexer-gateway.nix;
        subsquid-indexer-status-service =
          import ./services/subsquid-indexer-status-service.nix;
        subsquid-redis = import ./services/subsquid-redis.nix;
        subsquid-processor = import ./services/subsquid-processor.nix {
          inherit pkgs;
          inherit packages;
        };
      };
    })
  ];
  inherit pkgs;
}

