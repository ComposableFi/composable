
# Overview

This proposal suggest to open bidirectional HRMP channel between Picasso and Statemine. This will enable crosschain communication between Picasso and Statemine to enable various use cases including crosschain token transfer.

- [Overview](#overview)
  - [Steps](#steps)
  - [Preparation](#preparation)
  - [Picasso Governance to create request](#picasso-governance-to-create-request)
  - [Accept proposal and propose back](#accept-proposal-and-propose-back)
    - [Proposal body](#proposal-body)
      - [Decoded proposal](#decoded-proposal)
  - [Accept request from Statemine](#accept-request-from-statemine)
  - [Make price for USDT](#make-price-for-usdt)
  - [References](#references)

## Steps

0. Prepare
1. Open channel request from Picasso to Statemine
2. Statemine accept channel request and send request to Picasso
3. Accept channel form Statemine to Picasso
4. Make USDT priceable on Picasso

## Preparation

One should have `Identity` on Kusama to create `Proposal` on  https://parachains.polkassembly.io/ . 

Suggested amount is 50 KSM total for all operations on `Balance`, for creating identity, backing proposal and sending XCM messages.

Picasso chain also should have Balance, better 22 KSM. Because its sovereign account will pay some fee and lock amounts.

## Picasso Governance to create request

```shell
xcmp sudo execute --suri <private key> --call 0x290001010002100004000000000b0060defb740513000000000b0060defb74050006000700f2052a01383c00e8030000e8030000009001000d0100040001009d20 --network composable_picasso_on_parity_kusama --rpc 'wss://picasso-rpc.composable.finance:443'
```

Which will transact:

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode/0x3c00e8030000e803000000900100

## Statemine to accept proposal and propose back

Create offchain `Proposal` for referenda to ensure Kusama executed channel opening on its owned Statemine network:
```
This proposal aims to open HRMP channel between Statemine & Picasso. For more context please read here.

Let me explain the technical details of this call. It is a batch transaction with two calls:

1. A force transfer from Kusama treasury (`F3opxRbN5ZbjJNU511Kj2TLuzFcDq9BGduA9TgiECafpg29`) to Statemine (`F7fq1jSNVTPfJmaHaXCMtatT1EZefCUsa7rRiQVNR5efcah`). The amount is 11 KSM. 10 KSM will be used for deposit to accept (5 KSM) and open (5 KSM) HRMP channel. 1 KSM will be used by Statemine parachain to pay for transaction execution fee on Kusama. Note that 1 KSM is more than enough and unused funds will be trapped in XCM asset trap. But that's totally fine as it can be claimed & used for transaction fee in later XCM executions.

2. Send XCM message to Statemine to execute a transaction with superuser (root) permission.

The XCM message to Statemine is `0x1f00010100020c000400000000070010a5d4e81300000000070010a5d4e800060002286bee5c1802083c01270800003c0027080000e803000000900100`, which can be decoded on Statemine, and it is `polkadotXcm.send`. It sends a XCM message back to Kusama, to with 1 KSM for transaction fee and perform a transact of call `0x1802083c01270800003c0027080000e803000000900100`.

The call is is a `batchAll` that accepts open channel request from Picasso, and make an open channel request to Picasso.
```

See `References` links for portal to create proposal.

### Create on chain proposal


https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama.api.onfinality.io%2Fpublic-ws#/extrinsics/decode/0x1802080402006d6f646c70792f747273727900000000000000000000000000000000000000000070617261e80300000000000000000000000000000000000000000000000000000b00b01723010a630001000100a10f0204060202286beef41f00010100020c000400000000070010a5d4e81300000000070010a5d4e800060002286bee5c1802083c01270800003c0027080000e803000000900100

#### Decoded proposal

Above encoded call is has next XCM Transact message:

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fstatemine-rpc.dwellir.com#/extrinsics/decode/0x1f00010100020c000400000000070010a5d4e81300000000070010a5d4e800060002286bee5c1802083c01270800003c0027080000e803000000900100

Where [Kusama.utility.batchAll](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode/0x1802083c01270800003c0027080000e803000000900100) is encoded as ``

## Picasso to accept request from Statemine

Accept channel from Statemine:

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode/0x3c01e8030000

```shell
xcmp sudo execute --suri SECRET_KEY_OR_FILE --call 0x2900010100020c0004000000000700e876481713000100000700e876481700060102286bee183c01e8030000 --network composable_picasso_on_parity_kusama --rpc 'wss://picasso-rpc.composable.finance:443'
```

## Make price for USDT on Picasso

Register USDT:
```shell
xcmp sudo execute --suri SECRET_KEY_OR_FILE --call 0x3b00010300a10f043206400b0000000000000000000000000000000a000000000000000000000000000000010000c16ff286230000000000000000000104000000 --network composable_picasso_on_parity_kusama --rpc 'wss://picasso-rpc.composable.finance:443'
```

## References

- https://kusama.polkassembly.io/referendum/163

- https://kusama.polkassembly.io/referendum/164

- https://acala.discourse.group/t/open-hrmp-channel-between-karura-and-statemine/451

- https://acala.discourse.group/t/open-hrmp-cross-chain-communication-between-bifrost-and-karura-parachain/316/7