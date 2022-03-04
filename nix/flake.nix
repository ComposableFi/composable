{
  description = "Composable Devnet Scripts";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/ec7d9d4c182c54acd649674cf1023d40a0539eb7";
    flake-utils.url = "github:numtide/flake-utils/3cecb5b042f7f209c56ffd8371b2711a290ec797";
    localtunnel-src = {
      url = "github:localtunnel/localtunnel/c8e85f49624d606730779fc4295a38fd0e650af5";
      flake = false;
    };
  };
  outputs = { nixpkgs, flake-utils, localtunnel-src, ... }:
    let
      bins =
        (if builtins.pathExists ./devnet.json
         then builtins.fromJSON (builtins.readFile ./devnet.json)
         else throw "Devnet `devnet.json` definition missing, please follow the README.md instructions.");
      mk-composable = spec: { name, version, hash }: {
        inherit name;
        inherit version;
        inherit spec;
        inherit hash;
        nodes = [{
          name = "alice";
          wsPort = 9944;
          port = 30444;
        } {
          name = "bob";
          wsPort = 9955;
          port = 30555;
        } {
          name = "charlie";
          wsPort = 9966;
          port = 30666;
        } {
          name = "dave";
          wsPort = 9977;
          port = 30777;
        }];
      };
      mk-polkadot = spec: { version, hash }: {
        inherit version;
        inherit spec;
        inherit hash;
        nodes = [{
          name = "alice";
          wsPort = 9988;
          port = 31100;
        } {
          name = "bob";
          wsPort = 9997;
          port = 31200;
        } {
          name = "charlie";
          wsPort = 9996;
          port = 31300;
        }];
      };
      mk-latest = spec:
        ({ composable, polkadot }: {
          composable = mk-composable spec composable;
          polkadot = mk-polkadot "rococo-local" polkadot;
        }) bins;
      latest-dali = mk-latest "dali-dev";
      latest-picasso = mk-latest "picasso-dev";
    in
    {
      nixopsConfigurations.default =
        let
          pkgs-nixos = import nixpkgs {};
          conf = if builtins.pathExists ./ops.json
                 then builtins.fromJSON (builtins.readFile ./ops.json)
                 else throw "Operations credentials `ops.json` definition missng, please follow the README.md instructions.";
          credentials = {
            project = conf.project_id;
            serviceAccount = conf.client_email;
            accessKey = conf.private_key;
          };
          localtunnel = pkgs-nixos.mkYarnPackage rec {
            name = "localtunnel";
            src = localtunnel-src;
          };
        in
          builtins.foldl' (machines: { composable, polkadot }: machines // import ./devnet-gce.nix {
            inherit localtunnel;
            inherit credentials;
            inherit composable;
            inherit polkadot;
          }) {
            inherit nixpkgs;
            network.description = "Composable Devnet";
            network.storage.legacy = {};
          } [ latest-dali latest-picasso ];
    } //
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = import nixpkgs { inherit system; };
        in rec {
          packages.devnet-dali = (pkgs.callPackage ./devnet.nix {
            inherit (latest-dali) composable;
            inherit (latest-dali) polkadot;
          }).script;
          packages.devnet-picasso = (pkgs.callPackage ./devnet.nix {
            inherit (latest-picasso) composable;
            inherit (latest-picasso) polkadot;
          }).script;
          packages.localtunnel = pkgs.mkYarnPackage rec {
            name = "localtunnel";
            src = localtunnel-src;
          };
          packages.deploy = pkgs.mkShell {
            buildInputs = [
              packages.devnet-dali
              packages.devnet-picasso
              packages.localtunnel
              (pkgs.nixopsUnstable.override {
                overrides = (self: super: {
                  # FIXME: probably useless once 2.0 is stable
                  nixops = super.nixops.overridePythonAttrs (
                    _: {
                      src = pkgs.fetchgit {
                        url = "https://github.com/NixOS/nixops";
                        rev = "35ac02085169bc2372834d6be6cf4c1bdf820d09";
                        sha256 = "1jh0jrxyywjqhac2dvpj7r7isjv68ynbg7g6f6rj55raxcqc7r3j";
                      };
                    }
                  );
                });
            })];
            # NOTE: nixops depends on nixpkgs for the virtual machine initial conf...
            NIX_PATH = "nixpkgs=${pkgs.path}";
          };
          defaultPackage = packages.devnet-dali;
          devShell = pkgs.mkShell {
            buildInputs = [
              packages.devnet-dali
              packages.devnet-picasso
            ];
          };
        }
      );
}
