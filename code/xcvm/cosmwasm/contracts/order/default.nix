{ nixpkgs ? import <nixpkgs> { } }:

let
  rustOverlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [ (import rustOverlay) ];
  };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rust-bin.nightly."2023-10-06".default
    rust-analyzer
  ];

  RUST_BACKTRACE = 1;
}
