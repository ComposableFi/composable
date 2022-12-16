{ pkgs, rust-nightly }:
pkgs.callPackage ./polkadot-tmpl.nix rec {
  inherit pkgs rust-nightly;
  name = "polkadot-v${version}";
  version = "0.9.27";
  repo = "polkadot";
  owner = "paritytech";
  rev = "v${version}";
  hash = "sha256-LEz3OrVgdFTCnVwzU8C6GeEougaOl2qo7jS9qIdMqAM=";
  cargoSha256 = "sha256-6y+WK2k1rhqMxMjEJhzJ26WDMKZjXQ+q3ca2hbbeLvA=";
}
