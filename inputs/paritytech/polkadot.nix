{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, crane
    , systemCommonRust, subnix, ... }:
    let
      _cargo-debug-attrs = {
        CARGO_LOG = "debug";
        CARGO_NET_GIT_FETCH_WITH_CLI = "true";
        CARGO_HTTP_MULTIPLEXING = "false";
        CARGO_HTTP_DEBUG = "true";
        RUST_LOG = "debug";
      };
      buildPolkadotNode =
        { name, version, repo, owner, rev, hash, cargoSha256 }:
        pkgs.rustPlatform.buildRustPackage (subnix.subenv // rec {
          inherit name version cargoSha256;
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
    in {
      packages = rec {
        rococo-wasm-runtime-9360 = pkgs.stdenv.mkDerivation {
          name = "rococo-wasm-runtime";
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
        rococo-wasm-runtime-current = rococo-wasm-runtime-9360;

        polkadot-node-dep = let version = "current";
        in buildPolkadotNode rec {
          name = rococo-runtime-commit;
          inherit version;
          repo = "polkadot";
          owner = "paritytech";
          rev = rococo-runtime-commit;
          hash = "sha256-x2IEIHxH8Hg+jFIpnPrTsqISEAZHFuXhJD+H1S+G3nk=";
          cargoSha256 = "sha256-ZvHdlFpord1uPGsnQlGt4wDdYti07D4tpWuc2HWHtII=";
        };
        # for xcmv3 release and centauri client asap they upgrade
        polkadot-node-9390 = let version = "v0.9.39";
        in buildPolkadotNode rec {
          name = "polkadot-node-next";
          inherit version;
          repo = "polkadot";
          owner = "paritytech";
          rev = "refs/tags/${version}";
          hash = "sha256-++aSGovKRE4+1hRoDqo6lSO4aenNrdvkVqaIXz4s0bk=";
          cargoSha256 = "sha256-RG/FvtrMCJB1BbMosSPlGJCKmIbRJT7ZUDkj1dVKWKg=";
        };

        polkadot-node-on-parity-kusama = polkadot-node-dep;
        polkadot-node-on-parity-polkadot = polkadot-node-dep;
        polkadot-node-on-parity-rococo = polkadot-node-9390;
      };
    };
}
