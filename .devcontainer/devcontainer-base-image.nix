{ pkgs ? import <nixpkgs>, system ? builtins.defaultSystem }:
with pkgs;
let
  nix-to-container-image = import ./devcontainer-base-image-per-system.nix;
in dockerTools.pullImage ((nix-to-container-image system) // {
  imageName = "mcr.microsoft.com/vscode/devcontainers/base";
  os = "linux";
  imageDigest =
    "sha256:269cbbb2056243e2a88e21501d9a8166d1825d42abf6b67846b49b1856f4b133";
  finalImageTag = "0.202.7-bullseye";
})
