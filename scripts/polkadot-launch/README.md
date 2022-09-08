# Overview

This is a guide on how to run things locally or in an isolated environment with your own machine.

After starting local nodes wait up to 2 minutes until all will be wired.

## Run Composable's parachain only

```bash
nix run ../../#devnet-up
```

URLs:
* [the 1st Relay Chain node](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer)
* [the 1st Composable collator](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer)

## Run Composable's and Acala's parachains

```bash
nix run ../../#devnet-kusama-picasso-karura
```

* [the 1st Acala collator](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9998#/explorer)