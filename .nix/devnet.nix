{ pkgs, nixpkgs, devnet-input, gce-input }:
let
  assert gce-input.project_id != null "Remote credentials must be supplied"
  assert gce-input.client_email != null "Remote credentials must be supplied"
  assert gce-input.private_key != null "Remote credentials must be supplied"

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

  nixops = (pkgs.nixopsUnstable.override {
    overrides = (self: super: {
      # FIXME: probably useless once 2.0 is stable
      nixops = super.nixops.overridePythonAttrs (_: {
        src = pkgs.fetchgit {
          url = "https://github.com/NixOS/nixops";
          rev = "35ac02085169bc2372834d6be6cf4c1bdf820d09";
          sha256 = "1jh0jrxyywjqhac2dvpj7r7isjv68ynbg7g6f6rj55raxcqc7r3j";
        };
      });
    });
  });

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