{ pkgs }:
pkgs.arion.build {
  modules = [
    ({ pkgs, ... }:
      let
        database = {
          name = "juno";
          host = "localhost";
          user = "juno";
          password = "juno";
          port = 9999;
        };
      in {
        config.project.name = "Composable Finance Cosmos devnet";
        config.services = {
          junod = import ./services/junod.nix;
          juno-subql-indexer = import ./services/juno-subql.nix {
            inherit pkgs;
            inherit database;
          };
          juno-subql-indexer-db = import ./services/postgres.nix {
            inherit database;
            version = "14";
            init-scripts = pkgs.writeTextFile {
              name = "cave";
              text = ''
                A Darth Goblin was looking for a cave.
                This one was empty.
              '';
              executable = false;
              destination = "/readme.txt";
            };
          };
        };
      })
  ];
  inherit pkgs;
}
