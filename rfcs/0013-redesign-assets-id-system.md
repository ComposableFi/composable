# Redesign Assets System

This RFC will serve to finalize our approach to refactoring assets-registry and 
removing assets, currency-factory, and our dependence on hard-coded assets.

## Context

Previously, we planned to use currency-factory to reserve asset IDs. However,
currency-factory itself was mostly difficult to maintain legacy code with an
asset ID range system that was overly complex and not helpful to our current 
tech stack.

As it stands, we have removed most of the dependence on currency-factory from
Picasso in favor of hard-coded asset IDs. These hard-coded IDs have allowed us
to conduct an initial release using a simplistic storage migration to create
initial pools for Pablo. However, with the dependence on these hard-coded asset
IDs - we now must conduct additional, more complex, storage migrations to add 
more Pablo pools and more tokens.

Instead of depending on these hard-coded asset IDs, we should instead develop
a dynamic ID system that makes it easy for developers to both interact with 
assets within our consensus system and to retrieve information relevant to a 
given asset.

## Requirements

* There exists a double mapping between multi-locations and local asset IDs
  * Zero or One multi-locations ↔ One asset ID

* Given a way to uniquely identify an asset, one should be able to retrieve 
asset metadata (ticker-symbol, decimal precision, and ratio)

* Support the traits from `frame_support::traits::tokens::fungibles`
  * Specifically `MutateHold`

* Ability to mint mintable assets and the inability to mint non-mintable assets

* Support the existing local asset IDs we have hard-coded

* Supply an Asset Transactor to our XCM Config

# Other Solutions

* [Acala](./0013/acala-analysis.md)
* [Moonbeam](./0013/moonbeam-analysis.md)
* [Parallel](./0013/parallel-finance-analysis.md)

<!-- TODO: These are mostly done, but I will introduce them to the repo in 
their own files -->

# Design

Given the scope of assets, the design will be broken down into several more 
consumable sections.

## Local Asset ID Generation

Each asset within our consensus system should have its own local asset ID. 
Currently, we have hard-coded these local IDs rather than dynamically generating 
them as needed. While existing assets should maintain their current ID, we can 
define a new way to derive asset IDs automatically for assets created in the 
future.

A simple approach to this would be processing a nonce using a fixed-length 
hashing algorithm. Given the fixed length of the hash output, no collisions will
exist between previous hard-coded asset IDs and ones generated from the hash.

## Asset Types and Asset Routing

Within our consensus system, we have two primary types of currency mintable and 
non-mintable. Mintable assets tend to be purely local in their scope and 
function while non-mintable assets tend to come into our consensus system 
externally (via XCM). To ensure no overlap in function between these assets,
they each should belong to their own instance of pallet-assets.

When also considering pallet-balances for our native asset, it becomes clear 
that some amount of assisted routing will be necessary. While routing between 
pallet-assets and pallet-balances is trivial, routing between pallet-balances 
and two instances of pallet-assets becomes more difficult.

Ideally, this routing would not be needed, but both our local and foreign asset 
will, in some instances, interact within the same scope (i.e. Pablo).

Our current asset router (also called pallet-assets, to be renamed to assets 
manager) already completes the more trivial routing. To conduct the non-trivial 
routing, we can depend on asset-registry to inform us if an asset belongs to 
the mintable or non-mintable pallet-assets instance.

This new routing layer can implement traits that pallet-assets may be lacking.

## Adapted Asset Registry

Asset Registry must be updated with distinct paths for both local (mintable) and 
foreign (non-mintable) assets.

Asset Registry will also be responsible for maintaining asset metadata.

# Implementation

To complete the implementation of a new assets' system, we will conduct four 
main tasks.

To-Do comments for these tasks will be added in the Dali 
runtime.

## Declare Two pallet-asset Instances

A great reference for what it looks like to have two instances of pallet-assets 
can be found within the `PureStake/moonbeam` repository. Within the Moonbeam 
runtime, there are two instances of pallet-assets, the first for foreign assets 
and the second for local assets.

While the approach of having two instances of pallet-assets will work for us, 
it's important to note a difference between the goals of Moonbeam and Picasso.

From Moonbeam's `NormalFilter`:
> Normal Call Filter
> We dont allow to create nor mint assets, this for now is disabled
> We only allow transfers. For now creation of assets will go through
> asset-manager, while minting/burning only happens through xcm messages
> This can change in the future

