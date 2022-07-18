{ pkgs, nixpkgs }:
let
  bins = (if builtins.pathExists ./devnet-stage.json then
    builtins.fromJSON (builtins.readFile ./devnet-stage.json)
  else
    throw
    "Devnet `devnet-stage` definition missing, please follow the README.md instructions.");
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
    }) bins;
  latest-dali = mk-latest "dali-dev";
  latest-picasso = mk-latest "picasso-dev";
in {
  dali = (pkgs.callPackage ./devnet-stage.nix {
    inherit (latest-dali) composable;
    inherit (latest-dali) polkadot;
  });

  picasso = (pkgs.callPackage ./devnet-stage.nix {
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
    conf = if builtins.pathExists ./ops.json then
      builtins.fromJSON (builtins.readFile ./ops.json)
    else
      throw
      "Operations credentials `ops.json` definition missng, please follow the README.md instructions.";
    credentials = {
      project = conf.project_id;
      serviceAccount = conf.client_email;
      accessKey = conf.private_key;
    };
  in builtins.foldl' (machines:
    { composable, polkadot }:
    machines // import ./devnet-gce.nix {
      inherit credentials;
      inherit composable;
      inherit polkadot;
    }) {
      inherit nixpkgs;
      network.description = "Composable Devnet";
      network.storage.legacy = { };
    } [ latest-dali latest-picasso ];
}
