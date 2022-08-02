{ pkgs, nixpkgs, devnet-input, gce-input }:
  assert gce-input != null;
  assert gce-input.project_id != null;
  assert gce-input.client_email != null;
  assert gce-input.private_key != null;
let
  description = "Derives Dali and Picasso releases from remote branches with relevant remote depoyments and documentation";
  mk-composable = spec:
    def: def // {
      inherit spec;
      nodes = [
        {
          name = "alice";
          wsPort = 9944;
          port = 30444;
        }
        {
          name = "bob";
          wsPort = 9955;
          port = 30555;
        }
        {
          name = "charlie";
          wsPort = 9966;
          port = 30666;
        }
        {
          name = "dave";
          wsPort = 9977;
          port = 30777;
        }
      ];
    };
  mk-polkadot = spec:
    def: def // {
      inherit spec;
      nodes = [
        {
          name = "alice";
          wsPort = 9988;
          port = 31100;
        }
        {
          name = "bob";
          wsPort = 9997;
          port = 31200;
        }
        {
          name = "charlie";
          wsPort = 9996;
          port = 31300;
        }
      ];
    };
  mk-latest = spec:
    ({ composable, polkadot }: {
      composable = mk-composable spec composable;
      polkadot = mk-polkadot "rococo-local" polkadot;
    }) devnet-input;
  latest-dali = mk-latest "dali-dev";
  latest-picasso = mk-latest "picasso-dev";
in rec {
  dali = (pkgs.callPackage ./devnet-spec.nix {
    inherit (latest-dali) composable;
    inherit (latest-dali) polkadot;
  });

  picasso = (pkgs.callPackage ./devnet-spec.nix {
    inherit (latest-picasso) composable;
    inherit (latest-picasso) polkadot;
  });

  nixops = pkgs.callPackage ./nixops.nix {};

  machines = let
    credentials = {
      project = gce-input.project_id;
      serviceAccount = gce-input.client_email;
      accessKey = gce-input.private_key;
    };
  in builtins.foldl' (machines:
    { composable, polkadot }:
    machines // import ./devnet-gce.nix {
      inherit gce-input;
      inherit composable;
      inherit polkadot;
      devnet-spec = dali;
    }) {
      inherit nixpkgs;
      network.description = "Composable Devnet";
      network.storage.legacy = { };
    } [ latest-dali latest-picasso ];
}