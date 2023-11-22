# Overview

This tutorial describes part of MANTIS describing running solver node and user to post problems. 


## Deployments

| chain      | stage   | id                                                                  |
| ---------- | ------- | ------------------------------------------------------------------- |
| centauri-1 | mainnet | centauri1lnyecncq9akyk8nk0qlppgrq6yxktr68483ahryn457x9ap4ty2sthjcyt |
| osmosis-1  | mainnet |                                                                     |

## User posts problems

Example of problem you can see is here:

```json
          {
            "@type": "/cosmwasm.wasm.v1.MsgExecuteContract",
            "sender": "centauri1mgnu00vn0feumu660y6p7ty5mv58txvhgkr2lu",
            "contract": "centauri1lnyecncq9akyk8nk0qlppgrq6yxktr68483ahryn457x9ap4ty2sthjcyt",
            "msg": {
              "order": {
                "msg": {
                  "wants": {
                    "denom": "ppica",
                    "amount": "1000000"
                  },
                  "transfer": null,
                  "timeout": 2506928,
                  "min_fill": null
                }
              }
            },
            "funds": [
              {
                "denom": "ibc/EF48E6B1A1A19F47ECAEA62F5670C37C0580E86A9E88498B7E393EB6F49F33C0",
                "amount": "1"
              }
            ]
          }
```

Or mainnet transaction https://ping.pub/composable/tx/CA9489EC961BA97AB514A74EEC6BF3B6CD9900C00A031AA3BB80DC343CE85F2D

You can try live code with https://github.com/ComposableFi/composable/blob/main/code/cvm/mantis.ts .


## Deploy your own contracts

```sh
$BINARY tx wasm store $ORDER_WASM_FILE --from dz --gas=auto

$BINARY tx wasm instantiate 18 '{"admin": "centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k", "cvm_address" : "centauri1wpf2szs4uazej8pe7g8vlck34u24cvxx7ys0esfq6tuw8yxygzuqpjsn0d"}' --label "mantis_order_2" --admin centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k --gas=auto --from=dz
```

## Solver node

Documentation to run solver node is located in https://github.com/ComposableFi/cvm/tree/main/mantis 

Solver observer user orders on chain, and find matches, so they can exchange. 

If solver does not find match, formulates cross chain route.