# Overview

Resources which allows to grasp  XCM.  For concrete message examples relevant for Composable look [here](./xcm-examples.md).

## Basics

XCM - Cross Chain Message.

XCMP - XCM passing.

XCMP can be upward (parachain to relay), downward(relay to parachain) and sibling(lateral, parachain to parachain).

[Moonbeam: Cross-Consensus Messaging (XCM)](https://docs.moonbeam.network/builders/xcm/overview/)

## Conceptual

[Polkadot's Cross-chain Message Passing Protocol: Shared Security and Polkadot's Design(video)](https://www.youtube.com/watch?v=XU6dAAQD9UE)

[How XCM will actually be used with XCMP](https://forum.polkadot.network/t/how-xcm-will-actually-be-used-with-xcmp/190)

## How to setup XCMP basics

- [Polkadot XCM Cross-Chain Asset Transfer Demo](https://medium.com/oak-blockchain/polkadot-xcm-cross-chain-asset-transfer-demo-53aa9a2e97a7)
- [Tutorial Polkadot Cross-Chain Message Passing (XCMP) demo with ping pallet](https://medium.com/oak-blockchain/tutorial-polkadot-cross-chain-message-passing-xcmp-demo-with-ping-pallet-f53397158ab4)
- [XCMP overview](https://research.web3.foundation/en/latest/polkadot/XCMP/index.html)

## XCM standard

- [XCM Part II: Versioning and Compatibility](https://medium.com/polkadot-network/xcm-part-ii-versioning-and-compatibility-b313fc257b83)
- [XCM Part III: Execution and Error Management](https://medium.com/polkadot-network/xcm-part-iii-execution-and-error-management-ceb8155dd166)
- [Polkadot Cross-Consensus Message (XCM) Format (specification)](https://github.com/paritytech/xcm-format/blob/master/README.md)
- [XCM: The Cross-Consensus Message Format](https://medium.com/polkadot-network/xcm-the-cross-consensus-message-format-3b77b1373392)
- [The XCM executor(reference source)](https://github.com/paritytech/polkadot/blob/master/xcm/xcm-executor/src/lib.rs)
- [Cross-Consensus Message Format (XCM)](https://wiki.polkadot.network/docs/learn-xcm)
- [pallet-xcm(official public extrinsics for users)](https://github.com/paritytech/polkadot/blob/master/xcm/pallet-xcm/src/lib.rs)
- [Gavin Wood: XCM V3: Building a Heterogeneous Sharded Ecosystem | Polkadot Decoded 2022(video)](https://www.youtube.com/watch?v=ccfhYX3AimU)

## XCMP design

- [Substrate Builders Program Office Hours: XCM AMA(video)](https://www.youtube.com/watch?v=cS8GvPGMLS0)
- [Shawn Tabrizi: XCM - The Backbone Of A Multichain Future | Polkadot Decoded 2022(video)](https://www.youtube.com/watch?v=2tmspefsygQ)
- <https://www.youtube.com/watch?v=5cgq5jOZx9g>
- <https://substrate.stackexchange.com/questions/37/how-can-i-transfer-assets-using-xcm>
- <https://www.youtube.com/watch?v=wrA9vlPjVPE>
- <https://research.web3.foundation/en/latest/polkadot/XCMP/Opening_closing%20XCMP%20Channel.html>
- <https://www.youtube.com/watch?v=P_yLrFfmLrU>
- <https://blog.quarkslab.com/resources/2022-02-27-xcmv2-audit/21-12-908-REP.pdf>
- [Polkadot Implementers Guide Messaging Overview](https://github.com/paritytech/polkadot/blob/master/roadmap/implementers-guide/src/messaging.md)
- [Polkadot’s Messaging Scheme(article)](https://medium.com/web3foundation/polkadots-messaging-scheme-b1ec560908b7)

## Cross chain assets

<https://polkadot.network/blog/statemint-becomes-first-common-good-parachain-on-polkadot/>

## Other parachains usage

- <https://www.youtube.com/watch?v=5mspUoK1aIE>
- <https://docs.moonbeam.network/builders/xcm/xc20/overview/>
- [Statemine XCMP configuration](https://github.com/paritytech/cumulus/blob/master/parachains/runtimes/assets/statemine/src/xcm_config.rs)

## Solution and integrations

- [Subscan XCM (explorer/indexer)](https://picasso.subscan.io/xcm_dashboard)
- ORML + Cumulus, which does not support out of box access to all XMP and as of now opinionated implementations instructions.
- <https://www.youtube.com/watch?v=92w8rVXB5q8> extends and enhances to support more `Transact` patterns
- Checkout Composable [XCVM](../../xcvm/SPEC.md) which

- [Substrate Utilities(decoder/encoder)](https://www.shawntabrizi.com/substrate-js-utilities/)
- [Account Format Transform](https://polkadot.subscan.io/tools/ss58_transform)

## Security

[Kusama’s Governance Thwarts Would-be Attacker!](https://medium.com/kusama-network/kusamas-governance-thwarts-would-be-attacker-9023180f6fb)