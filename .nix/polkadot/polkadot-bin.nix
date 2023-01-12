{ pkgs, rust-nightly }:
pkgs.callPackage ./polkadot-tmpl.nix rec {
  inherit pkgs rust-nightly;
  name = "polkadot-v${version}";
  version = "0.9.30";
  repo = "polkadot";
  owner = "paritytech";
  rev = "v${version}";
  hash = "sha256-3hmoTTzdvC1s0GsfgEz6vaIh/obx+MHCqjnUJR6NRVk=";
  cargoSha256 = "sha256-YzQspHuHDWOeh9xvFIy0BF6xiep3pr8QEiJDK9Vb8fg=";
}
