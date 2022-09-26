{ nixpkgs, devnet-dali, devnet-picasso, book, gce-input }:
let
  persistent-machine = import ./devnet-gce.nix {
    inherit gce-input;
    inherit book;
    devnet = devnet-dali;
    disk-size = 200;
    machine-name = "composable-persistent-devnet";
    domain = "persistent.devnets.composablefinance.ninja";
  };
in builtins.foldl' (machines: devnet:
  let
    machine = import ./devnet-gce.nix {
      inherit gce-input;
      inherit devnet;
      inherit book;
      disk-size = 200;
      machine-name = "composable-devnet-${devnet.chain-spec}";
      domain = let prefix = nixpkgs.lib.removeSuffix "-dev" devnet.chain-spec;
      in "${prefix}.devnets.composablefinance.ninja";
    };
  in machines // machine) ({
    inherit nixpkgs;
    network = {
      description = "Composable Devnet";
      storage.legacy = { };
    };
  } // persistent-machine) [ devnet-dali devnet-picasso ]
