# Overview

This document describes basic usage of CVM on CosmWasm from command line and TypeScript Node environment.

It does not teaches you basics of Cosmos nor CosmWasm nor blockchain, but references some relevant information.

## Prerequisites

You aware in generate how to interact with CosmWasm contracts. If not, please follow official guides on test contracts.

You know how to call Cosmos RPC via CLI(see guides Wasmd/Osmosis/Notional) or TS. If not, please follow official guides. On mainnet you may consider Celatone.

You are aware of Bech32 encoding of accounts and IBC prefixed assets in Cosmos.

### On mainnet chain

| chain     | stage   | id                                                                  |
| --------- | ------- | ------------------------------------------------------------------- |
| osmosis-1 | mainnet | osmo126n3wcpf2l8hkv26lr4uc8vmx2daltra5ztxn9gpfu854dkfqrcqzdk8ql     |
| centauri  | mainnet | centauri1c676xpc64x9lxjfsvpn7ajw2agutthe75553ws45k3ld46vy8pts0w203g |

On devnets, there just CVM conracts, so can get their address via logs in `/tmp/composable-devnet/` or via RPC.

I maintained all latest idenfiers and mapping in [global configration file](./cvm.json).

### Shells

You have followed official guides of Osmosis and Centauri to setup their mainnet shells.

Optionally, you can use nix too [nix](../../docs/docs/nix.md) according docs. 
You can `nix develop "composable#centauri-devnet" --impure` to get devnet shell, same for `osmosis` and `mainnet` variants.


### Assets

You must have PICA and (optional) DOT. Please follow multi hop guide to transfer amount amounts.

On DevNet, `bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort`(which is validator too) has some PICA on start.


## End to end

These steps given that the user has send tx from Centauri to osmosis with PICA and swaps its to OSMO on Osmosis.

Identifiers for same flow with DOT are provided later, but left for manual repeating.

Also common queries to get state of CVM in general and specific user are described. 

Finally, program to handle stuck funds (in case of cross chain message failure) is given. In short, that is just transfer program.

### Recording 


You can find how devnet runs end to end here https://www.youtube.com/watch?v=_nMD407E3F4

You can find how mainnet rns end to end here https://www.youtube.com/watch?v=kxLkKzYW2xw

As of time of writing testnet is not ready for cross chain. So no recording.

#### Prominent identifiers

PICA on Centauri is 158456325028528675187087900673
OSMO on Centauri is 158456325028528675187087900674
DOT on Centauri is 158456325028528675187087900675

PICA on Osmosis is 237684487542793012780631851009
OSMO on Osmosis is 237684487542793012780631851010
DOT on Osmosis is 237684487542793012780631851011


PICA<->OSMO on Osmosis is 237684489387467420151587012609
PICA<->DOT on Osmosis is 237684489387467420151587012610


### Bridge Centauri PICA to Osmosis

Execute next message. This message transfer PICA from sender account to CVM and bridges it to Osmosis.

Please read and remove `//` commands before executing.

