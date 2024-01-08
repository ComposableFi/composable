# Composable CosmWasm CLI

Composable Cosmwasm CLI is a CLI tool to quickly get started with the ecosystem and interact with a chain that runs `pallet-cosmwasm`. In this guide, we will show you how to run the CLI on a local Picasso network and Picasso Rococo. 

:::info 
Picasso Rococo is a testnet (test network) for [Picasso](../networks/picasso-parachain-overview.md). It allows developers to experiment, test runtime module deployment, and refine their applications to ensure the stability and compatibility of new features before deploying on Picasso mainnet by interacting with the [Rococo Relay Chain](https://polkadot.network/blog/rococo-revamp-becoming-a-community-parachain-testbed/).
:::

## Setting up the development environemnt

The process of setting up a development environment for deploying CosmWasm contracts, both a local Picasso netork and on Picasso Rococo, follows the same procedure. There is a distinction in the RPC endpoint mentioned in the CLI commands to upload, instantiate and execute contracts. To interact with a local Picasso network, you will utilize `ws://127.0.0.1:9988` whereas to deploy on Picasso Rococo, you will employ `wss://picasso-rococo-rpc-lb.composablenodes.tech:443`. Additionally, please note that the "-n alice" sudo key will be substituted with your seed phrase when entering the commands.

Nix is a requirement to set up and start a local development environment with Composable's code. We recommend using the [Zero-to-Nix installer](https://zero-to-nix.com/start/install) to install Nix. Refer to our [docs](../nix.md) for more information.

### Installing `ccw`

To install the `ccw-vm`, run the following command using Nix:

```
nix run github:ComposableFi/composable/release-v9.10038.2#ccw --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed
```

:::note

Make sure to include the most recent release tag, which can be located on the Composable [releases page](https://github.com/ComposableFi/composable/releases).
:::
### Setting up the DevNet

To run a local network with Alice sudo key and start the development environment, run the following command:

```
nix run github:ComposableFi/composable/release-v9.10038.2#devnet-picasso --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed
```

This will take time at first but since it is cached, it will be almost instant afterward. But note that your node will be rebuilt if the commit hash changes. If you would like to avoid this, you can always use a specific commit hash like this example:

```
nix run "github:ComposableFi/composable/d2845fc731bc3ee418a17cf528336d50f4b39924#devnet-picasso"
```

### Setting up the environment to deploy on a local network of Picasso

Once your node is set up on local from the previous step, open the Polkadot-Js explorer to view activity changes on your local network by heading to the development section with the custom endpoint linked [here](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer). If Polkadot-JS fails to load your local network after running the node, it is possible that there was an error during the build process, resulting in the failure to load it correctly. 

### Setting up the environment to deploy on Picasso Rococo

Once you have completed the setup of your development environment, you can proceed to the [PolkadotJS explorer](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rococo-rpc-lb.composablenodes.tech#/explorer) dedicated to Picasso Rococo. This explorer allows you to monitor real-time events and also interact with `ccw` without the need to interact with the CLI. 

To deploy contracts on Picasso Rococo, you will need PICA tokens for testing. To assist with this, we have established a [faucet](https://matrix.to/#/#picasso-rococo-faucet:matrix.org) on the Matrix platform. It enables developers to receive PICA tokens specifically for the Picasso Rococo network. To retrieve your address, you can visit the [Accounts page](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rococo-rpc-lb.composablenodes.tech#/accounts) in PolkadotJS. For detailed instructions on creating a PolkadotJS wallet, please refer to [this guide](../user-guides/polkadotjs-extension-create-account.md) we have published. Additionally, make sure you have updated to the latest metadata and have enabled the 'Allow use on any chain' option within the PolkadotJS plugin.

A useful resource for interacting with PolkadotJS is their [developer documentation](https://polkadot.js.org/docs/). 

## Interacting with the CLI 

### Upload a CosmWasm contract

For interacting with `cosmwasm`, the `substrate` subcommand is used. To be able
to call your contracts, you need to upload them to the chain first. The difference between running on a local devnet and on Picasso Rococo is to replace '-n Alice' with your seed phrase in the commands and the RPC endpoints, an example is provided below during the upload of a local contract binary.

There are several sources to upload your contracts:

#### 1. Upload a local contract binary

You need to specify the file path and the signer to be able to upload a contract
from the file path. Extrinsics must be called by a signed entity in `pallet-cosmwasm`.

```
cd path/to/file 
```

```sh
# On Picasso local
ccw substrate -c ws://127.0.0.1:9988 -n alice tx upload --file-path .path/to/file
```

```sh
# On Picasso Rococo 
ccw substrate -c wss://picasso-rococo-rpc-lb.composablenodes.tech:443 --seed "<your SEED phrase>" tx upload --file-path .path/to/file
```

#### 2. Upload a contract from a running chain

If a Cosmos chain provides an RPC endpoint, you can use it to load the contracts
to `ccw`. All you need to know is the RPC endpoint to fetch the
contract from, and either the contract address that uses the contract code
or code ID that identifies the contract code.

Fetch using the contract address:
```sh
ccw substrate -n alice tx upload --cosmos-rpc https://juno-api.polkachu.com --contract juno19rqljkh95gh40s7qdx40ksx3zq5tm4qsmsrdz9smw668x9zdr3lqtg33mf
```

Fetch using the code ID:
```sh
ccw substrate -n alice tx upload --cosmos-rpc https://juno-api.polkachu.com --code-id 1
```

## Interact with contracts

For examples of interacting with the contract, go to the [walkthrough](./cosmwasm/walkthrough.md).
