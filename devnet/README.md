# Overview

Devnet configs based on [nix](../docs/nix.md)

## Run locally

1. `./update.sh REVISION` where `REVISION` is the latest deployed git commit hash.
2. Go back to the root directory and run `nix develop .#sre` then run the devnet using `run-dali-dev`.
3. Reach alice at `https://polkadot.js.org/apps/?rpc=ws://localhost:9944#/explorer`

## GCE

Download your GCE service account key and save it as `ops.json`.

### Deploy

1. `nix develop .#sre`
2. `nixops create -d devnet-gce`
3. `nixops deploy -d devnet-gce`

### Connect to CI deployed machines

1. Download nixops CI state: `gsutil cp gs://composable-state/deployment.nixops .`
2. Run `NIXOPS_STATE=deployment.nixops nixops ssh composable-devnet-dali-dev`
