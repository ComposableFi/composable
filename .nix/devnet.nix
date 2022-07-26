{ pkgs, nixpkgs, devnet-input, gce-input}:
  assert gce-input != null;
  assert gce-input.project != null;
  assert gce-input.serviceAccount != null;
  assert gce-input.accessKey != null;
let
  description = "Derives Dali and Picasso releases from remote branches with relevant remote depoyments and documentation";
  mk-parachain = binary:
    chain: binary // {
      inherit chain;
      nodes = [
        {
          name = "alice";
          wsPort = 9988;
          port = 31200;
        }
        {
          name = "bob";
          wsPort = 9989;
          port = 31201;
        }
        {
          name = "charlie";
          wsPort = 9990;
          port = 31202;
        }
      ];
    };
  mk-relaychain = binary:
    chain: binary // {
      inherit chain;
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
          wsPort = 9956;
          port = 30666;
        }
      ];
    };
  mk-latest = chain:
    ({ composable, polkadot }: {
      composable = mk-parachain composable chain;
      polkadot = mk-relaychain polkadot "rococo-local" ;
    }) devnet-input;
  latest-dali-rococo = mk-latest "dali-dev";
  latest-picasso-rococo = mk-latest "picasso-dev";
in rec {
  dali = (pkgs.callPackage ./devnet-spec.nix {
    inherit (latest-dali-rococo) composable;
    inherit (latest-dali-rococo) polkadot;
  });

  picasso = (pkgs.callPackage ./devnet-spec.nix {
    inherit (latest-picasso-rococo) composable;
    inherit (latest-picasso-rococo) polkadot;
  });
 
  machines =
   builtins.foldl' (machines:
    devnet:
    machines // import ./devnet-gce.nix {
      inherit gce-input;
      inherit devnet;
    }) {
      inherit nixpkgs;
      network.description = "Composable Devnet";
      network.storage.legacy = { };
    } [ dali picasso ];
}