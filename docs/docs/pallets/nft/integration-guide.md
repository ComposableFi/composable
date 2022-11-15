# fNFT Integration Guide

[**Pallet Overview & Workflow**](../pallet-name.md)

## Integration Status

| Dali | Picasso | Composable |
|------|---------| ---------- |
| No   | No      | No         |

## Setup / Configuration

<!--*Include any notes about pallet lifecycle or states. A state diagram that notes
transition requirements if you're feeling fancy*-->
 
Pallet fNFT is an abstract implementation for Financial NFTs. A user facing, mutating API is provided by other pallets.
fNFT's have four states: Create, update, transfer + (re-)proxy and burn. 
These states generally transition linearly, though the update state can occur independently.

During the create state fNFTs can be minted and collections can be created (initialized) and inserted. 
During the update state functions to inspect and edit attributes of minted assets and collections can be run.
During the transfer +(re)proxy state transfers and proxy configurations can be conducted.
During the burn state functions to permanently remove fNFT assets can be performed.

No automatic state transitions occur.

## RPC & Data Retrieval

<!--*RPCs w/ links to cargo docs?*-->

## Subsquid Data Retrieval

<!--*Not required yet since we have no subsquid yet* -->

## Locally Consumed Types
<!--*Types the pallet consumes, potentially linking to pallet#config docs* -->
### Types

- `MaxProperties` - Numeric type representing the maximum number of assets that can be owned by an account
- `FinancialNftCollectionId` - Unique identifier for fNFT collections
- `FinancialNftInstanceId` - Unique identifier for fNFT instances
- `ProxyType` - Array type representing the permissions allowed for a proxy account
- `AccountProxy` - Function setting the owning account of an fNFT as a delegate for the fNFT asset_account
- `ProxyTypeSelector` - 
### Constants

- `PalletId` - The unique identifier of this pallet

## Calculations & Sources of Values

<!--*"Provide calculations of APY or APR if any and mention the source of all values
that need to be fetched from the chain/backend/subsquid or any other data
source"* -->

## Extrinsic Parameter Sources

<!--*Document sources of extrinsic parameters, hard coded, calculated on the front
end, user provided*-->

Due to the nature of this pallet, its functions are not defined as extrinsics, therefore the source of all 
parameters is the higher level pallet extending a user facing, mutating API.

## Pricing Sources

<!--*"Pricing sources are a must have if any Zeplin designs show users values in USD
$"*-->