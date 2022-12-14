{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, crane, systemCommonRust, ... }:
  {
    packages = {
      mmr-polkadot-node = pkgs.callPackage ./polkadot-tmpl.nix rec {
        inherit pkgs;
        rust-nightly = self'.packages.rust-nightly;
        name = "mmr-polkadot-v${version}";
        version = "0.9.27";
        repo = "polkadot";
        owner = "ComposableFi";
        rev = "0898082540c42fb241c01fe500715369a33a80de";
        hash = "sha256-dymuSVQXzdZe8iiMm4ykVXPIjIZd2ZcAOK7TLDGOWcU=";
        cargoSha256 = "sha256-u/hFRxt3OTMDwONGoJ5l7whC4atgpgIQx+pthe2CJXo=";
      };

      polkadot-node = pkgs.callPackage ./polkadot-tmpl.nix rec {
        inherit pkgs;
        rust-nightly = self'.packages.rust-nightly;
        name = "polkadot-v${version}";
        version = "0.9.27";
        repo = "polkadot";
        owner = "paritytech";
        rev = "v${version}";
        hash = "sha256-LEz3OrVgdFTCnVwzU8C6GeEougaOl2qo7jS9qIdMqAM=";
        cargoSha256 = "sha256-6y+WK2k1rhqMxMjEJhzJ26WDMKZjXQ+q3ca2hbbeLvA=";
      };
    };
  };
}
