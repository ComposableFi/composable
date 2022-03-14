# /!\ Install Nix + Flakes

1. https://nixos.org/download.html
2. https://nixos.wiki/wiki/Flakes

# Run locally

1. `./latest.sh SPEC` where `SPEC` is one of the runtime:
   - `dali-dev`
   - `picasso-dev`
   - `composable-dev`
3. Reach alice at `https://polkadot.js.org/apps/?rpc=ws://localhost:9944#/explorer`

NOTE: The script will automatically run the latest devnet deployed from the latest origin/main commit.

# GCE

/!\ Download your GCE service account key and save it as `ops.json`.

## Deploy

1. `nix develop .#deploy`
2. `nixops create -d devnet-gce`
3. `nixops deploy -d devnet-gce`

## Connect to CI deployed machines

1. Download nixops CI state: `gsutil cp gs://composable-state/deployment.nixops .`
2. Run `NIXOPS_STATE=deployment.nixops nixops ssh composable-devnet-dali-dev`
