# Overview

Purpose of this document to show simple IBC-ICS20 and XCM transfers flows.

Describes assets identifiers on each chain and asset creation allowance scenarios.
.
## How assets are transferred?

Underlying mechanics ensure correctness of transfers. 

All transfers map on the wire(remote, foreign) asset identifiers to local. 

Will start from specific examples and go to more near reach multihop generalized examples.

Assets transfers require allowance to be transferred and stored on accounts. 
So we touch governance and fees too.

### Specific examples of transfer

**Each bold** section outlines separate consensus.

#### DOT from Polkadot to Picasso

Let transfer forward.

**Polkadot**

We send *Native*, zero(0) asset encoded as *DOT = XcmLocation(parents = 0, junctions = Here)*.

`NOTE: We use *DOT* symbol in this text, on chain it is never used for transfers, it is just for us to maintain flow`

**Composable**

Upon receive we have *Sender = XcmLocation(parents = 1, junctions = Here)* and asset to be *X = XcmLocation(parents = 0, junctions = Here)*.

`NOTE: We use X, as X is not yet known to be DOT`

*pallet-xcm* prefixes asset with sender, and we get *X = XcmLocation(parents = 1, junctions = Here)*.

`NOTE: *pallet-xcm* is really [several pallets and configs](./xcmp/xcmp.runtime.dot)

After that *xcm-to-local(X)* is called, and we get *DOT = 2*.

`NOTE: 2 is just number for simplicity, real implementation of local asset id may be hash`

One *pallet-ibc* maps *DOT* to *transfer/channel-0/2* and sends via `transfer` call.

`NOTE: 2 is mapped to string by *assets* system of chain. Simples case *to_string* call`

`NOTE: well known *transfer* is *IBC Port*, and *channel-0* is *IBC Channel* with counter` 

**Picasso**

Upon receive of ICS20 transfer of *transfer/channel-0/2* asset, *assets* maps it to its own *DOT = 1002*. 

`NOTE: we could map to same number for convenience, so DOT could be 2 on both chains`

### DOT from Picasso to Polkadot

Let transfer back.

**Picasso**

User calls *pallet-ibc* *transfer* with asset *DOT = 1002*.

Asset is mapped prefixed to became `transfer/channel-0/2` and send to `Composable`

`NOTE: see that we map 1002 to whole 'transfer/channel-0/2' string representation, so 1002 should be idenfified to be IBC remote`


**Composable**

Asset prefix removed to form `2`string. And mapped to `DOT = 2`

`2` is mapped to `XCM(1, Here)` location.

XCM sent.

**Polkadot**

Polkadot maps `XCM(1, Here)` to `XCM(0, Here)` to Native asset.


### Generalisation



## How `assets routes` are created

Currently ICS20 allows to send only 1 asset which are allowed to pay fee for storage(ED) on destination.

Governance should allow such assets explicitly.

### For DOT

**Composable**

Governance defines mapping of `XcmAsset(1, Here)` to `2`. That can be monotinic number or hash of XcmAsset 

Governance opens channel to `IBC picasso` and gets well known `portOnA/channelOnA/` prefix.

Governance makes DOT asset payable (it has ratio to PICA to pay fees for incoming transaction and messages). Actually this is also bimap.

Governance makes DOT bimap to metadata name (not used for fees or transfers)

**Picasso**

Governance ensres bimap of `portOnA/channelOnA/42` to `55`. In case of number we can gover it to be 42. In case of hash it will eother.

Governance makes DOT bimap to symbol name `ibcDOT <-> 42`

Governance makes DOT bimap to metadata name (not used for fees or transfers)



## Refrences

[ICS20: Fungible Token Transfer](https://github.com/cosmos/ibc/tree/main/spec/app/ics-020-fungible-token-transfer)

[CW20 Spec: Fungible Tokens](https://github.com/CosmWasm/cw-plus/blob/main/packages/cw20/README.md)

https://github.com/CosmWasm/cw-plus/blob/main/packages/cw20/src/denom.rs

https://github.com/paritytech/xcm-format

https://github.com/cosmos/ibc/tree/main/spec/app/ics-029-fee-payment

https://github.com/cosmos/cosmos-sdk/blob/main/types/coin.go

https://ibc.cosmos.network/main/architecture/adr-001-coin-source-tracing.html

https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/coin.rs