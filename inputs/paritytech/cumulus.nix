{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, crane
    , systemCommonRust, subnix, ... }:
    let
      cargo-lock = builtins.fromTOML (builtins.readFile ../../code/Cargo.lock);
      bin = "polkadot-parachain";
      polkadot-parachain-dep = builtins.head
        (builtins.filter (x: x.name == "parachain-info") (cargo-lock.package));
      polkadot-parachain-commit =
        builtins.elemAt (builtins.split "#" polkadot-parachain-dep.source) 2;
    in {
      packages = {
        polkadot-parachain = pkgs.stdenv.mkDerivation (rec {
          name = bin;
          pname = bin;
          src = pkgs.fetchgit {
            url = "https://github.com/paritytech/cumulus.git";
            rev = polkadot-parachain-commit;
            sha256 = "sha256-nFC1+sEbXm89Vc+Fqz9uzJCfFTqqVGCZP4jR3TqBjDY=";
            fetchSubmodules = false;
          };
          __noChroot = true;
          configurePhase = ''
            mkdir home
            export HOME=$PWD/home
            export WASM_TARGET_DIRECTORY=$PWD/home
          '';
          buildPhase = ''
            cargo build --release --locked --bin ${bin} --no-default-features
          '';
          installPhase = ''
            mkdir --parents $out/bin && mv ./target/release/${bin} $out/bin
          '';
        } // subnix.subenv // {
          CARGO_NET_GIT_FETCH_WITH_CLI = "false";
        });
      };
    };
}
