# Composable CosmWasm CLI

Composable Cosmwasm CLI is a CLI tool to quickly get started with the XCVM ecosystem and
interact with a chain that runs `pallet-cosmwasm`. In this guide, we will show you how to run the CLI on Picasso rococo.

## Installing `ccw`

There are two methods to installing the ccw-vm:

1. At first, clone the [Composable moonorepo](https://github.com/ComposableFi/composable):

```
git clone https://github.com/ComposableFi/composable
```

Then run the following command to install the ccw binary:

```
cargo install --path ./composable/code/parachain/frame/cosmwasm/cli
```

2. An alternative method to run the ccw-vm is by completing the next step to install Nix first and run the following command:

```
nix run composable#ccw
```

## Installing `nix` 

We are using Nix to set up and start our local development environment. So check out our 
[Nix installation page](https://docs.composable.finance/nix/install) to install Nix.

Then start the development environment by running:

```
nix run "github:ComposableFi/composable#dali-devnet"
```

This will take time at first but since it is cached, it will be almost instant afterward. But note that your node will be rebuilt if the commit hash changes. If you would like to avoid this, you can always use a specific commit hash like this example:

```
nix run "github:ComposableFi/composable/d2845fc731bc3ee418a17cf528336d50f4b39924#dali-devnet"
```

## Create a CosmWasm project

You can create a base CosmWasm project that you can work on.

```
ccw new --name get-started --description "Get started with CosmWasm"
```

See [here](./cosmwasm/new-project.md) for more.

## Upload a CosmWasm contract

For interacting with `pallet-cosmwasm`, `substrate` subcommand is used. To be able
to call your contracts, you need to upload them to the chain first. There are several
sources to upload your contracts:

### 1. Upload a local contract binary

You need to specify the file path and the signer to be able to upload a contract
from the file path. Extrinsics must be called by a signed entity in `pallet-cosmwasm`.
For now, the examples will use development accounts for signing extrinsics, but
we will explain it further later.

```sh
ccw -n alice upload -f /path/to/file.wasm
```

### 2. Upload a contract from a running chain

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

### 3. Upload a contract from a server

One common thing is to go to a contract's release page and download the contract
binary from there. You don't have to do that with `ccw`.

```sh
# Fetch the official release of `cw20_base.wasm`
ccw substrate -n alice tx upload --url https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm
```

## Interact with a contract

For examples of interacting with the contract, go to the [walkthrough](./cosmwasm/walkthrough.md).