```json
            {
              "execute_program": {
                "execute_program": {
                  "salt": "737061776e5f776974685f6173736574", // each user has instances of interpreter contract per user per salt, so each new slat instances new contract, 
                                                              // while reusing salt reuses existing instances (and funds on these)
                  "program": {
                    "tag": "737061776e5f776974685f6173736574", // a number give by user which allows to differentiate on program from other (just of offchain indexing)  
                    "instructions": [
                      {
                        "spawn": {
                          "network_id": 3, // this is Osmosis
                          "salt": "737061776e5f776974685f6173736574",
                          "assets": [
                            [
                              "158456325028528675187087900673", // PICA on Centauri
                              {
                                "amount": {
                                  "intercept": "1234567890", // amount to move to Osmosis, but be same or larger than moved to interpreter
                                  "slope": "0"
                                },
                                "is_unit": false
                              }
                            ]
                          ],
                          "program": {
                            "tag": "737061776e5f776974685f6173736574",
                            "instructions": [
                              {
                                "exchange": {
                                  "exchange_id": "237684489387467420151587012609", // PICA<->OSMO pool id as configured on Osmosis
                                  "give": [
                                    [
                                      "237684487542793012780631851009", // PICA on Osmosis has other identifier
                                      {
                                        "amount": {
                                          "intercept": "123456789",
                                          "slope": "0"
                                        },
                                        "is_unit": false
                                      }
                                    ]
                                  ],
                                  "want": [
                                    [
                                      "237684487542793012780631851010", // OSMO on Osmosis
                                      {
                                        "amount": {
                                          "intercept": "1000", // min want amount, larger value is less slippage
                                          "slope": "0"
                                        },
                                        "is_unit": false
                                      }
                                    ]
                                  ]
                                }
                              },
                              {
                                "spawn": {
                                  "network_id": 2, // Centauri
                                  "salt": "737061776e5f776974685f6173736574",
                                  "assets": [
                                    [
                                      "237684487542793012780631851010",
                                      {
                                        "amount": {
                                          "intercept": "0",
                                          "slope": "1000000000000000000" // 100% of OSMO after swap to be transferred to Centauri
                                        },
                                        "is_unit": false
                                      }
                                    ]
                                  ],
                                  "program": {
                                    "tag": "737061776e5f776974685f6173736574",
                                    "instructions": [
                                      {
                                        "transfer": {
                                          "to": {
                                            "account": "AB9vNpqXOevUvR5+JDnlljDbHhw=" // base64 encoded 32 byte account to deposit assets on Centauri
                                          },
                                          "assets": [
                                            [
                                              "158456325028528675187087900674", // OSMO identifier on Osmosis
                                              {
                                                "amount": {
                                                  "intercept": "0",
                                                  "slope": "1000000000000000000"
                                                },
                                                "is_unit": false
                                              }
                                            ]
                                          ]
                                        }
                                      }
                                    ]
                                  }
                                }
                              }
                            ]
                          }
                        }
                      }
                    ]
                  },
                  "assets": [
                    [
                      "158456325028528675187087900673", // this PICA identifier on Centauri
                      "1234567890", // When sending amount, in program amount must be equal to CW transaction amount
                    ]
                  ]
                },
                "tip": "centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n" // any address, use self for now
              }
            }
```

This is full execute messages for Wasmd. Please follow Wasmd/Notional/Osmosis how to send message for execution.

As alway - ensure you have funds. If you do not - please trace errors which tell you about this.

If you think something not working. Remove `Exchange` and `Spawn`. Do only `Transfer`, than add back `Exchange`. Observer success on each step.

I use `nix run "composable#xc-swap-pica-to-osmo"` the way to above program on devnet.

On `mainet`, after entering `centauri-mainnet` shell, I do this `$BINARY tx wasm execute centauri1c676xpc64x9lxjfsvpn7ajw2agutthe75553ws45k3ld46vy8pts0w203g "$(cat swap-pica-to-osmosis.json)" --from=dz -y --gas=5000000 --amount=123456789000ppica`

#### Tracing


All events are prefixed by `cvm.` as raised by CVM. All logs are prefixed by `cvm::` as logged by CVM contracts.

On sender side look for wasmd `cvm` prefixed events, specifically `cvm.gateway.bridge.track.added` if packet was sent from Centauri.

`cvm.interpreter.exchange.succeeded` indicated swap success on Osmosis.

Events prefixed `cvm.interpreter.` trace deep execution of program in `interpeter`. All interpreter events can be seen by [generating schema](./cosmwasm/README.md).

All CVM events are wrapped around by IBC and wasmd modules events as documented by relevant parties.

Some `cvm` events are prefixed with `wasm-` by wasmd. 

Very specific event is 
`wasm-cvm.interpreter.instantiated` with `_contract_address`, which may be equal `centauri12u8s70drvm6cg4fc6j93q0q3g5nw6rvk926rjctx96md4fttedaq787pyl`. 

This address will be used to query `interpreter` state. 

In generally Celatone and other generalized indexers show execution very well. I happens according to sequence diagram in CVM description.



