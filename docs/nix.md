# Overview

`nix` ecosystem replaces fully or partially `cargo install`, `rustup`, `cargo make`, `docker build`, `sh`, `rust-cache`, `sccache`, specific `github actions`, parts of `github workflow runs`  etc.

If to target `nixos` then `ansible`, `ssh`, `terraform`.

Shells are organized roughly according GitHub teams.

## Setup

Install Nix + Flakes
1. https://nixos.org/download.html
2. https://nixos.wiki/wiki/Flakes


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