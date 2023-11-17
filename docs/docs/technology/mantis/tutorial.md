# Overview

This tutorial describes part of MANTIS describing running solver node and user to post problems. 


## Deployments

| chain      | stage   | id                                                                  |
| ---------- | ------- | ------------------------------------------------------------------- |
| centauri-1 | mainnet | centauri1lnyecncq9akyk8nk0qlppgrq6yxktr68483ahryn457x9ap4ty2sthjcyt |
| osmosis-1  | mainnet |                                                                     |

## Solver node

Documentation to run solver node is located in https://github.com/ComposableFi/composable/blob/main/code/mantis/node/README.md .

Solver observer user orders on chain, and find matches, so they can exchange. 

If solver does not find match, formulates cross chain route.

## User posts problems

Example of problem you can see is here:

```json

```

You can try live code with https://github.com/ComposableFi/composable/blob/main/code/cvm/mantis.ts .


## Deploy your own contracts

```sh
$BINARY tx wasm store $ORDER_WASM_FILE --from dz --gas=auto

$BINARY tx wasm instantiate 18 '{"admin": "centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k", "cvm_address" : "centauri1wpf2szs4uazej8pe7g8vlck34u24cvxx7ys0esfq6tuw8yxygzuqpjsn0d"}' --label "mantis_order_1" --admin centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k --gas=auto --from=dz
```