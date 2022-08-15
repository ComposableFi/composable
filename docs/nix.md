# Overview

Describes set of guiding principles authoring nix and how to start using nix.

## Design

### I want to use specific tool which helps setup environment, build or CI/CD, can I?

Generally you should check what nix can do for you. There is some list of tools which you may not ned with nix.

`nix` ecosystem replaces fully or partially `cargo install`, `rustup`, `cargo make`, `make`, `docker build`, `sh`, `rust-cache`, `sccache`, `github actions`, `github workflow runs`, `crates.io`, `npmjs.com`, `npx`, etc.

If to target `nixos` in the cloud, then `ansible`, partially `ssh`, partially `terraform`.

### How can get up to speed during team onboarding? How replicate CI environment locally?

`devShells` are roughly organized according `GitHub teams`.
Tht helps to ensure ensure locally tested nix is operating well in the CI given appropriate credentials.

### Where should I put my files?

Nix works well if your workspace(solution) folder has its own ignore files (example, .gitignore).
In nix ignores are integrated with `src` filters.
This way you speed up nix builds and get clean separation in monorepo.

## Install and run

### Setup

Install Nix + Flakes

1. <https://nixos.org/download.html>
2. <https://nixos.wiki/wiki/Flakes>

```shell
nix-env --install --attr nixpkgs.cachix
cachix use composable-community
```

### Run

```shell
nix build .#devnet-container
docker load --input ./result
# run 
```

or

```shell
nix run .#devnet
```

### You are Technical writer

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
