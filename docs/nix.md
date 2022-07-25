# Overview

`nix` ecosystem replaces fully or partially `cargo install`, `rustup`, `cargo make`, `docker build`, etc.

Shells are organized roughly according GitHub teams.

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


## You are Technical writer


```shell
nix shell .#technical-writer
```

## Your are XCVM developer

As of now you have to install many manually, but we are working on it.