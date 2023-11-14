{pkgs ? import <nixpkgs> {}}: let
  packages = ps:
    with ps; [
    ];
  python = pkgs.python3.withPackages packages;
in
  python.env
