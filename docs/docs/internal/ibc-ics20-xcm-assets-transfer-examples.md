# Overview

Purpose of this document to show simple IBC-ICS20 and XCM transfers flows.

Describes assets identifiers on each chain and asset allowance scenarios(governance).

> This is not RFC and not initiative. 

> It devoted to help shared understanding on how things work according protocols. 

## How assets are transferred?

Underlying mechanics ensure correctness of transfers, see references for details. Not discussed here. 

All transfers map `on the wire` (remote, foreign) asset identifiers to and from `local`. 

`NOTE: This text uses remote to number mapping as explainer. Hashes for transfers would work too` 
`
Will start from specific examples and go to more real  multihop generalized examples.

Assets transfers require allowance to be transferred and stored on accounts. 
So we touch governance for basic "fees".

### Specific examples of transfer

**Each bold** section outlines separate consensus transformation.

#### DOT from Polkadot to Picasso

Let transfer forward.

**Polkadot**

We send *Native*, zero(0) asset configured as *DOT = XcmLocation(parents = 0, junctions = Here)*.

`NOTE: We use *DOT* symbol in this text, on chain it is never used for transfers, it is just for us to describe flow`

**Composable**

Upon receive we have *Sender = XcmLocation(parents = 1, junctions = Here)* and asset to be *X = XcmLocation(parents = 0, junctions = Here)*.

`NOTE: We use *X*, as *X* is not yet known to be *DOT* `

*pallet-xcm* prefixes asset with sender, and we get *X = XcmLocation(parents = 1, junctions = Here)*.

`NOTE: *pallet-xcm* is really [several pallets and configs](./xcmp/xcmp.runtime.dot)

After that *assets configuration* maps *X* to *DOT = 2*.

`NOTE: *2*is just number for simplicity`.

`NOTE: *assets configuration* can be implemented several ways, possible examples shown later`

*pallet-ibc* calls *assets configuration* to map *2* number to *2* string. 

`NOTE: so *DOT* asset has XCM remote location in *assets configuration*, but for *pallet-ibc* transfer happens if asset is local`

**Picasso**

Upon receive of ICS20 transfer of *2*,  *pallet-ibc* maps it to *transfer/channel-0/2* asset, and then *pallet-ibc* asks *assets configuration*  to map prefixed denomination to local *DOT = 1002*. 

`NOTE: well known *transfer* is *IBC Source Port on Composable*, and *channel-0* is *IBC Source Channel on Composable*(with counter)`

`NOTE: We have to open channels and setup mapping before send` 

`NOTE: We could map to same number for convenience, so *DOT* could be *2* on both chains`

**Done**.

### DOT from Picasso to Polkadot

Let transfer back.

**Picasso**

We call *pallet-ibc* *transfer* with asset *DOT = 1002*.

*DOT* local asset is mapped to became *transfer/channel-0/2* and sent via IBC.

`NOTE: so you see we mapping of local asset to IBC asset here`

**Composable**

Asset prefix removed by *pallet-ibc* to form *2* string. And mapped to *DOT = 2* by *assets configuration*.

*2* is mapped to *XcmLocation(parents = 1, junctions = Here)* remote location by *assets*.

XCM sent.

**Polkadot**

`pallet-xcm` maps ` XcmLocation(parents = 1, junctions = Here)` to ` XcmLocation(parents = 0, junctions = Here)` to Native, zero(0) asset.

**Done**.

### Generalization

We have chains *A*, *B*, *C*, *D*.

#### Topology *A - XCM - B - IBC - C - XCM - D*

Real world would be Polkadot -> Composable -> Picasso -> Moonbeam.

**Forward**

A. *XcmLocation(< as it is here>)*

B. *XcmLocation(< as it sent by A>) -> XcmLocation(< rewrite ancestry to point to A>) -> LocalIdOnB -> LocalIdOnBAsString -> IBC-ICS20 prefixed denomination(LocalIdOnBAsString)*

C. *IBC-ICS20 prefixed denomination(LocalIdOnBAsString) -> IBC-ICS20 prefixed denomination(sourcePortOnB/sourceChannelOnB/LocalIdOnBAsString) -> LocalIdOnC -> XcmLocation( < LocalIdOnC as here> )*

`NOTE: See how asset above also it is foreign because from B, but on bridge switch it became local and mapped to XCM remote`

`NOTE: Invariant is that foreign asset has can have one and only one remote location, but configured as "local" when sent to other channels.`

D. *XcmLocation( < LocalIdOnC as here> ) -> XcmLocation( < LocalIdOnC prefixed with route from Origin > ) -> LocalIdOnD*

> Local asset has no true remote location (None), foreign asset has one and only one remote location.

#### Try other transfer flows

Just list we can expand as reasonable to run in future:

1. *A - IBC - B - XCM - C - IBC - D*

2. *A - IBC - B - IBC - C - IBC - D*

Multihop IBC and XCM are supported.

### Some possible user experiences

In case of PBLO. On Picasso is *100*. So it is Local asset. When sent via IBC to other chain, it will get IBC prefix mapping, app can show *ibcPBLO* because of that. 

In case of USDT, USDT from Statemine can be show as xcmUSDT(because it is common good parachain prefix). USDT from Cosmos can be ibcUSDT. 
Later, when there would be IBC/XCM to ETH, new *assets* module to be created which classifies not only bridge, but prefix(channel) used.

`NOTE: XCVM/CW to be defined in some other document later, likely that would be own bridge type`

## How `assets routes` are created

Currently ICS20 allows to send only 1 asset which are allowed to pay fee for storage(ED) on destination.

Governance should allow such assets explicitly.


### For DOT from Polkadot to Picasso 

**Composable**

Governance defines prefix of *XcmLocation(parents = 1, junctions = Here)* in *pallet-xcm* (and open relevant channel). 

Governance opens channel to *IBC Picasso* and gets well known *transfer/channel-0/* prefix.

Above are "handshakes".


Governance defines bimap of *XcmLocation(parents = 1, junctions = Here)* to *2*. 

Governance makes *2* asset as payable(defines conversion ratio to Native). 

Governance makes *2* bimap to metadata (may be "DOT" symbol, not used for fees or transfers). But used in user experience. 

**Picasso**

Governance defines bimap *portOnComposable/channelOnComposable/2* to *1002*.

Governance makes *DOT* bimap to metadata (not used for fees or transfers). But used in user experience. 

`NOTE: DOT symbol may or may be not different value. It can be prefixed *ibc() depending if there is *remote* IBC mapping for it`

`NOTE: multihop infrastructure for user can be built on top later` 

### On local and remote difference regarding expected configuration

#### Foreign asset
Has well known remote prefix (like XCM origin or IBC port), suffix part is fully controlled by counterparty and may vary.
When received is usually minted, and when send back to origin is usually burnt. 
`NOTE: If send downward or to sibling bridge, start acting as local asset for that end, but still yet on this chain identifies as foreign`
`NOTE: In future when IBC will support transfer of multiple assets and we will support non sufficient assets, 
may consider define prefix and wildcard to avoid storing big one to on map and become more permissonless`

#### Local asset 
Does not have remote IBC or XCM location which indicates that it was bridged. 
May have some convetion how to form prefix and well known suffix when sent sibling or downward by bridges.
The convetion usually idenfifies local consesus protocol/pallet/contract which *governs* asset.
Is escrowed on sent, does not allows transer back more than escrowed.

#### Teleport assets
Considers some foreign and local assets to be equivalent. Likely should be separate asset configuration.


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