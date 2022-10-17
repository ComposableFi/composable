# Overview

To run, any chain needs:

- genesis state (to be shared with cluster of nodes which form consensus)
- runtime to execute (logic of)
- bootstrap networking information (which nodes to connect)
- and, either relay setup or what relay to use
- and other useful setup like logging, metrics
- private keys in keystore 
- logical network connectivity allowance (bridges)

There are guides and tools which help to make life easier

## How tos

- https://docs.substrate.io/tutorials/v3/private-network/
- https://docs.substrate.io/tutorials/v3/cumulus/start-relay/
- https://docs.substrate.io/tutorials/v3/permissioned-network/
- https://docs.substrate.io/tutorials/v3/cumulus/connect-parachain/
- https://docs.substrate.io/how-to-guides/v3/basics/custom-chain-spec/

## Common Parachains

- https://wiki.polkadot.network/docs/learn-common-goods
- https://github.com/paritytech/cumulus
- https://github.com/paritytech/cumulus/blob/master/polkadot-parachains/src/command.rs


## Tools

- https://github.com/paritytech/polkadot-launch/blob/master/src/spawn
- https://github.com/paritytech/zombienet