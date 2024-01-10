# Overview

This tutorial describes part of MANTIS describing running solver node and user to post problems. 

## Deployments

### mantis-order contract

| chain      | stage   | id                                                                  |
| ---------- | ------- | ------------------------------------------------------------------- |
| centauri-1 | mainnet |  centauri10tpdfqavjtskze6325ragz66z2jyr6l76vq9h9g4dkhqv748sses6pzs0a |
| osmosis-1  | mainnet |      osmo1lmmer03c6m4al67782qum79ct0ajf87j23v7dpl3udhpv32mny7qhhw4qg    |                                                           |
| neutron-1  | mainnet |                                                                     |

## User posts problemsni

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

## GTP bot

You may consider train the bot by asking questions here https://discord.com/channels/828751308060098601/1162324949277622333
