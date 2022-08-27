{ pkgs ? import <nixpkgs>, system ? builtins.defaultSystem }:
with pkgs;
let
  nix-to-container-image = import ./devcontainer-base-image-per-system.nix;
  # image which will be base for remote development
  # we do not start from nixos:
  # - no all people like devcontainer to be nix (gh know better)
  # - devcontainer has setup in shell for code, users, groups and remote stuff
  # - it has nice cli/shell setup, unlike bare nixos docker        
  # we want devcontainer to be built of nix:
  # - so it has same version or rust as our env and ci
  # - it has same all tooling we have
  # - and we do not need to maintain separate script for that
in dockerTools.pullImage ((nix-to-container-image system) // {
  imageName = "mcr.microsoft.com/vscode/devcontainers/base";
  os = "linux";
  imageDigest =
    "sha256:269cbbb2056243e2a88e21501d9a8166d1825d42abf6b67846b49b1856f4b133";
  finalImageTag = "0.202.7-bullseye";
})
