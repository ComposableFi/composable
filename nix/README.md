# Install Nix + Flakes

1. https://nixos.org/download.html
2. https://nixos.wiki/wiki/Flakes

# Run locally

1. `nix develop`
2. `launch-devnet`
3. Reach alice at `https://polkadot.js.org/apps/?rpc=ws://localhost:9944#/explorer`

# Deploy to GCE

1. Download your GCE service account key and save it as `ops.json`
2. `nix develop .#deploy`
3. `nixops create -d devnet-gce`
4. `nixops deploy -d devnet-gce`
