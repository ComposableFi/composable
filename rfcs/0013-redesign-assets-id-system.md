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
asset metadata (ticker-number, decimal precision, and ratio)

* Support the traits from `frame_support::traits::tokens::fungibles`
  * Specifically `MutateHold`

* Ability to mint mintable assets and the inability to mint non-mintable assets

* Support the existing local asset IDs we have hard-coded

* Utilize both pallet-balances and pallet-assets from Parity for maintaining 
balances within our consensus system

# Other Solutions

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

Our current asset router (also called pallet-assets, to be renamed to asset 
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

## Declare Two pallet-asset Instances

<!-- TODO -->

## Update Asset Registry

<!-- TODO -->

## Create Asset Manager

Calling this Asset Manager instead of Asset Router to avoid confusion between 
this and Asset Registry when abbreviated with "AR".

<!-- TODO -->

## Migrate Hard-Coded Assets

<!-- TODO -->

# Glossary

* **Multi-Location** - A way to identify a single asset in the scope of multiple
consensus systems

* **Local Asset ID** - A way to identify a single asset within the scope of a
single consensus system

* **Ticker-Number** - Small set of letters commonly used to identify an asset 
(PBLO, PICA, BTC, USDT, POOP)

* **decimal precision** - In the context of some amount of a single fungible asset,
the number of digits after the decimal point

* **ratio** - The value ratio between our consensus system's native asset and 
another asset

* **mintable asset** - An asset we can mint within our consensus system

* **non-mintable asset** - An asset we can **NOT** mint within our consensus 
system
