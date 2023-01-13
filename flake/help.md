# Composable Nix Quickstart guide

## Run a local devnet

```sh
nix run ".#devnet-dali"
```

## Format the entire repository

```sh
nix run ".#fmt"
```

## Run most CI checks locally

```sh
nix run ".#check"
```

## Show all possible packages, apps, and devShells

```sh
nix flake show
```

## Run a package/app with full logs

```sh
nix run ".#devnet-dali" -L
```

[Read the full docs here](https://docs.composable.finance/nix/)
