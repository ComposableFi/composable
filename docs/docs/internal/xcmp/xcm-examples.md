# Overview

List of useful to have XCM messages working for testing and playbooks.

For details on the general workflow of XCM and transfers [look here](./ping.plantuml)

## Transfer

[Transfer ROC from Rococo to Rockmine](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rpc.polkadot.io#/extrinsics/decode/0x630901000100a10f0100010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c84341010400000000070010a5d4e80000000000)

[Teleport transfer KSM from Rococo to Rockmine to specified account](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rpc.polkadot.io#/extrinsics/decode/0x630101000100a10f01000101002aa47c41b763a16946b6cc7e051174877b14fafe5d8daf075b0e39e2398c8e4c0104000000000b00a0724e180900000000)

[Reserve transfer KSM from Picasso to Kusama to specified account](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode/0x29020101000100010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c8434101040000000002c2eb0b00000000)

[Reserve transfer KSM from Kusama to Karura to specified account](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2F1rpc.io%2Fksm#/extrinsics/decode/0x630201000100411f0100010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c84341010400000000070010a5d4e800000000)

[Reserve transfer KSM from Karura to Kusama by local id](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-0.aca-api.network#/extrinsics/decode/0x3600008200ca9a3b0000000000000000000000000101010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c8434100ca9a3b00000000) ([subscan](https://karura.subscan.io/xcm_transfer/kusama-2000-2973319-13))

[Reserve transfer ROC from Testnet Rococo to Dali](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rpc.polkadot.io#/extrinsics/decode/0x6308010001009d200100010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c8434101040000000003ba5cbf480000000000)


[Reserve transfer KSM from Karura to Picasso by local id](
https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-0.aca-api.network#/extrinsics/decode/0x3600008200ca9a3b000000000000000000000000010102009d200100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c8434100ca9a3b00000000) ([subscan](https://picasso.subscan.io/block/1133228?tab=event))

[Reserve transfer KSM from Dali to Rococo Testnet](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.composablefinance.ninja#/extrinsics/decode/0x2c00040000000000000000000000000000001bb8a3720000000000000000000000000101010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c8434100ca9a3b00000000)

[Low level reserve transfer ROC from Dali to Rococo Testnet](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.composablefinance.ninja#/extrinsics/decode/0x2903020800040001000056346f1d100100000008130001000056346f1d0107006e2e12010d01000400010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c843410084d71700000000)

[Low level reserve transfer KSM from Kusama to Picasso](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode/0x6303020800040000000056346f1d05040000000056346f1d0001009d2008130001000056346f1d000d01000400010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c84341eb16b04c02000000)

[Send simple transaction event from Kusama to Picasso](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode/0x6303020800040000000006286bee05040000000006286bee0001009d200c130001000006286bee00060102286bee5800084c313233343536373839303938373635343332310d01000400010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c8434100ca9a3b00000000)

**Local**

[USDT from Statemine to Dali](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A10008#/extrinsics/decode/0x1f08000200019d200100010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c843410104000002043205011f000b00407a10f35a0000000000)

[USDT from Dali to Statemine](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A10008#/extrinsics/decode/0x2c030100010300a10f0432050400070010a5d4e801000100000700e40b540201010200a10f0100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d00bca06501000000)

## Local assets, mint and swap

[Swap KSM to KAR on Karura](
https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-0.aca-api.network#/extrinsics/decode/0x5d00040008008200800700e876481700)

[Create USDT on local devnet Rockmine](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A10008#/extrinsics/decode/0x02001f1000105553445410555344540600)

[Updated metadata of USDT on local devnet Rockmine](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A10008#/extrinsics/decode/0x02001f1000105553445410555344540600)

[Make USDT payable on local devnet Rockmine](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A10008#/extrinsics/decode/0x02001f12011f00d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d00d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d00d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d00d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da10f0100)

[Mint USDT on local devnet Rockmine](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A10008#/extrinsics/decode/0x1f03011f00d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d130000e8890423c78a)

[Transfer KSM on Kusama to parachain in batch call](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode/0x180204040000706172612708000000000000000000000000000000000000000000000000000052220600)

[Register some Karura token on Picasso](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.composablefinance.ninja#/extrinsics/decode/0x3700010200411f0608008140420f0000000000000000000000000001000064a7b3b6e00d0000000000000000010c000000)

[Register PICA on Karura](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-1.aca-api.network#/extrinsics/decode/0x7a000001019d20105049434110504943410c00ca9a3b000000000000000000000000)

**Testnet Rococo Rockmine**

[Create non payable(not sufficient) asset on Rockmine](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rockmine-rpc.polkadot.io#/extrinsics/decode/0x32005222060000b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c84341e8030000000000000000000000000000)

[Mint non payable(not sufficient) asset on Rockmine](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rockmine-rpc.polkadot.io#/extrinsics/decode/0x32035222060000b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c843410f0000c16ff28623)

[Transfer ROC from Statemine to Dali](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rockmine-rpc.polkadot.io#/extrinsics/decode/0x1f08010101009d2001000101002aa47c41b763a16946b6cc7e051174877b14fafe5d8daf075b0e39e2398c8e4c010400010000070010a5d4e80000000000)

[Transfer USDT from Rockmine to Dali](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-rockmine-rpc.polkadot.io#/extrinsics/decode/0x1f08010101009d200100010100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c843410104000002043205011f00025a62020000000000)

[USDT from Dali to Rockmine](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.composablefinance.ninja#/extrinsics/decode/0x2c04080400000000000000000000000000000000e40b540200000000000000000000008200000000000000000000000000000040420f000000000000000000000000000000000001010200a10f0100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c8434100ca9a3b00000000)

**Local**

[Create USDT asset on Rockmine](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/extrinsics/decode/0x2000630001000100a10f00060200f2052a01000000b02802043203011f03b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c843417656601d)

## Configuration

**Testnet Dali Rococo**

[Sending any asset to Statemine would cost 0.001 KSM](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.composablefinance.ninja#/extrinsics/decode/0x02003702e8030000010100a10f0100ca9a3b000000000000000000000000)

**Picasso**


[Accept open channel request to Picasso from Statemine and Karura](//https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode/0x2900010100020c000400000000070010a5d4e81300000000070010a5d4e8000600035e2e5d713c1800083c01e80300003c01d0070000)

[Sending any asset to Statemine would cost 0.001 KSM](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/extrinsics/decode/0x02003b02e8030000010100a10f0100ca9a3b000000000000000000000000)

**Local**

[Sending any asset to Statemine would cost 0.001 KSM](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/extrinsics/decode/0x02003b02e8030000010100a10f0100ca9a3b000000000000000000000000)

## Other

[Robonomics and Rockmine examples (video)](https://www.youtube.com/watch?v=rygXb21YCDo) 

[Templates to setup XCMP channels (tutorial)](https://docs.substrate.io/reference/how-to-guides/parachains/add-hrmp-channels/) 

[Substrate Stackexchange XCM questions](https://substrate.stackexchange.com/questions/tagged/xcm)

[XCM Simulator Tests(Rust)](https://github.com/paritytech/polkadot/blob/master/xcm/xcm-simulator/example/src/lib.rs)