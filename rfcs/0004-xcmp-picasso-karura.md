# Overview

This is guide on how to open XCMP channel to Karura and register PICA token here.

This proposal does not covers `paper work` around tech stuff as it is described in Acala docs. 

- [Overview](#overview)
  - [Prerequisites](#prerequisites)
  - [Picasso to Karura](#picasso-to-karura)
    - [Test Dali to Rococo](#test-dali-to-rococo)
  - [Accept Picasso to Karura](#accept-picasso-to-karura)
  - [Karura to Picasso](#karura-to-picasso)
  - [Accept Karura on Picasso](#accept-karura-on-picasso)
  - [Karura assets on Picasso](#karura-assets-on-picasso)
  - [Picasso tokens on Karura](#picasso-tokens-on-karura)
  - [Reference](#reference)

## Prerequisites

**Picasso**

Ensure have at least 100 KAR on Karura and 20 KSM on Kusama and can do democracy on Picasso.

Create identity on Kusama and Picasso.

**Rococo**

<!-- cspell:disable-next -->
Root is `5D2cjLGNWibiSsEh4oPXo8MmBTkeptrpuiUdEKRMauUn1TkZ` and make sure it has 20 ROC.

<!-- cspell:disable-next -->
Parachain is `5Ec4AhNu5P23CC2GdrmsC8HYBKiidqhPnfSfRQPaH9swr2b7` and make sure it has 20 ROC.

Transfer to [dali-rococo] accounts some amounts.

## Picasso to Karura

That to be executed https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode/0x3c00d0070000e803000000900100 on behalf of  parachain via next

run https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode/0x290001010002100004000000000b0060defb740513000000000b0060defb74050006000700f2052a01383c00d0070000e8030000009001000d0100040001009d20

### Test Dali to Rococo

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rpc.polkadot.io#/extrinsics/decode/0x170000080000e803000000900100

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.composablefinance.ninja#/extrinsics/decode/0x290001010002100004000000000b0060defb740513000000000b0060defb74050006000700f2052a0138170000080000e8030000009001000d0100040001009d20

## Accept Picasso to Karura

That should be executed 

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama.api.onfinality.io%2Fpublic-ws#/extrinsics/decode/0x3c0127080000

on behalf of 

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-2.aca-api.network%2Fws#/extrinsics/decode/0x3300010100020c000400000000070010a5d4e81300000000070010a5d4e80006010700e8764817183c0127080000

**Democracy**

Encode above preimage via `Submit preimage` via https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura.api.onfinality.io%2Fpublic-ws#/democracy 

Then `Submit proposal`.

## Karura to Picasso

This should be executed

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama.api.onfinality.io%2Fpublic-ws#/extrinsics/decode/0x3c0027080000e803000000900100

On behalf of 

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-0.aca-api.network#/extrinsics/decode/0x3300010100020c000400000000070010a5d4e81300000000070010a5d4e80006010700e8764817383c0027080000e803000000900100

**Democracy**

Encode above preimage via `Submit preimage` via https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura.api.onfinality.io%2Fpublic-ws#/democracy 

Then `Submit proposal`.

## Accept Karura on Picasso

This should be executed

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama.api.onfinality.io%2Fpublic-ws#/extrinsics/decode/0x3c01d0070000

executed on behalf of 

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode/0x2900010100020c000400000000070010a5d4e81300000000070010a5d4e80006010700e8764817183c01d0070000


## Karura assets on Picasso

Need to ensure token exists and on deposit on Karura, register pay your gas fee on Picasso, and transfer from Picasso.


**On [karura-kusama]**

- Check `system.account` storage you have native token.
- Check there is `assetRegistry.assetsMetadata` for `ForeignAssetId(7)` and `tokens.totalIssuance` for it is good.
- Use `dex.swapWithExactSupply` to make swap onto target asset (may require doing routing over intermediate currency) .


**On Picasso**

Register Karura token like this 

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.composablefinance.ninja#/extrinsics/decode/0x3700010200411f0608008140420f0000000000000000000000000001000064a7b3b6e00d0000000000000000010c000000


Examples:

```
Asset Name: Karura Native Token 
Asset Symbol:  KAR
Decimals:  12
existentialDeposit: 0.1
Multilocation: { parents: 1, interior: { X2: [ {Parachain: 2000}, {GeneralKey: 0x0080} ]}}

Asset Name: Acala Dollar
Asset Symbol:  AUSD
Decimals:  12
existentialDeposit: 0.01
Multilocation: { parents: 1, interior: { X2: [ {Parachain: 2000}, {GeneralKey: 0x0081} ]}}

Asset Name:  Karura Liquid KSM
Asset Symbol:  LKSM
Decimals:  12
existentialDeposit: 0.0005
Multilocation: { parents: 1, interior: { X2: [ {Parachain: 2000}, {GeneralKey: 0x0083} ]}}
```

**On Karura again**

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rococo.aca-dev.network#/extrinsics/decode/0x360000800010a5d4e80000000000000000000000010102009d2001002aa47c41b763a16946b6cc7e051174877b14fafe5d8daf075b0e39e2398c8e4c00e40b5402000000

Repeat for other token. Validate via `events` and via `tokens` storage.

## Picasso tokens on Karura

**KAR**

You can transfer KAR back, it will use Karura identifier for that asset and send it Karura using `xtokens`

**PICA**

Create preimage and submit proposal via https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-1.aca-api.network#/democracy

to enact https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-1.aca-api.network#/extrinsics/decode/0x7a000001019d20105049434110504943410c00ca9a3b000000000000000000000000

for `PICA` token.

## Reference

https://docs.acalaswap.app/developer-guides/create-a-new-token

https://wiki.acala.network/build/development-guide/composable-chains/open-hrmp-channel

https://acala.notion.site/Acala-Karura-Token-Listing-Playbook-c6b97e022ac6402cb15ce3cb419c48e5

https://wiki.acala.network/integrate/integration-1/token-transfer

https://acala.discourse.group/t/open-hrmp-channel-between-composable-picasso-and-karura/918

[kusama]: https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode
[rococo]: https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rpc.polkadot.io#/explorer
[dali-rococo]: https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.composablefinance.ninja#/explorer
[picasso]: https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode