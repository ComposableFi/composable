# Calculation of all On-Chain Existential Deposit Values

## Abstract

This document aims to provide standards for the initial definition of all 
on-chain Existential Deposit (ED) values. While defining these values, we also
need to ensure that undefined behavior does not emerge from how these values 
interact.

## Background

While the values defined for ED have mostly been consistent, we have so far 
lacked an official standard for how these values are defined. We have also not
ensured that ED values are accessible wherever needed in our runtime, node, and
front-end applications. We must decide on a consistent standard to use for the
initial definition of all ED values on our chain while also allowing for 
extensibility in the future.

## Requirements

* ED values for all assets supported by the chain are available AND consistent
  across our runtime, node, and front-end applications.

* When the ED of asset A is swapped into asset B, the new balance of asset B is
  substantial for ED. In other words, a user cannot "delete" their balance when 
  they convert it into another asset.

* The implementation of our ED value retrieval should allow for the overwriting
  of the standard ED value defined in this document.

## Method

### Initial Standard ED Value

With the following definitions: 

* `local_ed` - The functional ED of a asset on our chain.

* `native_asset_ed` - The ED of our chains native asset.

* `foreign_asset_ratio` - The ratio of 'one in the foreign asset' over 'one in 
our native asset'

The ED of any asset should be defined as follows:
```
local_ed = native_asset_ed * foreign_asset_ratio
```

This method of setting ED ensures requirement 2.

Example
```
asset_a_local_ed = 100_000_000
asset_a_ratio = Ratio(1_000_000_000, 1_000_000_000_000)
asset_b_local_ed = 100_000
asset_a_ratio = Ratio(1_000_000, 1_000_000_000_000)
native_asset_ed = 100_000_000_000

fn native_to_asset(native_balance, asset_ratio) {
  native_balance * asset_ratio
}

fn asset_to_native(asset_balance, asset_ratio) {
  asset_balance / asset_ratio
}

asset_a_ed_as_native = asset_to_native(asset_a_local_ed, asset_a_ratio)
asset_b_ed_as_native = asset_to_native(asset_b_local_ed, asset_b_ratio)
native_as_asset_b = native_to_asset(native_asset_ed, asset_b_ratio)
asset_a_as_asset_b = native_to_asset(asset_a_ed_as_native, asset_b_ratio)

assert(asset_a_ed_as_native == native_asset_ed)
assert(asset_b_ed_as_native == native_asset_ed)
assert(asset_a_ed_as_native == asset_b_ed_as_native)
assert(native_as_asset_b == asset_a_as_asset_b)
```

### Overwriting & Asset Registry Storage

For a asset to have its ED be correctly calculated, and therefore to exists on 
chain, it should have its `foreign_asset_ratio` defined and stored.

If the need arises, an optional ED value can be provided to an asset within
Asset Registry. This will have priority over the default ED calculation and will
be evaluated as is.

## Implementation

### Updates to the Functionality of `multi_existential_deposits`

The current implementation of `multi_existential_deposits` does the following:
  
  1. If an ED is defined for an asset in Asset Registry, return the ED as is.
  
  2. **Otherwise, pull a hard-coded ED from a `match` statement**.
  
  3. If no matches are found, return `Balance::MAX` which will prune unknown
     assets.
    
It should be updated as follows:

  1. If an ED is defined for an asset in Asset Registry, return the ED as is.
  
  2. **If a asset ratio is defined for an asset in Asset Registry, calculate the
     standard ED as defined by this document.**
  
  3. If no matches are found, return `Balance::MAX` which will prune unknown
     assets.
    
### Existential Deposit Retrieval

For use in the front-end, an RPC will need to be made available that exposes the
functionality of `multi_existential_deposits`.
    
### Asset Registry

**Note**: If AssetRegistry storage is already in use on Picasso, we will need to
perform a storage migration in the next runtime upgrade for all of these changes
to be possible.

* The function `AssetRegistry::register_asset` should be updated with the 
  following signature:

  ```rust
  pub fn register_asset(
    origin: OriginFor<T>,
    location: T::ForeignAssetId,
    ed: Option<T::Balance>, // Used to be required
    ratio: Ratio, // Used to be optional
    decimals: Option<Exponent>,
  ) -> DispatchResultWithPostInfo;
  ```

* Additionally, Asset Registry needs to support creating assets with a difined
asset ID. 

  ```rust
  pub fn register_asset_with_id(
    origin: OriginFor<T>,
    asset_id: T::AssetId,
    location: T::ForeignAssetId,
    ed: Option<T::Balance>, // Used to be required
    ratio: Ratio, // Used to be optional
    decimals: Option<Exponent>,
  ) -> DispatchResultWithPostInfo;
  ```

## Quality Assurance

* Manually changing the values of different EDs is not the standard. By default
  EDs should behave as defined in this document.

## Questions

* Will inflation/deflation be a concern as assets enter and leave the chain if
  the ED of a foreign asset is lower/higher on our chain than it is on the
  assets native chain?
  
* Can an RPC expose a function that only exists in the runtime? If not, we may
  need to move `multi_existential_deposits` (or at least its functionality) into
  Asset Registry.
  
* Do we already have an in-use storage for AssetRegistry in Picasso?