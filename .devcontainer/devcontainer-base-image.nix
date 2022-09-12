{ pkgs ? import <nixpkgs>, system ? builtins.defaultSystem }:
with pkgs;
let nix-to-container-image = import ./devcontainer-base-image-per-system.nix;
in dockerTools.pullImage ((nix-to-container-image system) // {
  imageName = "mcr.microsoft.com/vscode/devcontainers/base";
  os = "linux";
  finalImageTag = "0.202.7-bullseye";
})
