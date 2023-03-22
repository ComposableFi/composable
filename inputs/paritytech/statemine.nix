{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, crane
    , systemCommonRust, subTools, ... }: {
      packages = {
        statemine-node = let version = "release-parachains-v9380";
        in pkgs.stdenv.mkDerivation (rec {
          name = "statemine-node";
          inherit version;
          pname = "polkadot-parachain";
          src = pkgs.fetchgit {
            url = "https://github.com/paritytech/cumulus.git";
            rev = "refs/heads/${version}";
            sha256 = "sha256-Arc7eK9RekqbSDyHZRLxfyqLBCNUgO12YwwI3XxcXR4=";
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
        } // subTools.subenv);
      };
    };
}
