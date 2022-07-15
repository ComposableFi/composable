# Overview

## Setup

```shell
nix-env --install --attr nixpkgs.cachix
cachix use composable-community
```

## Run

```shell
nix build .#devnet-container
docker load --input ./result
# run 
```

or

```shell
nix run .#devnet
```
