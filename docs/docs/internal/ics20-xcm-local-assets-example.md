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

One *pallet-ibc* maps *DOT* to *2* string and sends via `transfer` call.

`NOTE: 2 is mapped to string by *assets* system of chain. Simples case *to_string* call`

`NOTE: so DOT asset has remote in *xcm assets*, but for *pallet-ibc* transfer it is like local`

**Picasso**

Upon receive of ICS20 transfer of *2*  *pallet-ibc* maps to *transfer/channel-0/2* asset, and then *pallet-ibc* asks *assets* maps it to its local *DOT = 1002*. 

`NOTE: well known *transfer* is *IBC Source Port on Composable*, and *channel-0* is *IBC Source Channel on Composable*(with counter)`

`NOTE: So we can know before send, what prefix would be just after opening channel` 

`NOTE: we could map to same number for convenience, so DOT could be 2 on both chains`

### DOT from Picasso to Polkadot

Let transfer back.

**Picasso**

User calls *pallet-ibc* *transfer* with asset *DOT = 1002*.

`DOT` local asset is mapped to became *transfer/channel-0/2* and sent via IBC.

`NOTE: so you see we mapping local asset to IBC asset here`

**Composable**

Asset prefix removed by *pallet-ibc* to form *2* string. And mapped to `DOT = 2` by *assets* *parse*.

`2` is mapped to `XcmLocation(parents = 1, junctions = Here)` remote location by *assets*.

XCM sent.

**Polkadot**

Polkadot maps ` XcmLocation(parents = 1, junctions = Here)` to ` XcmLocation(parents = 0, junctions = Here)` to Native, zero(0) asset.

### Generalization

We have chains *A*, *B*, *C*, *D*.

#### Topology *A - XCM - B - IBC - C - XCM - D*


Real world, Polkadot -> Composable -> Picasso -> Moonbeam.

**Forward**

A. *XcmLocation(< as it is here>)*

B. *XcmLocation(< as it sent by A>) -> XcmLocation(< as it is for Here to route to A>) -> LocalIdOnB -> LocalIdOnBAsString -> IBC-ICS20 prefixed denomination(LocalIdOnBAsString)*

C. *IBC-ICS20 prefixed denomination(LocalIdOnBAsString) -> IBC-ICS20 prefixed denomination(sourcePortOnB/sourceChannelOnB/LocalIdOnBAsString) -> LocalIdOnC -> XcmLocation( < LocalIdOnC as here> )*

`NOTE: See how asset above also it is foreign because from B, but on bridge switch it became `local` and mapped to XCM remote`

`NOTE: Invariant is that foreign asset has can have 1 remote location, but mapped as local to all other bridges`

D. *XcmLocation( < LocalIdOnC as here> ) -> XcmLocation( < LocalIdOnC prefixed with route from Origin > ) -> LocalIdOnD*

#### Others

Just list we can expand as reasonable to run in future:

1. *A - IBC - B - XCM - C - IBC - D*

2. *A - IBC - B - IBC - C - IBC - D*


## How `assets routes` are created

Currently ICS20 allows to send only 1 asset which are allowed to pay fee for storage(ED) on destination.

Governance should allow such assets explicitly.

### For DOT

**Composable**


Governance defines prefix of *XcmLocation(parents = 1, junctions = Here)* to as well known. 

Governance opens channel to *IBC Picasso* and gets well known *transfer/channel-0/* prefix.

Above are "handshakes".


Governance defines bimapping of *XcmLocation(parents = 1, junctions = Here)* to *2*. 

Governance makes *2* asset as payable(defines conversion ratio to Native). 

Governance makes *DOT* bimap to metadata (not used for fees or transfers). But used in user experience. 

**Picasso**

Governance defines bimap *portOnComposable/channelOnComposable/2* to *1002*.

Governance makes *DOT* bimap to metadata (not used for fees or transfers). But used in user experience. 

`NOTE: DOT symbol may or may be not different value`

## References

[ICS20: Fungible Token Transfer](https://github.com/cosmos/ibc/tree/main/spec/app/ics-020-fungible-token-transfer)

[CW20 Spec: Fungible Tokens](https://github.com/CosmWasm/cw-plus/blob/main/packages/cw20/README.md)

[RFC013](../../../rfcs/0013-redesign-assets-id-system.md)

https://github.com/CosmWasm/cw-plus/blob/main/packages/cw20/src/denom.rs

https://github.com/paritytech/xcm-format

https://github.com/cosmos/ibc/tree/main/spec/app/ics-029-fee-payment

https://github.com/cosmos/cosmos-sdk/blob/main/types/coin.go

https://ibc.cosmos.network/main/architecture/adr-001-coin-source-tracing.html

https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/coin.rs

https://media.githubusercontent.com/media/cosmos/ibc/old/papers/2020-05/build/paper.pdf