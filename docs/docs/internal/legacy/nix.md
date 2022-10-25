# Old Nix Docs

Overview of nix usage and design.

## Design

### I want to use specific tool which helps setup environment, build or CI/CD, can I?

Generally you should check what nix can do for you. There is some list of tools which you may not ned with nix.

`nix` ecosystem replaces fully or partially `cargo install`, `rustup`, `cargo make`, `make`, `docker build`, `sh`, `rust-cache`, `sccache`, `github actions`, `github workflow runs`, `github artifacts`, `crates.io`, `npmjs.com`, `npx`, 
`nvm`, `brew`, `apt`, `devcontainers`, etc.

If to target `nixos` in the cloud, then `ansible`, partially `ssh`, partially `terraform`.

### How can get up to speed during team onboarding? How replicate CI environment locally?

`devShells` are roughly organized according `GitHub teams`.
Tht helps to ensure ensure locally tested nix is operating well in the CI given appropriate credentials.

### Where should I put my files?

Nix works well if your workspace(solution) folder has its own ignore files (example, .gitignore).


In nix ignores are integrated with `src` filters.

This way you speed up nix builds and get clean separation in monorepo.

### Where should I put nix files?

Root .nix folder can contain 
configurations external to this repository checkout, 
either 3rd party or not in this repo composable or not this revision of repo,
3rd parties either do not have nix configurations 
or we need heavily patch them.

Other scrips are located in folders on which they act upon, until scope is multi-folder or not scoped.

This allows for monorepo with codeowners and proper nix caching.

### Should I use `develop` or `run` or `build` or `home-manager` or `devcontainer`?

It should be possible to run things via `nix build/run` and also exec into devcontainer/shell/home and run native tooling build and run commands.

So for each `nix build/run` attribute there should be equivalent "`shell`" to exec into and run native tooling commands.

Examples, you can build `dali node` via `nix build`, so you can `nix shell .#developers` and run `cargo build`.

It means that `home-manager`, `devcontainer`, `apps`, `shells` should share and combine dependencies.

Any `app` or `packages` must be tested by CI by building it or starting it.

### Naming

Variables which are input from external non `nix` files (examples, json/yaml/toml) to be suffixed with `-input`. Inputs prevent early validation of packages without instantiation.  

### Source or binary for Rust deps?

Binary may not output all targets, not all commits, not forks and hard to diff changes from release to release and audit security.

Rust can do `cargo install` which is the same as `crane` does - builds from sources.

So for rust dependencies, we use source by default.

Because we use cachix, build as fast as binary.

Also Rust(and Go) are projects with a tendency toward deterministic builds.


### How to `unit test` nix?

Also nix lazy, derivation is side effect free, 
so you can evaluate tree and make nix `statically` typed.

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

## Shells

Organized roughly according GitHub teams.

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

### Your are XCVM developer

As of now you have to install many manually, but we are working on it.
