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
      packages = {
        polkadot-node = let version = "v0.9.38";
        in buildPolkadotNode rec {
          name = "polkadot-node";
          inherit version;
          repo = "polkadot";
          owner = "paritytech";
          rev = "refs/tags/${version}";
          hash = "sha256-byUC+6SQ+TgvQCXdQWIGf/BAyxitnT1q69RdyZL8AAc=";
          cargoSha256 = "sha256-BYJzMagEhXEa6rjy862ESBZW2FQXopRTyRESefa4rqo=";
        };
      };
    };
}
