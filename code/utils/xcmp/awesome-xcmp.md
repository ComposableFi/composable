# Overview

Resources which allows to grasp  XCM. General understanding how bridges work will help too.

## Basics

XCM - Cross Chain Message.

XCMP - XCM passing.

XCMP can be upward (parachain to relay), downward(relay to parachain) and sibling(lateral, parachain to parachain).

[Moonbeam: Cross-Consensus Messaging (XCM)](https://docs.moonbeam.network/builders/xcm/overview/)

## Conceptual

[Polkadot's Cross-chain Message Passing Protocol: Shared Security and Polkadot's Design](https://www.youtube.com/watch?v=XU6dAAQD9UE)

[How XCM will actually be used with XCMP](https://forum.polkadot.network/t/how-xcm-will-actually-be-used-with-xcmp/190)

## How to setup XCMP

- [Polkadot XCM Cross-Chain Asset Transfer Demo](https://medium.com/oak-blockchain/polkadot-xcm-cross-chain-asset-transfer-demo-53aa9a2e97a7)
- <https://medium.com/oak-blockchain/tutorial-polkadot-cross-chain-message-passing-xcmp-demo-with-ping-pallet-f53397158ab4>

## Format of messages

- <https://medium.com/polkadot-network/xcm-part-ii-versioning-and-compatibility-b313fc257b83>
- <https://medium.com/polkadot-network/xcm-part-iii-execution-and-error-management-ceb8155dd166>
- <https://github.com/paritytech/xcm-format/blob/master/README.md>
- <https://research.web3.foundation/en/latest/polkadot/XCMP/index.html>
- <https://medium.com/polkadot-network/xcm-the-cross-consensus-message-format-3b77b1373392>

## XCM(P) design

- <https://www.youtube.com/watch?v=cS8GvPGMLS0>
- <https://www.youtube.com/watch?v=5cgq5jOZx9g>
- <https://substrate.stackexchange.com/questions/37/how-can-i-transfer-assets-using-xcm>
- <https://www.youtube.com/watch?v=wrA9vlPjVPE>
- <https://research.web3.foundation/en/latest/polkadot/XCMP/Opening_closing%20XCMP%20Channel.html>
- <https://www.youtube.com/watch?v=P_yLrFfmLrU>
- <https://blog.quarkslab.com/resources/2022-02-27-xcmv2-audit/21-12-908-REP.pdf>
- <https://github.com/paritytech/polkadot/blob/master/roadmap/implementers-guide/src/messaging.md>

## Generic context

- <https://wiki.polkadot.network/docs/learn-bridges>
- <https://wiki.polkadot.network/docs/learn-parachains>
- <https://polkadot.network/Polkadot-lightpaper.pdf>
- <https://wiki.polkadot.network/docs/learn-crosschain>
- <https://medium.com/web3foundation/polkadots-messaging-scheme-b1ec560908b7>

## Assets

<https://polkadot.network/blog/statemint-becomes-first-common-good-parachain-on-polkadot/>

### Other parachains usage

- <https://www.youtube.com/watch?v=5mspUoK1aIE>
- <https://docs.moonbeam.network/builders/xcm/xc20/overview/>

### Solution and integrations

- ORML + Cumulus, which does not support out of box access to all XMP and as of now opinionated implementations instructions.
- <https://www.youtube.com/watch?v=92w8rVXB5q8> extends and enhances to support more `Transact` patterns
- Checkout Composable [XCVM](../../xcvm/SPEC.md) which

### General cross~~bank~~chain patterns, bridges and helpers

- <https://gendal.me/2013/11/24/a-simple-explanation-of-how-money-moves-around-the-banking-system/>
- <https://medium.com/composable-finance/trustless-bridging-438a6e5c917a>
- <https://research.csiro.au/blockchainpatterns/general-patterns/blockchain-payment-patterns/token-swap/>

### Operations

<https://docs.substrate.io/reference/how-to-guides/parachains/add-hrmp-channels/>


Sends KSM from Picasso to Kusama to specified account:

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode/0x29020101000100010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c8434101040000000002c2eb0b00000000