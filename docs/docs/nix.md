# Nix

Nix is a requirement to set up and start a local development environment with Composable's code. We recommend using the Zero-to-Nix installer. Refer to our docs for how to [install and configure Nix](nix/install.md).

After configuration, familiarize yourself with the following commands:

`nix develop --impure` in to enter dev shell (command line, variables, tools, compilers).

`nix run "composable#devnet-picasso"` to run local devnet for Polkadot CosmWasm development.

`nix run "composable#fmt"` format all files.

`nix build "composable#unit-tests"` check unit tests.


### Per commit examples


```shell
# run Composable node
nix run "github:ComposableFi/composable/<COMMIT>" --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed
````

```shell
# run local Picasso DevNet (for CosmWasm development)
nix run "github:ComposableFi/composable/<COMMIT>#devnet-picasso" --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed 
```

```shell
# CosmWasm on Substrate CLI tool
nix run "github:ComposableFi/composable/<COMMIT>#ccw" --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed 
```

```shell
# run cross chain devnet with Dotsama and Cosmos nodes 
nix run "github:ComposableFi/composable/<COMMIT>#devnet-xc-fresh" --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed 
# or same with docker
nix build "github:ComposableFi/composable/<COMMIT>#devnet-xc-image" --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed \
&& docker load --input result && docker run -it --entrypoint bash devnet-xc:latest -c /bin/devnet-xc-fresh 
```
