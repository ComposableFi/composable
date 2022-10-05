{ pkgs, rust-nightly }:
let substrate-attrs = import ./substrate.nix { inherit rust-nightly pkgs; };
in with pkgs;
rustPlatform.buildRustPackage (substrate-attrs // rec {
  name = "polkadot";
  src = fetchFromGitHub {
    repo = "polkadot";
    owner = "paritytech";
    rev = "6882eff7961008a54749fa8ed445e40844febc3a";
    hash = "sha256-LEz3OrVgdFTCnVwzU8C6GeEougaOl2qo7jS9qIdMqAM=";
  };
  cargoHash = "sha256-eSZGz7HXjLxQ584OMTVQNBOWPVFBtGWQfuJid489Xm4=";
  meta = { mainProgram = "polkadot"; };
})
