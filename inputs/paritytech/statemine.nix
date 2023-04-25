{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, crane
    , systemCommonRust, subnix, ... }: {
      packages = {
        statemine-node = pkgs.stdenv.mkDerivation (rec {
          name = "statemine-node";
          pname = "polkadot-parachain";
          src = pkgs.fetchgit {
            url = "https://github.com/paritytech/cumulus.git";
            rev = "9b4e0247137f158d1a35118197d34adfa58858b7";
            sha256 = "sha256-Ble9E7wWzQ3W801BLfBtDRyJQs/3uU4hhaAEAbyAJxg=";
            fetchSubmodules = false;
          };
          __noChroot = true;
          configurePhase = ''
            mkdir home
            export HOME=$PWD/home
            export WASM_TARGET_DIRECTORY=$PWD/home
          '';
          buildPhase = ''
            cargo build --release --locked --bin polkadot-parachain --no-default-features
          '';
          installPhase = ''
            mkdir --parents $out/bin && mv ./target/release/polkadot-parachain $out/bin
          '';
        } // subnix.subenv // {
          CARGO_NET_GIT_FETCH_WITH_CLI = "false";
        });
      };
    };
}
