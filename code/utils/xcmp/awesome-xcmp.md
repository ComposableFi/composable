# Overview

Resources which allows to grasp  XCM and XCMP. General understanding how bridges work will help too. 

## Terminology

XCM - Cross Chain Message.

XCMP - XCM passing.

XCMP can be upward (parachain to relay), downward(relay to parachain) and sibling(lateral, parachain to parachain).

## Conceptual

<https://www.youtube.com/watch?v=XU6dAAQD9UE> - trust and XCMP

<https://docs.moonbeam.network/builders/xcm/overview/>

## How to setup XCMP

- [Polkadot XCM Cross-Chain Asset Transfer Demo](https://medium.com/oak-blockchain/polkadot-xcm-cross-chain-asset-transfer-demo-53aa9a2e97a7)
- <https://medium.com/oak-blockchain/tutorial-polkadot-cross-chain-message-passing-xcmp-demo-with-ping-pallet-f53397158ab4>

## Format of messages

- <https://medium.com/polkadot-network/xcm-part-ii-versioning-and-compatibility-b313fc257b83>
- <https://medium.com/polkadot-network/xcm-part-iii-execution-and-error-management-ceb8155dd166>
- <https://github.com/paritytech/xcm-format/blob/master/README.md>
- <https://research.web3.foundation/en/latest/polkadot/XCMP/index.html>
- <https://medium.com/polkadot-network/xcm-the-cross-consensus-message-format-3b77b1373392>

## XCM(P) design

- <https://www.youtube.com/watch?v=cS8GvPGMLS0>
- <https://www.youtube.com/watch?v=5cgq5jOZx9g>
- <https://substrate.stackexchange.com/questions/37/how-can-i-transfer-assets-using-xcm>
- <https://www.youtube.com/watch?v=wrA9vlPjVPE>
- <https://research.web3.foundation/en/latest/polkadot/XCMP/Opening_closing%20XCMP%20Channel.html>
- <https://www.youtube.com/watch?v=P_yLrFfmLrU>
- <https://blog.quarkslab.com/resources/2022-02-27-xcmv2-audit/21-12-908-REP.pdf>
- <https://github.com/paritytech/polkadot/blob/master/roadmap/implementers-guide/src/messaging.md>
- https://forum.polkadot.network/t/how-xcm-will-actually-be-used-with-xcmp/190

## Generic context

- <https://wiki.polkadot.network/docs/learn-bridges>
- <https://wiki.polkadot.network/docs/learn-parachains>
- <https://polkadot.network/Polkadot-lightpaper.pdf>
- <https://wiki.polkadot.network/docs/learn-crosschain>
- <https://medium.com/web3foundation/polkadots-messaging-scheme-b1ec560908b7>

## Assets

<https://polkadot.network/blog/statemint-becomes-first-common-good-parachain-on-polkadot/>

### Other parachains usage

- <https://www.youtube.com/watch?v=5mspUoK1aIE>
- <https://docs.moonbeam.network/builders/xcm/xc20/overview/>

### Solution and integrations

- ORML + Cumulus, which does not support out of box access to all XMP and as of now opinionated implementations instructions.
- <https://www.youtube.com/watch?v=92w8rVXB5q8> extends and enhances to support more `Transact` patterns
- Checkout Composable [XCVM](../../xcvm/SPEC.md) which

### General cross~~bank~~chain patterns, bridges and helpers

- <https://gendal.me/2013/11/24/a-simple-explanation-of-how-money-moves-around-the-banking-system/>
- <https://medium.com/composable-finance/trustless-bridging-438a6e5c917a>
- <https://research.csiro.au/blockchainpatterns/general-patterns/blockchain-payment-patterns/token-swap/>

### Operations

https://www.youtube.com/watch?v=rygXb21YCDo

<https://docs.substrate.io/reference/how-to-guides/parachains/add-hrmp-channels/>
- https://substrate.stackexchange.com/questions/tagged/xcm

Sends KSM from Picasso to Kusama to specified account:

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode/0x29020101000100010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c8434101040000000002c2eb0b00000000

### Examples of XCM messages


https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rpc.polkadot.io#/extrinsics/decode/0x630101000100a10f01000101002aa47c41b763a16946b6cc7e051174877b14fafe5d8daf075b0e39e2398c8e4c0104000000000b00a0724e180900000000 - teleport transfer KSM from Rococo to statemine


You
10:12 AM
https://github.com/ComposableFi/composable/blob/main/rfcs/0003-xcmp-picasso-statemine.md#make-price-for-usdt
You
10:16 AM
https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode/0x3b00010300a10f043206400b0000000000000000000000000000000a000000000000000000000000000000010000c16ff286230000000000000000000104000000
You
10:20 AM
https://tether.to/en/tether-token-usdt-launches-on-kusama/
You
10:32 AM
[
    [
      {
        ForeignAssetId: 7
      }
    ]
    {
      name: Tether USD
      symbol: USDT
      decimals: 6
      minimalBalance: 10,000
    }
You
10:33 AM
{
      parents: 1
      interior: {
        X3: [
          {
            Parachain: 1,000
          }
          {
            PalletInstance: 50
          }
          {
            GeneralIndex: 1,984
          }
        ]
      }
    }
1 PICA = 0.015 USDT
You
10:34 AM
0.015*10^18
You
10:51 AM
// We take first multiasset
	// Check whether we can convert fee to asset_fee (is_sufficient, min_deposit)
	// If everything goes well, we charge.
You
11:27 AM
https://github.com/search?l=TypeScript&q=org%3AAcalaNetwork+palletinstance&type=Code
You
11:29 AM
https://github.com/AcalaNetwork/acala.js/blob/ec30b5019a7efdbc16c451810a42663eede88ffe/packages/sdk/src/cross-chain/adapters/acala-adapter.ts#L86
You
11:40 AM
https://matrix.to/#/#rococo-faucet:matrix.org