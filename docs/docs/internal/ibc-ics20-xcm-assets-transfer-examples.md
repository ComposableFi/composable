# Overview

Purpose of this document to show simple IBC-ICS20 and XCM transfers flows.

Describes assets identifiers on each chain and asset allowance scenarios(governance).

> This is not RFC and not initiative. 

> It devoted to help shared understanding on how things work according protocolss. 

## How assets are transferred?

Underlying mechanics ensure correctness of transfers. 

All transfers map `on the wire` (remote, foreign) asset identifiers to `local`.

`NOTE: This text uses remote to number mapping as explainer. Hashes for transfers would work too` 

Will start from specific examples and go to more near reach multihop generalized examples.

Assets transfers require allowance to be transferred and stored on accounts. 
So we touch governance for basic "fees".

### Specific examples of transfer

**Each bold** section outlines separate consensus transformation.

#### DOT from Polkadot to Picasso

Let transfer forward.

**Polkadot**

We send *Native*, zero(0) asset encoded as *DOT = XcmLocation(parents = 0, junctions = Here)*.

`NOTE: We use *DOT* symbol in this text, on chain it is never used for transfers, it is just for us to describe flow`

**Composable**

Upon receive we have *Sender = XcmLocation(parents = 1, junctions = Here)* and asset to be *X = XcmLocation(parents = 0, junctions = Here)*.

`NOTE: We use X, as X is not yet known to be DOT`

*pallet-xcm* prefixes asset with sender, and we get *X = XcmLocation(parents = 1, junctions = Here)*.

`NOTE: *pallet-xcm* is really [several pallets and configs](./xcmp/xcmp.runtime.dot)

After that *assets* maps *X* to *DOT = 2*.

`NOTE: 2 is just number for simplicity, real implementation of local asset id may be hash`.

After *assets* upon callback from *pallet-ibc* map *DOT* to *2* string. 

*pallet-ibc* sends assets ICS20 `transfer` with *2* as *denomination*.

`NOTE: so DOT asset has XCM remote location in *assets*, but for *pallet-ibc* transfer happens if asset is local`

**Picasso**

Upon receive of ICS20 transfer of *2*  *pallet-ibc* maps it to *transfer/channel-0/2* asset, and then *pallet-ibc* asks *assets* map to map prefixed denomination to local *DOT = 1002*. 

`NOTE: well known *transfer* is *IBC Source Port on Composable*, and *channel-0* is *IBC Source Channel on Composable*(with counter)`

`NOTE: We have to open channels and setup mapping before send` 

`NOTE: We could map to same number for convenience, so DOT could be 2 on both chains`

Done.

### DOT from Picasso to Polkadot

Let transfer back.

**Picasso**

We call *pallet-ibc* *transfer* with asset *DOT = 1002*.

*DOT* local asset is mapped to became *transfer/channel-0/2* and sent via IBC.

`NOTE: so you see we mapping of local asset to IBC asset here`

**Composable**

Asset prefix removed by *pallet-ibc* to form *2* string. And mapped to *DOT = 2* by *assets*.

*2* is mapped to *XcmLocation(parents = 1, junctions = Here)* remote location by *assets*.

XCM sent.

**Polkadot**

`pallet-xcm` maps ` XcmLocation(parents = 1, junctions = Here)` to ` XcmLocation(parents = 0, junctions = Here)` to Native, zero(0) asset.

Done.

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

> If chain is source of asset then prefix is done by IBC/XCM configs/pallets before sent as configured.

> If chain is sink of asset then mapping is done by assets Registry/Router

D. *XcmLocation( < LocalIdOnC as here> ) -> XcmLocation( < LocalIdOnC prefixed with route from Origin > ) -> LocalIdOnD*

> Local asset has no remote location (None), foreign asset has one and only one remote location.

#### Try other transfer flows

Just list we can expand as reasonable to run in future:

1. *A - IBC - B - XCM - C - IBC - D*

2. *A - IBC - B - IBC - C - IBC - D*

Multihop IBC and XCM are supported.

### Some possible user experiences

In case of PBLO. On Picasso is 100. So it is Local asset. When sent via IBC to other chain, it will get IBC prefix, app can show ibcPBLO because of that. 

In case of USDT, USDT from Statemine can be show as xcmUSDT(because it is common good parachain prefix). USDT from Cosmos can be ibcUSDT. Later, when there would be IBC/XCM to ETH, new *assets* module to be created which classifies not only bridge, but prefix(channel) used.

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

`NOTE: DOT symbol may or may be not different value. It can be prefixed ibc depending if there is *remote* IBC mapping for it`

`NOTE: multihop infrastructure for user can be built on top later` 

## On local and on the wire encoding


### Number maps


```
32 <-> (parent = 1, pallet = 50, general = 42) # Relay XCM

44 <-> portOnA/challeOnA/77 # IBC-ICS20

7 <-> 7 # Local can be mapped to local  
```

Requires to check map each time and load if from cache (and may be transfer as data for validation).

Can provide nice numbers for important assets and make numbers same on different chains.

### Hash

```
hash(canonicalize(parent = 1, pallet = 50, general = 42)) <-> parent = 1, pallet = 50, key = jsdaSaaE123lasd # Relay XCM

hash(portOnA/challeOnA/jsdaSaaE123lasd) <-> portOnA/challeOnA/jsdaSaaE123lasd # IBC-ICS20

pseudo_hash(7) <-> 7 # Local can be mapped to local  
```

Avoids half reads of storage read by using computation.

In best case if hashes are same size but out of curve, can be blended with public keys of contracts.

In bad case can conflict with other hashes or public keys.


### Hash and canonical path

```
hash(canonicalize(parent = 1, pallet = 50, general = 42)) <-> parent = 1, pallet = 50, key = jsdaSaaE123lasd # Relay XCM

hash(portOnA/challeOnA/canonicalize(parent = 1, pallet = 50, general = 42)) <-> portOnA/challeOnA/canonicalize(parent = 1, pallet = 50, general = 42) # IBC-ICS20

pseudo_hash(7) <-> 7 # Local can be mapped to local  
```

Largest payload on wire.

Allows nicer user experience. 

Hardest to define and maintain.

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
