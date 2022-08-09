{ pkgs, nixpkgs, devnet-dali, devnet-picasso, book, gce-input }:
assert gce-input != null;
assert gce-input.project != null;
assert gce-input.serviceAccount != null;
assert gce-input.accessKey != null;
let
  description =
    "Derives Dali and Picasso releases from remote branches with relevant remote depoyments and documentation";
in builtins.foldl' (machines: devnet:
  machines // import ./devnet-gce.nix {
    inherit gce-input;
    inherit devnet;
    inherit book;
  }) {
    inherit nixpkgs;
    network.description = "Composable Devnet";
    network.storage.legacy = { };
  } [ devnet-dali devnet-picasso ]
