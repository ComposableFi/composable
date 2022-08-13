{ nixpkgs, devnet-dali, devnet-picasso, book, gce-input }:
let
  description =
    "Derives Dali and Picasso releases from remote branches with relevant remote depoyments and documentation";
in builtins.foldl' (machines: devnet:
  let
    machine = import ./devnet-gce.nix {
      inherit gce-input;
      inherit devnet;
      inherit book;
    };
  in machines // machine) {
    inherit nixpkgs;
    network = {
      description = "Composable Devnet";
      storage.legacy = { };
    };
  } [ devnet-dali devnet-picasso ]
