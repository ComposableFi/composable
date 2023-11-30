# Overview

This tutorial describes part of MANTIS describing running solver node and user to post problems. 


## Deployments

| chain      | stage   | id                                                                  |
| ---------- | ------- | ------------------------------------------------------------------- |
| centauri-1 | mainnet | centauri1nmrz67mprlngt2tx4qnm0seufsvtjc6v5qzx7jlf7dwlwrxpyc9sp0wxw3 |
| osmosis-1  | mainnet |                                                                     |
| neutron-1  | mainnet |                                                                     |

## User posts problems

Example of problem you can see is here:

```json
{
  "@type": "/cosmwasm.wasm.v1.MsgExecuteContract",
  // ...
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
$BINARY tx wasm store $ORDER_WASM_FILE --from dz --gas=auto -y

$BINARY tx wasm instantiate 30 '{"admin": "centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k", "cvm_address" : "centauri1wpf2szs4uazej8pe7g8vlck34u24cvxx7ys0esfq6tuw8yxygzuqpjsn0d"}' --label "mantis_order_7s" --admin centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k --gas=auto --from=dz -y
```

## GTP bot

You may consider train the bot by asking questions here https://discord.com/channels/828751308060098601/1162324949277622333
