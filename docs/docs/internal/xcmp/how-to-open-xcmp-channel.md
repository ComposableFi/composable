---
title: How to open XCMP channel
---

# Overview 

This document describes all technical steps required to open a channel to any arbitrary chain.

Please adapt this to the variation of governance, business value and discussion forums according to the target chains.

This guide contains examples and links with guiding explanations on how to modify this for the next channel.

## Assumptions

This guide assumes the following conditions:
- You can get a minimum of 30 KSM and up to 70 KSM equivalent to handle end to end process

- You are aware of how to use Polkadotjs and open relevant chains

- You have a basic understanding of Dotsama encodings and the basics of XCM

- You are aware of how Root transactions can be executed on the target chain

- Transaction execution starts from Root unless stated otherwise

## Identity and communication mediums


- Create on chain and off chain identity on [polkassembly] 

- Login to https://matrix.to/#/#kusama:matrix.parity.io to observe discussions

- Each chain may have its own chat to find


## Minimum amount of native tokens for relay tip


Check that the chains involved have enough relay native tokens.

As of now this is in `Storage -> configuration -> activeConfig`  related to channels.

The minimal amount of relay native token required is `hrmpSenderDeposit` + `hrmpRecipientDeposit`. It's recommended to have 2x the minimal amount.

Chain accounts can be obtained by mapping a parachain id to an account via [substrate-js-utilities] 


## Send Request to open channel to counterparty parachain

Example, [0x290001010002100004000000000b0060defb740513000000000b0060defb74050006000700f2052a01383c00e8030000e8030000009001000d0100040001009d20](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode/0x290001010002100004000000000b0060defb740513000000000b0060defb74050006000700f2052a01383c00e8030000e8030000009001000d0100040001009d20).

It will request to open `hrmp` to other parachain

It will execute a request to open a channel (half of future duplex).

**Details**

A message will be sent to the relay and pay all fees in the relay native tokens.

Ensure the transaction is executed from `Native` (Parachain itself) `originType`
 
Ensure the function `DepositAsset` will deposit to the relevant Parachain identifier (spending chain)

Ensure `requireWeightAtMost` is safely larger (e.g. 2-3x) than the weight of the transact.

`Developer -> Runtime Calls -> TransactionPaymentCallApi -> queryCallInfo -> weight`
but strictly (and if possible safely) less than `requireWeightAtMost`.

`requireWeightAtMost` should be equivalent or less than in `BuyExecution` (you can compare it to the `queryCallInfo.partialFee` of the call and make `BuyExecution` 10x+ larger than this). In case of failure to do so, XCM will fail with `TooExpensive`

Do not set `requireWeightAtMost` too big as you will get `dmpQueue.OverweightEnqueued`. `requireWeightAtMost` has some similar semantics to `requireWeightMinimum`. It is not clear if the message will be retried automatically. For a sanity check navigate to `configuration -> activeConfig -> umpMaxIndividualWeight` and see if it is way higher than the weight of our transact.

For additional safety consider scheduling the transact messages instead of executing immediately.

[0x3c00e8030000e803000000900100](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode/0x3c00e8030000e803000000900100).

Check that: 

- The `Transact` message contains a proper existing parachain recipient.

- `proposedMaxCapacity` is less or equal to `Storage -> configuration -> activeConfig` `hrmpChannelMaxCapacity`

- `proposedMaxMessageSize` is less or equal to `hrmpChannelMaxMessageSize` here.

- `hrmpMaxParachain*Channels` is not exceeded by any participant chains.
## Counterparty chain to accept our request and request back

Encode the preimage above via `Democracy -> Submit preimage`.

Then `Submit proposal`.

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-3.aca-api.network%2Fws#/extrinsics/decode/0x33000101000210000400000000070010a5d4e81300000000070010a5d4e800060003009435775c1802083c01270800003c0027080000e8030000009001000d010004000100411f

**Details** 

Find the latest `proposal` or successful `referenda` to open channels and compare differences with our message. Things change so the further examples are likely to be out of date.

Please do all relevant checks from the previous step.

## Accept on our chain request

This should be executed

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama.api.onfinality.io%2Fpublic-ws#/extrinsics/decode/0x3c01d0070000

executed on behalf of 

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode/0x2900010100020c000400000000070010a5d4e81300000000070010a5d4e80006010700e8764817183c01d0070000


**Details**

All relevant check steps similar to before do apply.

Additionally check that there is `hrmp` open channel request to us.

## Tokens and transfers

Follow relevant guides.

You may find some [examples](./xcm-examples.md)

[polkassembly]:https://parachains.polkassembly.io/

[karura-gov]: https://karura.subsquare.io/democracy/

[substrate-js-utilities]: https://www.shawntabrizi.com/substrate-js-utilities

[moonbeam]: (https://docs.moonbeam.network/builders/xcm/xc-integration/)


## References

https://docs.acalaswap.app/developer-guides/create-a-new-token

https://wiki.acala.network/build/development-guide/composable-chains/open-hrmp-channel

https://acala.notion.site/Acala-Karura-Token-Listing-Playbook-c6b97e022ac6402cb15ce3cb419c48e5

https://wiki.acala.network/integrate/integration-1/token-transfer

https://acala.discourse.group/t/open-hrmp-channel-between-composable-picasso-and-karura/918

https://karura.subsquare.io/democracy/proposal/

https://kusama.polkassembly.io/referendum/163

https://kusama.polkassembly.io/referendum/164

https://acala.discourse.group/t/open-hrmp-channel-between-karura-and-statemine/451

https://acala.discourse.group/t/open-hrmp-cross-chain-communication-between-bifrost-and-karura-parachain/316/7