
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

All steps are SCALE encoded and can be decoded and executed by named consensuses.

Proposal follows same steps as other chains did.

## Preparation

One should have `Identity` on Kusama to create `Proposal` on  https://parachains.polkassembly.io/ . 

Suggested amount is 50 KSM total for all operations on `Balance`, for creating identity, backing proposal and sending XCM messages.

Picasso chain also should have Balance, better 22 KSM. Because its sovereign account may also pay some fee.

## Picasso Governance to create request

Picasso to ask Statemine to open channel on Kusama encoded as `hrmp`  `0x3c00e8030000e803000000900100` to open channel. Can be decoded by Kusama.

That should be send from Parachain account from Picasso via next `relayerXcm.send` :

Encoded as `0x2900010100020c0004000000000700e876481713000100000700e876481700060102286bee383c00e8030000e803000000900100`.

Can be decoded via https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode

## Accept proposal and propose back

Create `Proposal` for referenda to ensure Kusama executed channel opening on its owned Statemine network:
```
This proposal aims to open HRMP channel between Statemine & Picasso. For more context please read here.

Let me explain the technical details of this call. It is a batch transaction with two calls:

1. A force transfer from Kusama treasury (`F3opxRbN5ZbjJNU511Kj2TLuzFcDq9BGduA9TgiECafpg29`) to Statemine (`F7fq1jSNVTPfJmaHaXCMtatT1EZefCUsa7rRiQVNR5efcah`). The amount is 11 KSM. 10 KSM will be used for deposit to accept (5 KSM) and open (5 KSM) HRMP channel. 1 KSM will be used by Statemine parachain to pay for transaction execution fee on Kusama. Note that 1 KSM is more than enough and unused funds will be trapped in XCM asset trap. But that's totally fine as it can be claimed & used for transaction fee in later XCM executions.

2. Send XCM message to Statemine to execute a transaction with superuser (root) permission.

The XCM message to Statemine is `0x1f00010100020c000400000000070010a5d4e81300000000070010a5d4e800060002286bee5c1802083c01270800003c0027080000e803000000900100`, which can be decoded on Statemine, and it is `polkadotXcm.send`. It sends a XCM message back to Kusama, to with 1 KSM for transaction fee and perform a transact of call `0x1802083c01270800003c0027080000e803000000900100`.

The call is is a `batchAll` that accepts open channel request from Picasso, and make an open channel request to Picasso.
```

### Proposal body

`batchAll` call:
<!-- cspell:disable -->
```json
calls: [
    {
        "callIndex": "0x0402",
        "args": {
            "source": {
                "id": "F3opxRbN5ZbjJNU511Kj2TLuzFcDq9BGduA9TgiECafpg29"
            },
            "dest": {
                "id": "F7fq1jSNVTPfJmaHaXCMtatT1EZefCUsa7rRiQVNR5efcah"
            },
            "value": 11000000000000
        }
    },
    {
        "callIndex": "0x6300",
        "args": {
            "dest": {
                "v1": {
                    "parents": 0,
                    "interior": {
                        "x1": {
                            "parachain": 1000
                        }
                    }
                }
            },
            "message": {
                "v2": [
                    {
                        "transact": {
                            "originType": "Superuser",
                            "requireWeightAtMost": 1000000000,
                            "call": {
                                "encoded": "0x1f00010100020c000400000000070010a5d4e81300000000070010a5d4e800060002286bee5c1802083c01e70700003c00e7070000e803000000900100"
                            }
                        }
                    }
                ]
            }
        }
    }
]
```
<!-- cspell:enable -->

#### Decoded proposal

Above encoded call is next XCM message:
<!-- cspell:disable -->
```json
[
  {
    name: "calls",
    type: "Vec<Call>",
    value: [
      {
        call_index: "0402",
        call_module: "Balances",
        call_name: "force_transfer",
        params: [
          {
            name: "source",
            type: "sp_runtime:multiaddress:MultiAddress",
            value: {
              Id: "0x6d6f646c70792f74727372790000000000000000000000000000000000000000"
            }
          },
          {
            name: "dest",
            type: "sp_runtime:multiaddress:MultiAddress",
            value: {
              Id: "0x70617261e8030000000000000000000000000000000000000000000000000000"
            }
          },
          {
            name: "value",
            type: "compact<U128>",
            value: "11000000000000"
          }
        ]
      },
      {
        call_index: "6300",
        call_module: "XcmPallet",
        call_name: "send",
        params: [
          {
            name: "dest",
            type: "xcm:VersionedMultiLocation",
            value: {
              V1: {
                interior: {
                  X1: {
                    Parachain: 1000
                  }
                },
                parents: 0
              }
            }
          },
          {
            name: "message",
            type: "xcm:VersionedXcm",
            value: {
              V2: [
                {
                  Transact: {
                    call: "1f00010100020c000400000000070010a5d4e81300000000070010a5d4e800060002286bee5c1802083c01270800003c0027080000e803000000900100",
                    origin_type: "Superuser",
                    require_weight_at_most: 1000000000
                  }
                }
              ]
            }
          }
        ]
      }
    ]
  }
]
```
<!-- cspell:enable -->

Where transact [decodes](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fstatemine-rpc.dwellir.com#/extrinsics/decode) by Statemine into:

```rust
polkadotXcm
send(dest, message)
send
dest: XcmVersionedMultiLocation
{
  V1: {
    parents: 1
    interior: Here
  }
}
message: XcmVersionedXcm
{
  V2: [
    {
      WithdrawAsset: [
        {
          id: {
            Concrete: {
              parents: 0
              interior: Here
            }
          }
          fun: {
            Fungible: 1,000,000,000,000
          }
        }
      ]
    }
    {
      BuyExecution: {
        fees: {
          id: {
            Concrete: {
              parents: 0
              interior: Here
            }
          }
          fun: {
            Fungible: 1,000,000,000,000
          }
        }
        weightLimit: Unlimited
      }
    }
    {
      Transact: {
        originType: Native
        requireWeightAtMost: 1,000,000,000
        call: {
          encoded: 0x1802083c01270800003c0027080000e803000000900100
        }
      }
    }
  ]
}
```

Where [Kusama.utility.batchAll](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics) is encoded as `0x1802083c01270800003c0027080000e803000000900100`

## Accept request from Statemine

Accept channel from state mine as  `0x3c01e8030000` (can be decoded by Kusama)


Sent as XCM  message from Picasso with that acceptance via `0x2900010100020c0004000000000700e876481713000100000700e876481700060102286bee183c01e8030000`

Decoded by https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode

## Make price for USDT

Register USDT in registry with asset id 11 and decimals of 4 via `assetsRegistry.registerAsset` with next preimage (tune ratio as needed):
`0x3b00010300a10f043206400b0000000000000000000000000000000a000000000000000000000000000000010000c16ff286230000000000000000000104000000`


## References

- https://kusama.polkassembly.io/referendum/163

- https://kusama.polkassembly.io/referendum/164

- https://acala.discourse.group/t/open-hrmp-channel-between-karura-and-statemine/451

- https://acala.discourse.group/t/open-hrmp-cross-chain-communication-between-bifrost-and-karura-parachain/316/7