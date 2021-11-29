# Overview

Runs transfers from some Composable based parachain to Composable parachain. And other parachains integrations.

## Flow

### Setup

- Relay must be be added with parachains which will communicate
- Each parachain must add other parachain into `ParachainSystem` to allow requests from other chain
- Each parachain setups execution prices and filters to secure XCMP messaging
- Each parachain must add mapping for currency it wants to send to other parachain

### Transfer currency

Transfer currency is based on sending some named messages interpreted on each chain, but always ends with `dispatch` calls on target chain.

- calls `XTokens` pallet to map local tokens to remote
- it will call `xcm_executor::traits::TransactAsset`  to form proper XCM messages with unique request id
- then messages will be put into `XcmQueue` on-chain
- Networking layer will ensure that messages appear on another chain
- Messages will be `dispatched` to call relevant pallet for accepting foreign assets.
- It is possible to send a message and ask send a response about success/fail operation, but that happens not in same block

## Readings


### How to setup XCMP

- [Polkadot XCM Cross-Chain Asset Transfer Demo](https://medium.com/oak-blockchain/polkadot-xcm-cross-chain-asset-transfer-demo-53aa9a2e97a7)
- https://medium.com/oak-blockchain/tutorial-polkadot-cross-chain-message-passing-xcmp-demo-with-ping-pallet-

### Format of messages

- https://medium.com/polkadot-network/xcm-part-ii-versioning-and-compatibility-b313fc257b83
- https://medium.com/polkadot-network/xcm-part-iii-execution-and-error-management-ceb8155dd166