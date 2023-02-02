# Overview

Purpose of this document to show simple ICS 20 and XCM transfers flow.

Assets identfiers and encodings describrd on each chain.

Also permissions to create assets and create payable assets described.

The goal of the document to align people around how assets should be handled and monitored in relevant imlementations.

Exact events and exact account derivations are specified elsewehre.



## Legend

XCM, ICS20, IBC are defined elsewhere.

Coin,  fungible asset, token are used interchably in this docuent and actually in relevat specfiicaiton.

## How assets are transfed?

We start from transfers, only asset idenfiers as they will be finally are relevant. 

How assets are created and permissioned is next section.

### Specific examples of transfer

#### DOT from Polkadot to Picasso

**Polkadot**


We send Native, 0 asset encoded as `DOT = XCM(0, Here)`.

**Composable**


Upon receive we have `Sender = Location(1, Here)` and asset to be `X = XCM(0, Here)`.

XCM asset is prefixed with sender prefix, and we get `X = XCM(1, Here)`.

After that `xcm-to-id` is called, and we get `DOT = 42`.

One composable asset is mapped to `transfer/channel-0/42` and send via `pallet-ibc` `transfer` method. 


**Picasso**

Upon receive of ICS20 transfer of `transfer/channel-0/42` asset, it maps it to its own `DOT = 42`. 

`NOTE: we use same asset id for convenience. There is no reason why ID should be the same`


### DOT from Picasso to Polkadot


**Picasso**

User calls `pallet-ibc` `transfer` with asset `DOT = 42`.

Asset is prefixed to became `transfer/channel-0/42` and send to `Composable`


**Composable**

Asset prefix removed to form `/42`. And mapped to `DOT = 42`

`42` is mapped to `XCM(1, Here)` location.

XCM sent.

**Polkadot**

Polkadot maps `XCM(1, Here)` to `XCM(0, Here)` to Native asset.


### Generalisation





## How `assets routs` are created

Here will be used `Governance` to as body which does `operations`. 

Act of governance can be executing storage update on behalf of some `admin` or upgrading `runtime` with hardcoded assets metadata.



### For DOT

**Composable**

Governance defines mapping of `XcmAsset(1, Here)` to 42. That can be monotinic number or hash of XcmAsset 

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