# CVM Tutorial

This document provides an introductory tutorial on the fundamental utilisation of the Composable Virtual Machine (CVM), both from the Command Line Interface (CLI) and the TypeScript Node environment.

Please note that this document assumes prior knowledge of Cosmos, CosmWasm, and blockchain fundamentals. It serves as a reference for users already acquainted with these concepts and aims to provide guidance on working with CVM.

## Prerequisites

Ensure that you have a clear understanding of how to interact with CosmWasm contracts. If you're unfamiliar with this process, we recommend referring to the official test contract guides.

Additionally, make sure you are well-versed in how to make Cosmos RPC calls via the CLI. If you need assistance, consider consulting the official guides or reaching out on the Composable Discord. For mainnet usage, you may want to explore Celatone as an option.

Lastly, it's important to be aware of Bech32 encoding for accounts and the use of IBC prefixed assets within the Cosmos ecosystem. 

### On Mainnet

| chain     | stage   | id                                                                  |
| --------- | ------- | ------------------------------------------------------------------- |
| osmosis-1 | mainnet | osmo126n3wcpf2l8hkv26lr4uc8vmx2daltra5ztxn9gpfu854dkfqrcqzdk8ql     |
| centauri-1  | mainnet | centauri1c676xpc64x9lxjfsvpn7ajw2agutthe75553ws45k3ld46vy8pts0w203g |

If you are interacting with CVM contracts on the Devnet, you can get their address via logs in `/tmp/composable-devnet/` or via calling the RPC.

All of the latest idenfiers and mapping can be found in the [CVM global configration file](https://github.com/ComposableFi/composable/blob/main/code/cvm/cvm.json).

### Shells

Ensure that you have diligently followed the official setup guides provided for Osmosis and Composable Cosmos mainnet shells.

Optionally, for those familiar with Nix and interested in leveraging it, consult the [Nix documentation](../../docs/docs/nix.md) for detailed instructions. To run a development network shell, you can execute the following commands:

For Composable Cosmos Devnet:

```
nix develop "composable#centauri-devnet" --impure
```

For Osmosis Devnet:
```
nix develop "composable#osmosis-devnet" --impure
```

For Composable Cosmos Mainnet:
```
nix develop "composable#centauri-mainnet" --impure
```

For Osmosis Mainnet:
```
nix develop "composable#centauri-mainnet" --impure
```

### Assets

Please follow the multi-hop guide to transfer PICA or DOT (optional). 

For access to tokens on Devnet, you can request tokens via the #cvm-mantis-dev-chat channel on the Composable discord.

## End to end

The following steps outline a user's transaction journey: sending PICA from Composable Cosmos to Osmosis swapping them for OSMO. Identifiers for a similar process with DOT will be released in the near future, but these steps must be manually replicated.

Additionally, this documentation includes commonly utilised queries for obtaining the state of the CVM in both a general context and for specific users.

Finally, a program to address situations where funds become stuck due to cross-chain message failures is given. This is simply a transfer program

#### Prominent identifiers

PICA on Composable Cosmos is 158456325028528675187087900673
OSMO on Composable Cosmos is 158456325028528675187087900674
DOT on Composable Cosmos is 158456325028528675187087900675

PICA on Osmosis is 237684487542793012780631851009
OSMO on Osmosis is 237684487542793012780631851010
DOT on Osmosis is 237684487542793012780631851011


PICA <-> OSMO on Osmosis is 237684489387467420151587012609
PICA <-> DOT on Osmosis is 237684489387467420151587012610


### Transfer Composable Cosmos PICA to Osmosis

Executing the following message enables the transfer of PICA from the sender account to the CVM executor contract and then transfers it to Osmosis.

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

As always - ensure you have funds. If you do not - please trace errors which tell you about this.

If you think something is not working correctly, remove `Exchange` and `Spawn`, and re-attempt the message with only `Transfer` and then only `Exchange` to observe where the error is occuring.

The above program can be executed using `nix run "composable#xc-swap-pica-to-osmo"` on devnet.

On `mainnet`, after using the `centauri-mainnet` shell, run:

```
$BINARY tx wasm execute <account> "<json file path>" --from=<wallet name> -y --gas=5000000 --amount=123456789000ppica`
```

#### Tracing


All events raised by CVM are prefixed by `cvm.` All logs are prefixed by `cvm::` as logged by CVM contracts.

On the sender side, look for wasmd `cvm` prefixed events, specifically `cvm.gateway.bridge.track.added` if the packet was sent from Composable Cosmos.

`cvm.interpreter.exchange.succeeded` indicates the swap was successful on Osmosis.

Events prefixed `cvm.interpreter.` trace deep execution of programs in the `interpeter`. All interpreter events can be seen by [generating schema](./cosmwasm/README.md).

All CVM events are wrapped around by IBC and wasmd modules events as documented by relevant parties.

Some `cvm` events are prefixed with `wasm-` by wasmd. 

A very specific event is `wasm-cvm.interpreter.instantiated` with `_contract_address`, which may be equal `centauri12u8s70drvm6cg4fc6j93q0q3g5nw6rvk926rjctx96md4fttedaq787pyl`. 

This address will be used to query the state of the `interpreter`. 

In general, Celatone and other generalised indexers show execution very well. It occurs according to the sequence diagram in the CVM description.

### Query state of contract 

After the schema is generated, you will be able to view all the queries that can be called. 

You can use the `State` query in the Interpreter to dump the whole state of the interpreter.

You can follow the CW20 and Cosmos Bank guide to retrieve the amount of assets on the `interpreter` address.


The following example is to retrieve the CVM state of the `interpreter`:
```sh
(devenv) bash-5.2$ $BINARY query wasm  contract-state smart centauri12u8s70drvm6cg4fc6j93q0q3g5nw6rvk926rjctx96md4fttedaq787pyl '{
"state" : [] }'

{"data":{"result_register":{"Err":"codespace: client, code: 29"},"ip_register":0,"owners":["centauri176cs0sw6awmc3jvmewcfqmtc08l4wf8jrrka208xnnkprset6kkqh2uwdx"],"config":{"gateway_address":"centauri176cs0sw6awmc3jvmewcfqmtc08l4wf8jrrka208xnnkprset6kkqh2uwdx","interpreter_origin":{"user_origin":{"network_id":2,"user_id":"63656e7461757269317171306b376435366a7575376834396172656c7a677730396a6363646b3873756a7263726a64"},"salt":"737061776e5f776974685f6173736574"}}}}

```

Field details are comprehensively documented in Rust doc comments and within the schema, which is generated from these doc comments.

In some instances, failures may arise at the IBC and WASMD levels before the CVM reaches a stage where it can issue events. In such cases, it is recommended to consult the IBC and WASMD guides to monitor the execution process. Utilising indexers like Numia, Mintscan, and Cosmos Indexers can be particularly beneficial in addressing these scenarios. 

### CVM Exercise 

Replace the asset id for DOT and DOT<->OSMO pool identifier identifier in `execute` message body and then try to execute this.

### Unstuck funds

Here is a program to release stuck funds on the `interpreter`.

This transfers 1 PICA to Osmosis, and than transfers some assets on the Osmosis `interpreter` to some account. 

Additionally, the `interpreter` includes a CW1 proxy contract, allowing users to recover stuck funds from Osmosis directly by simply sending CW20 or Bank transfer messages on behalf of the `interpreter`.



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

If you are using React, consider generating React `state` and `query` integrations using the CosmWasm client generator from JSON schemas.
