# Parallel Finance - Assets System Analysis

This document acts as an overview of how assets are managed within Parallel
finance.

## Overview

Parallel Finance manages their assets with the following tech stack:

* Local Assets: 
  * Parity's [pallet-assets](https://paritytech.github.io/substrate/master/pallet_assets/index.html)
  * Parity's [pallet-balances](https://paritytech.github.io/substrate/master/pallet_balances/index.html)
  * Their own [pallet-currency-adapter](https://api-docs.parallel.fi/rustdocs/pallet_currency_adapter/index.html)
* XCM Assets:
  * [orml-xtokens](https://github.com/open-web3-stack/open-runtime-module-library/tree/master/xtokens)
  * Their own [pallet-asset-registry](https://api-docs.parallel.fi/rustdocs/pallet_asset_registry/index.html)
  * [xcm-executor](https://paritytech.github.io/polkadot/doc/xcm_executor/index.html)
  * [xcm-builder](https://paritytech.github.io/polkadot/doc/xcm_builder/index.html)

The handling of XCM and local assets are almost entirely separate workflows. 
They are only brought together through the `AssetTransactors` type.

They are on `polkadot-v0.9.32`.

## Local Assets

To manage both their native asset and other local assets, Parallel uses their
pallet-currency-adapter to route between pallet-assets (local) and
pallet-balances (native). Additionally, this pallet exposes a force locking
mechanism which a single origin is able to use.

Parallel's pallet-currency-adapter is equivalent to Composable's `pallet-assets` 
in terms of what it tries to accomplish.

## XCM Assets

NOTE: The wrapper enum `AssetType` is used for wrapping the XCM `MultiLocation` 
type.

Parallel's pallet-asset-registry primarily maintains a bidirectional mapping 
between a local `AssetId` and the `AssetType`. This is then wrapped by the 
`WrapAssetRegistry` which is a `XcmAssetRegistry`.

## Asset Transactor Combining

The `AssetTransactors` type sits at the top of the Parallel Finance consensus 
system for routing the entrance/exit of assets to/from the consensus system.

Two main transactors make up the `AssetTransactors`: `LocalAssetTransactor` and 
`ForeignFungiblesTransactor`. Priority is first given to the 
`LocalAssetTransactor` type and then `ForeignFungiblesTransactor` if failure 
occurs.

### `LocalAssetTransactor`

The `LocalAssetTransactor` is an instance of Parallel's `MultiCurrencyAdapter` 
struct which implements `TransactAsset` from `xcm-executor`. This implementation 
of the `MultiCurrencyAdapter` uses `pallet-currency-adapter` for its generic 
`Assets` type.

### `ForeignFungiblesTransactor`

The `ForeignFungiblesTransactor` is an instance of the `FungiblesAdapter` struct 
from `xcm-builder` which also implements `TransactAsset`. This implementation of
the `FungiblesAdapter` uses `pallet-assets` for its generic `Assets` type.
