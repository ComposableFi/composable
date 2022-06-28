# Overview

This is guide on how to open XCMP channel to Karura and register PICA token here.

This proposal does not covers `paper work` around tech stuff as it is described in Acala docs. 

## Steps

0. Prerequisites
1. Initiate opening channel
2. Accept channel on Karura
3. Send opening channel from Karura
4. Accept channel on Picasso
5. Make PICA transferable to Karura

## Prerequisites

Ensure have at least 100 KAR on Karura and 20 KSM or Kusama and root on Picasso.

## Initiate open channel


This call `0x3c00d0070000e803000000900100` can be decoded by https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode  ,

Initiate this calls from Picasso as `0x2900010100020c0004000000000700e876481713000100000700e876481700060102286bee383c00d0070000e803000000900100` . 

## Accept open channel

`0x3c0127080000` accept channel on Kusama, should be send from Karura origin with next preimage:

`0x3300010100020c0004000000000700e876481713000100000700e87648170006010700e8764817183c0127080000`

can be decoded by   https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-0.aca-api.network#/extrinsics/decode

## Send Initiate open channel from Karura

`0x3c0027080000e803000000900100` executed on Kusama as send by Karura parachain:

`0x3300010100020c0004000000000700e876481713000100000700e87648170006010700e8764817383c0027080000e803000000900100`  decoded on ` https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-0.aca-api.network#/extrinsics/decode`

## Accept on Picasso

`0x3c01d0070000` on Kusama as sent by Picasso:

`0x2900010100020c0004000000000700e876481713000100000700e876481700060102286bee183c01d0070000` 

## Register PICA

Create preimage and submit proposal via https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-1.aca-api.network#/democracy

where preimage is `0x7a000001019d20105049434110504943410c00ca9a3b000000000000000000000000` 

## Reference

https://docs.acalaswap.app/developer-guides/create-a-new-token

https://wiki.acala.network/build/development-guide/composable-chains/open-hrmp-channel

https://acala.notion.site/Acala-Karura-Token-Listing-Playbook-c6b97e022ac6402cb15ce3cb419c48e5