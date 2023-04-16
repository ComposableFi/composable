{ self, ... }: {
  perSystem =
    { config
    , self'
    , inputs'
    , pkgs
    , lib
    , system
    , crane
    , systemCommonRust
    , subnix
    , ...
    }:
    let
      _cargo-debug-attrs = {
        CARGO_LOG = "debug";
        CARGO_NET_GIT_FETCH_WITH_CLI = "true";
        CARGO_HTTP_MULTIPLEXING = "false";
        CARGO_HTTP_DEBUG = "true";
        RUST_LOG = "debug";
      };
      buildPolkadotNode =
        { name, repo, owner, rev, hash, cargoSha256 }:
        pkgs.rustPlatform.buildRustPackage (subnix.subenv // rec {
          inherit name cargoSha256;
          # git may be different locally and remotely, so we freeze for determinism (because node builds wasm)
          buildPackage = [ pkgs.git ];
          src = pkgs.fetchgit {
            url = "https://github.com/${owner}/${repo}.git";
            inherit rev;
            sha256 = hash;
            fetchSubmodules = false;
          };
          # env = _cargo-debug-attrs;
          meta = { mainProgram = "polkadot"; };

          __noChroot = true;

        });
      cargo-lock = builtins.fromTOML (builtins.readFile ../../code/Cargo.lock);
      rococo-runtime-dep = builtins.head
        (builtins.filter (x: x.name == "rococo-runtime") (cargo-lock.package));
      rococo-runtime-commit =
        builtins.elemAt (builtins.split "#" rococo-runtime-dep.source) 2;
    
#      mkRelayКгтешьу = 

    in
    {
      packages = rec {
        # intentionally defined each env separately because it can  evolve/tested/deployed separately
        rococo-runtime-from-dep = pkgs.stdenv.mkDerivation {
          name = "rococo-runtime-from-dep";
          dontUnpack = true;
          src = pkgs.fetchurl {
            url =
              "https://github.com/paritytech/polkadot/releases/download/v0.9.36/rococo_runtime-v9360.compact.compressed.wasm";
            hash = "sha256-inq526PxU2f4+m4RSTiv5oOpfSZfnQpXkhpYmqZ9gOs=";
          };
          installPhase = ''
            mkdir -p $out/lib
            cp $src $out/lib/relay_runtime.compact.compressed.wasm
          '';
        };



      kusama-runtime-on-parity = pkgs.stdenv.mkDerivation {
          name = "kusama-runtime-on-parity";
          dontUnpack = true;
          src = pkgs.fetchurl {
            url =
              "https://github.com/paritytech/polkadot/releases/download/v0.9.38-rc12/kusama_runtime-v9381.compact.compressed.wasm";
            hash = "sha256-LTKYGGMyQj+hzpp+9DvpPExzwiukRHQBA+e4DDAjBto=";
          };
          installPhase = ''
            mkdir -p $out/lib
            cp $src $out/lib/relay_runtime.compact.compressed.wasm
          '';
        };


      polkadot-runtime-on-parity = pkgs.stdenv.mkDerivation {
          name = "polkadot-runtime-on-parity";
          dontUnpack = true;
          src = pkgs.fetchurl {
            url =
              "https://github.com/paritytech/polkadot/releases/download/v0.9.37/polkadot_runtime-v9370.compact.compressed.wasm";
            hash = "sha256-n8+2GpqqU/kHderUqea4Q7yv4UmsESw25laH1/oZryE=";
          };
          installPhase = ''
            mkdir -p $out/lib
            cp $src $out/lib/relay_runtime.compact.compressed.wasm
          '';
        };

        

        polkadot-node-from-dep = buildPolkadotNode rec {
          name = "polkadot-node-from-dep";
          repo = "polkadot";
          owner = "paritytech";
          rev = rococo-runtime-commit;
          hash = "sha256-24UcJTnbVDe8oW8S0stayHc7/vVyFQaqTSSPHNqJXkg=";
          cargoSha256 = "sha256-24UcJTnbVDe8oW8S0stayHc7/vVyFQaqTSSPHNqJXkg=";
        };


        polkadot-node-on-parity-rococo = buildPolkadotNode rec {
          name = "polkadot-node-on-parity-rococo";
          repo = "polkadot";
          owner = "paritytech";
          rev = "e203bfb396ed949f102720debf32fb98166787af";
          hash = "sha256-+rGrAyQH//m6xFiUstDiZKhvHq928rs36TajT/QxrKM=";
          cargoSha256 = "sha256-+rGrAyQH//m6xFiUstDiZKhvHq928rs36TajT/QxrKM=";
        };

        polkadot-node-on-parity-westend = buildPolkadotNode rec {
          name = "polkadot-node-on-parity-westend";
          repo = "polkadot";
          owner = "paritytech";
          rev = "e203bfb396ed949f102720debf32fb98166787af";
          hash = "sha256-+rGrAyQH//m6xFiUstDiZKhvHq928rs36TajT/QxrKM=";
          cargoSha256 = "sha256-+rGrAyQH//m6xFiUstDiZKhvHq928rs36TajT/QxrKM=";
        };

        polkadot-node-on-parity-kusama = buildPolkadotNode rec {
          name = "polkadot-node-on-parity-kusama";
          repo = "polkadot";
          owner = "paritytech";
          rev = "e203bfb396ed949f102720debf32fb98166787af";
          hash = "sha256-+rGrAyQH//m6xFiUstDiZKhvHq928rs36TajT/QxrKM=";
          cargoSha256 = "sha256-+rGrAyQH//m6xFiUstDiZKhvHq928rs36TajT/QxrKM=";
        };

        polkadot-node-on-parity-polkadot = buildPolkadotNode rec {
          name = "polkadot-node-on-parity-polkadot";
          repo = "polkadot";
          owner = "paritytech";
          rev = "e203bfb396ed949f102720debf32fb98166787af";
          hash = "sha256-+rGrAyQH//m6xFiUstDiZKhvHq928rs36TajT/QxrKM=";
          cargoSha256 = "sha256-+rGrAyQH//m6xFiUstDiZKhvHq928rs36TajT/QxrKM=";
        };
      };
    };
}
