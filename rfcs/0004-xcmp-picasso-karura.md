# Overview

Details of Karura open channels proposal for https://karura.subsquare.io/democracy/proposal/9

## Preimage

[0x330001010002100004000000000b00204aa9d10113000000000b00204aa9d10100060003009435775c1802083c01270800003c0027080000e8030000009001000d010004000100411f](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkarura-rpc-3.aca-api.network%2Fws#/extrinsics/decode/0x330001010002100004000000000b00204aa9d10113000000000b00204aa9d10100060003009435775c1802083c01270800003c0027080000e8030000009001000d010004000100411f)


## Proposal


```markdown
Picasso aims to become the DeFi hub of Kusama, connecting parachains with the broader Cosmos ecosystem. Through opening this XCM channel, Karura assets will have access to Picasso dapps, thereby increasing the utility and TVL of aUSD and other Acala tokens.

For example, we will setup pools for aUSD and KAR on Pablo, as well as integrating them within our later released pallets such as CosmWasm. This opens up the possibility to also support these tokens within our IBC bridge that will launch in the future too.

### Flow

- Accepts existing open request channel from Picasso
- Sends open request to Picasso

### Requires

This requires 12 KSM to be on the target Parachain 2000(Karura) account.


### Detailed flow

- Sends XCM message to Kusama
- Puts 2 KSM into holder from Parachain 2000 account
- Sets 2 KSM as payment asset
- Executes batch transaction https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode/0x1802083c01270800003c0027080000e803000000900100
- Accepts open channel request from Picasso 2087
- Sends open channel request to Picasso 2087 with default parameters
- Remaining fee assets are returned back to Parachain 2000 account
```