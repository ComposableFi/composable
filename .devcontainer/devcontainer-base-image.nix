{ pkgs ? import <nixpkgs>, system ? builtins.defaultSystem}:
with pkgs;
let 
   nix-to-container-image = import ./devcontainer-base-image-per-system.nix;
in  
 dockerTools.pullImage ((nix-to-container-image system) // {
  imageName = "ghcr.io/jmgilman/dev-container";
  os = "linux";
  finalImageTag = "sha-6124ab0";             
})