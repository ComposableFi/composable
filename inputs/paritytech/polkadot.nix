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
            cp $src $out/lib/rococo_runtime.compact.compressed.wasm
          '';
        };

        polkadot-node-from-dep = buildPolkadotNode rec {
          name = "polkadot-node-from-dep";
          repo = "polkadot";
          owner = "paritytech";
          rev = rococo-runtime-commit;
          hash = "sha256-24UcJTnbVDe8oW1S0stayHc7/vVyFQaqTSSPHNqJXkg=";
          cargoSha256 = "sha256-OA0m9b3opPahHfsOMJylmstu6XmmCXC60T1e56uMqyE=";
        };


        polkadot-node-on-parity-rococo = buildPolkadotNode rec {
          name = "polkadot-node-on-parity-rococo";
          repo = "polkadot";
          owner = "paritytech";
          rev = "e203bfb396ed949f102720debf32fb98166787af";
          hash = "sha256-++aSGovKRE4+1hRoDqo6lSO1aenNrdvkVqaIXz4s0bk=";
          cargoSha256 = "sha256-RG/FvtrMCJB2BbMosSPlGJCKmIbRJT7ZUDkj1dVKWKg=";
        };

        polkadot-node-on-parity-westend = buildPolkadotNode rec {
          name = "polkadot-node-on-parity-westend";
          repo = "polkadot";
          owner = "paritytech";
          rev = "e203bfb396ed949f102720debf32fb98166787af";
          hash = "sha256-++aSGovKRE4+1hRoDqo1lSO4aenNrdvkVqaIXz4s0bk=";
          cargoSha256 = "sha256-RG/FvtrMCJB2BbMosSPlGJCKmIbRJT7ZUDkj1dVKWKg=";
        };

        polkadot-node-on-parity-kusama = buildPolkadotNode rec {
          name = "polkadot-node-on-parity-kusama";
          repo = "polkadot";
          owner = "paritytech";
          rev = "e203bfb396ed949f102720debf32fb98166787af";
          hash = "sha256-++aSGovKRE4+1hRoDqo1lSO4aenNrdvkVqaIXz4s0bk=";
          cargoSha256 = "sha256-RG/FvtrMCJB2BbMosSPlGJCKmIbRJT7ZUDkj1dVKWKg=";
        };

        polkadot-node-on-parity-polkadot = buildPolkadotNode rec {
          name = "polkadot-node-on-parity-polkadot";
          repo = "polkadot";
          owner = "paritytech";
          rev = "e203bfb396ed949f102720debf32fb98166787af";
          hash = "sha256-++aSGovKRE4+1hRoDqo1lSO4aenNrdvkVqaIXz4s0bk=";
          cargoSha256 = "sha256-RG/FvtrMCJB2BbMosSPlGJCKmIbRJT7ZUDkj1dVKWKg=";
        };
      };
    };
}
