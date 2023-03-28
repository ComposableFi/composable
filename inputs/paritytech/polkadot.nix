{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, crane
    , systemCommonRust, subTools, ... }:
    let
      buildPolkadotNode =
        { name, version, repo, owner, rev, hash, cargoSha256 }:
        pkgs.rustPlatform.buildRustPackage (rec {
          inherit name version cargoSha256;
          src = pkgs.fetchgit {
            url = "https://github.com/${owner}/${repo}.git";
            inherit rev;
            sha256 = hash;
            fetchSubmodules = false;
          };

          meta = { mainProgram = "polkadot"; };

          __noChroot = true;

        } // subTools.subenv);
    in {
      packages = rec {
        # current version for centauri
        polkadot-node-9370 = let version = "v0.9.37";
        in buildPolkadotNode rec {
          name = "polkadot-node-current";
          inherit version;
          repo = "polkadot";
          owner = "paritytech";
          rev = "refs/tags/${version}";
          hash = "sha256-TTi4cKqQT/2ZZ/acGvcilqTlh2D9t4cfAtQQyVZWdmg=";
          cargoSha256 = "sha256-3EcGLJNsbFl9ITJuVIrItp1e6weO9/NMNnMOJngsCP0=";
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

        polkadot-node-on-parity-kusama = polkadot-node-9370;
        polkadot-node-on-parity-polkadot = polkadot-node-9370;
        polkadot-node-on-parity-rococo = polkadot-node-9390;
      };
    };
}
