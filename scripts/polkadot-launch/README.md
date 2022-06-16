# Overview
This is a guide on how to run things locally or in isolated environment with your own stuff

## Prepare 

1. `1-build-composable-collator.sh`

2. `2-download-polkadot-relay.sh`

3. `3-download-basilisk-collator.sh`

4. run `yarn` to build the runner
## Run Composable's parachain only

```bash
yarn composable
```

URLs:
* [the 1st Relay Chain node](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer)
* [the 1st Composable collator](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer)

## Run Composable's and Basilisk's parachains

Need to do to run 5 Relay Chain nodes, 2 Composable collators and 2 Basilisk collators:

```bash
yarn composable_and_basilisk
```

## Run  Kusama relay + Dali parachain + Hydra paracahin in Docker via [polkadot-launch](https://github.com/paritytech/polkadot-launch)

Build via `sandbox docker` job in Actions into latest and git hash.

```
cargo make start-devnet-docker
```

URLs:
* [the 1st Relay Chain node](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer)
* [the 1st Composable collator](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer)
* [the 1st Basilisk collator](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9998#/explorer)
