# Overview

This RFC example payload and flow is legacy.

With recent upgrade, Parity allowed to open channels [without using 3 hops XCM message for Common Good Parachains](https://github.com/paritytech/polkadot/pull/6155), [message is simple now](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode/0x18020c0402006d6f646c70792f747273727900000000000000000000000000000000000000000070617261e80300000000000000000000000000000000000000000000000000000b00a0724e18093c0727080000e8030000e8030000009001003c07e803000027080000e803000000900100).

Details of Statemine open channel proposal for https://kusama.polkassembly.io/proposal/111

## Preimage

[0x1802080402006d6f646c70792f747273727900000000000000000000000000000000000000000070617261e80300000000000000000000000000000000000000000000000000000b00b01723010a630001000100a10f0204060203002f6859ad011f000101000214000400000000070010a5d4e81300000000070010a5d4e800060003009435775c1802083c01270800003c0027080000e803000000900100140d01010000000004000101006d6f646c70792f74727372790000000000000000000000000000000000000000](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama.api.onfinality.io%2Fpublic-ws#/extrinsics/decode/0x1802080402006d6f646c70792f747273727900000000000000000000000000000000000000000070617261e80300000000000000000000000000000000000000000000000000000b00b01723010a630001000100a10f0204060203002f6859ad011f000101000214000400000000070010a5d4e81300000000070010a5d4e800060003009435775c1802083c01270800003c0027080000e803000000900100140d01010000000004000101006d6f646c70792f74727372790000000000000000000000000000000000000000)


## Proposal

```markdown
Picasso aims to become the DeFi hub of Kusama, connecting parachains with the broader Cosmos ecosystem. Through opening this XCM channel, Statemine assets will have access to Picasso dapps, thereby increasing the utility and TVL of USDT and other Statemine tokens.

For example, we will setup pools for USDT and KSM on Pablo, as well as integrating them within our later released pallets such as CosmWasm. This opens up the possibility to also support these tokens within our IBC bridge that will launch in the future too. 

### Flow

- In a forced transfer of 11 KSM from Kusama treasury (`F3opxRbN5ZbjJNU511Kj2TLuzFcDq9BGduA9TgiECafpg29`) to Statemine (`F7fq1jSNVTPfJmaHaXCMtatT1EZefCUsa7rRiQVNR5efcah`). 10 KSM are used as a deposit for channels. 1 KSM is used to pay transaction fees.

- Send XCM message to Statemine to execute [0x1f000101000214000400000000070010a5d4e81300000000070010a5d4e800060003009435775c1802083c01270800003c0027080000e803000000900100140d01010000000004000101006d6f646c70792f74727372790000000000000000000000000000000000000000](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fstatemine.api.onfinality.io%2Fpublic-ws#/extrinsics/decode/0x1f000101000214000400000000070010a5d4e81300000000070010a5d4e800060003009435775c1802083c01270800003c0027080000e803000000900100140d01010000000004000101006d6f646c70792f74727372790000000000000000000000000000000000000000)

This message does the following:

- Statemine sends message back to Kusama
- Puts 1 KSM into holder from Parachain 1000 account
- Sets 1 KSM as payment asset
- Executes batch transaction [0x1802083c01270800003c0027080000e803000000900100](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama.api.onfinality.io%2Fpublic-ws#/extrinsics/decode/0x1802083c01270800003c0027080000e803000000900100)
- Accepts open channel request from Picasso 2087
- Sends open channel request to Picasso 2087 with default parameters
- Remaining fee assets are returned back to Treasury account `F3opxRbN5ZbjJNU511Kj2TLuzFcDq9BGduA9TgiECafpg29`
```