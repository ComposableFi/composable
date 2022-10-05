{ pkgs, rust-nightly }:
let substrate-attrs = import ./substrate.nix { inherit rust-nightly pkgs; };
in with pkgs;
rustPlatform.buildRustPackage (substrate-attrs // rec {
  name = "polkadot";
  src = fetchFromGitHub {
    repo = "polkadot";
    owner = "paritytech";
    #rev = "v0.9.27";
    rev = "6882eff7961008a54749fa8ed445e40844febc3a";
    #hash = "sha256-LEz3OrVgdFTCnVwzU8C6GeEougaOl2qo7jS9qIdMqAM=";
    hash = "sha256-Fv/auvHytPr/bF4SGK/Cwl7PvWml+ZSchkE7GGPYcaU=";
  };
  cargoHash = "sha256-5R3b7w+l9+F9WMztlxs8Lu1WjgRmZ1HuF6BBXTwfvng=";
  #cargoHash = "sha256-eSZGz7HXjLxQ584OMTVQNBOWPVFBtGWQfuJid489Xm4=";
  meta = { mainProgram = "polkadot"; };
})
