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

You can get all tooling to build book locally:

```shell
nix shell .#technical-writer
```

You can user `developers` codespace `nix` based devcontainer remotely or locally (see relevant codespace docs on how to).

You can `watch` book with

```shell
nix build .#composable-book
```

to see live preview.

## Your are XCVM developer

As of now you have to install many manually, but we are working on it.