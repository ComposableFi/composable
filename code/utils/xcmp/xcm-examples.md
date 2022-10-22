# Overview

List of useful to have XCM messages working for testing and playbooks.

General flow for XCM is:

1. Open channels
2. Register foreign assets
3. Obtain assets (mint or swap on DEX)
4. Execute XCM transactions


If any of these steps is missing, parachain XCM messages will not work. 

[Teleport transfer KSM from Rococo to Rockmine to specified account](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rpc.polkadot.io#/extrinsics/decode/0x630101000100a10f01000101002aa47c41b763a16946b6cc7e051174877b14fafe5d8daf075b0e39e2398c8e4c0104000000000b00a0724e180900000000)


[Reserve transfer KSM from Picasso to Kusama to specified account](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode/0x29020101000100010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c8434101040000000002c2eb0b00000000)

[Reserve transfer assets from Kusama to Karura to specified account](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2F1rpc.io%2Fksm#/extrinsics/decode/0x630201000100411f0100010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c84341010400000000070010a5d4e800000000)

[Swap KSM to KAR on Karura](
https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-0.aca-api.network#/extrinsics/decode/0x5d00040008008200800700e876481700)

[Robonomics and Rockmine examples (video)](https://www.youtube.com/watch?v=rygXb21YCDo) 

[Templates to setup XCMP channels (tutorial)](https://docs.substrate.io/reference/how-to-guides/parachains/add-hrmp-channels/) 

[Substrate Stackexchange XCM questions](https://substrate.stackexchange.com/questions/tagged/xcm)

[Transfer ROC from Rococo to Rockmine](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rpc.polkadot.io#/extrinsics/decode/0x630901000100a10f0100010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c84341010400000000070010a5d4e80000000000)

[Transfer ROC from Statemine to Dali](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rockmine-rpc.polkadot.io#/extrinsics/decode/0x1f08010101009d2001000101002aa47c41b763a16946b6cc7e051174877b14fafe5d8daf075b0e39e2398c8e4c010400010000070010a5d4e80000000000)