### Query state of contract 


So after you generated schema, you will see all queries you can do. 

Interpreter has `State` query which will dump whole state of interpreter.

You can follow CW20 and Cosmos Bank guide to get amounts of assets on `interpreter` address.


Next is example of getting CVM state of `interpreter`:
```sh
(devenv) bash-5.2$ $BINARY query wasm  contract-state smart centauri12u8s70drvm6cg4fc6j93q0q3g5nw6rvk926rjctx96md4fttedaq787pyl '{
"state" : [] }'

{"data":{"result_register":{"Err":"codespace: client, code: 29"},"ip_register":0,"owners":["centauri176cs0sw6awmc3jvmewcfqmtc08l4wf8jrrka208xnnkprset6kkqh2uwdx"],"config":{"gateway_address":"centauri176cs0sw6awmc3jvmewcfqmtc08l4wf8jrrka208xnnkprset6kkqh2uwdx","interpreter_origin":{"user_origin":{"network_id":2,"user_id":"63656e7461757269317171306b376435366a7575376834396172656c7a677730396a6363646b3873756a7263726a64"},"salt":"737061776e5f776974685f6173736574"}}}}

```

Field details are documented in Rust doc comments and in schema (generated from doc comments).


Failure can happen on IBC and WASMD level, without CVM executed to point where it can issue events. 
For this case please follow IBC and WASMD guides to track execution (generalized indexers like Numia, Mintscan and Cosmos Indexer are super useful in this case).

### Try 

Replace DOT asset id and DOT<->OSMO pool identifier identifier in `execute` message body. Try execute.

### Unstuck funds

Here is program to unstuck funds on `interpreter`.

So what it does - it transfers 1 PICA to Osmosis, and than transfer some assets on Osmosis `interpreter` to some account. 

Also `interpreter` implements CW1 proxy contract allowing user to unstuck funds from Osmosis directly (just send CW20 or Bank transfer message on behalf of `interpreter`).

```json
            {
              "execute_program": {
                "execute_program": {
                  "salt": "737061776e5f776974685f6173736574", // retain same salt to talk to same interpreter
                  "program": {
                    "tag": "137061776e5f776974685f6173736574",
                    "instructions": [
                      {
                        "spawn": {
                          "network_id": 3, 
                          "salt": "737061776e5f776974685f6173736574",
                          "assets": [
                            [
                              "158456325028528675187087900673", // PICA on Centauri
                              {
                                "amount": {
                                  "intercept": "1234567890", // amount to move to Osmosis, but be same or larger than moved to interpreter
                                  "slope": "0"
                                },
                                "is_unit": false
                              }
                            ]
                          ],
                          "program": {
                            "tag": "137061776e5f776974685f6173736574",
                            "instructions": [
                              {
                              "transfer": {
                                "to": {
                                  "account": "AB9vNpqXOevUvR5+JDnlljDbHhw=" // base64 encoded 32 byte account to deposit assets
                                },
                                "assets": [
                                  // he we move all 100% assets we know about
                                  [
                                    "237684487542793012780631851009",
                                    {
                                      "amount": {
                                        "intercept": "0",
                                        "slope": "1000000000000000000"
                                      },
                                      "is_unit": false
                                    }
                                  ],
                                  [
                                    "237684487542793012780631851010",
                                    {
                                      "amount": {
                                        "intercept": "0",
                                        "slope": "1000000000000000000"
                                      },
                                      "is_unit": false
                                    }
                                  ]
                                ]
                              }
                            }
                          }
                        }
                      }
                    ]
                  },
                  "assets": [
                    [
                      "158456325028528675187087900673", 
                      "1", // just something
                    ]
                  ]
                },
                "tip": "centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n" 
              }
            }
```


## Usage from TypeScript

You can find types and simple client and CosmWasm JSON schemas in https://www.npmjs.com/package/cvm-cw-types package.

Example of usage located in https://github.com/ComposableFi/composable/blob/main/code/cvm/cvm.ts .

For usage with React consider generating React state and query integration using cosmwasm client generator from JSON schemas.
