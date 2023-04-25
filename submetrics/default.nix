let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
        openssl
    ];
    nativeBuildInputs = with pkgs; [
        pkg-config
    ];
  }