While Moonbeam does not want asset minting or burning to occur outside XCM, 
on Picasso - minting and burning is a core function of Pablo and Staking.

**Steps**:
* Import & Declare pallet-assets
  * To interact with Parity's pallet assets, we must first import it from our 
  patched Substrate repository.
  * We must then declare two instances within our `construct_runtime!` macro.

* Configure Each Instance of pallet-assets
  * NOTE: For details on the configuration of pallet-assets, see [here](https://paritytech.github.io/substrate/master/pallet_assets/pallet/trait.Config.html#)

## Update Assets Registry

Assets Registry will be responsible for registering both foreign and local 
assets. As part of the registration process, asset-meta data should be provided 
and updatable through assets-registry.

### Pallet Configuration

To be responsible for registering both foreign and local assets, assets-registry
will need access to the creation functionality of both instances of 
pallet-assets.

**Steps**:

* Provide configuration item for asset creation to assets-registry

* Provide configuration item for max length of ticker-symbols

### Pallet Storage

To accomplish this, minimal storage modifications will be required. The current 
pallet storage items do not need to be altered, but several new ones will need 
to be created.

**Steps**:

* Add a nonce storage item that will be used for generating foreign asset IDs

* Add storage item for Asset ticker-symbol

### Pallet Functions

To avoid confusion top level `Call` functions should be explicit in 
which asset type they are dealing with. Underlying functions can be more 
generic in their input.

**Steps**:

* Update `register_*_asset` routes

* Update `update_*_asset` routes

### Asset ID Creation

To assist in the routing between both instances of pallet-assets, we can 
dedicate the first 8 bytes to either a pallet ID or other information that 
determines the source of the asset. The remaining 8 bytes of the asset ID can 
be the hash of a nonce provided by the source. A hard-coded list of source 
prefixes can be used to determine which instance of pallet-assets will belong 
to.

To create a hash from the nonce, `sp_core::hashing::blake2_64` can be used

```rust
/// Create a new asset ID derived from `source_prefix` and a `source_nonce`.
///
/// # Parameters
/// * `source_prefix` - The prefix uniquely identifying the source (normally a 
/// `frame_support::PalletId`)
/// * `source_nonce` - A nonce provided by the source, unique to this asset in 
/// the scope of the source
fn create_asset_id(source_prefix: [u8; 8], source_nonce: u64) -> T::LocalAssetId {
  
}
```

**Steps**:

* Create `create_asset_id` function within assets-registry and wrap it in a 
trait

* Provide assets-registry with the new trait to pallets that need to create 
assets, 

* Create nonce within the pallets for asset IDs, call function accordingly

## Create Assets Manager

NOTE: *Calling this Asset Manager instead of Asset Router to avoid confusion 
between this and Asset Registry when abbreviated with "AR".*

Assets Manager will mostly be a migration of the current pallet-assets that we 
created to route between pallet-balances and orml-tokens. The primary difference 
being that assets-manager will also need to handle routing between our two 
instances of Parity's pallet-assets as well as pallet-balances.

As stated in the design, we can depend on information provided by Assets 
Registry to route between our two instances of pallet-assets. 

**Steps**:

* Rename our `pallet-assets` to `pallet-assets-manager`

* Add routes for both instances to existing functions
  * Use first 8 bytes of the asest ID to route the asset

* Implement `MutateHold` on-top of pallet-assets via pallet-assets-manager

* Use a call filter to block calls into the individual instances of 
pallet-assets

* Provide assets-manager as the XCM Asset Transactor

## Migrate Hard-Coded Assets

The data-migration may be handled in two main tasks:

* Append new storage elements to assets-registry
  * Add nonce
  * Add ticker-number to existing tokens
  * Create entries for local assets not previously found in assets-registry

* Migrate existing orml-tokens storage to appropriate instance of pallet-assets

<!-- TODO This should provide more clear details and will in the future -->

# Glossary

* **Multi-Location** - A way to identify a single asset in the scope of multiple
consensus systems

* **Local Asset ID** - A way to identify a single asset within the scope of a
single consensus system

* **Ticker-Symbol** - Small set of letters commonly used to identify an asset 
(PBLO, PICA, BTC, USDT, POOP)

* **decimal precision** - In the context of some amount of a single fungible asset,
the number of digits after the decimal point

* **ratio** - The value ratio between our consensus system's native asset and 
another asset

* **mintable asset** - An asset we can mint within our consensus system

* **non-mintable asset** - An asset we can **NOT** mint within our consensus 
system
