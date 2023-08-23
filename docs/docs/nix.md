# Nix

Nix is a requirement to set up and start a local development environment with Composable's code. We recommend using the Zero-to-Nix installer. Refer to our docs for how to [install and configure Nix](./nix/install).

After configuration, familiarise yourself with the following commands:

`nix develop` in to enter dev shell (command line, variables, tools, compilers).

`nix run "composable#devnet-picasso"` to run local devnet for CosmWasm development.

`nix run "composable#fmt"` format all files.

`nix build "composable#unit-tests"` check unit tests.