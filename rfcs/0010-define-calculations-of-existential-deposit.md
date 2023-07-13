# Calculation of all On-Chain Existential Deposit Values

Definition of all on-chain Existential Deposit (ED) values. While defining these values, we also
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

## Method

### Initial Standard ED Value

With the following definitions: 

* `local_ed` - The functional ED of an asset on our chain.

* `native_asset_ed` - The ED of our chains native asset.

* `foreign_asset_ratio` - The ratio of 'one in the foreign asset' over 'one in 
our native asset'

The ED of any asset should be defined as follows:
```python
assert(price(amount_of_foreign_asset) == price(amount_of_native_asset))
foreign_asset_ratio = amount_of_foreign_asset / amount_of_native_asset
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

### Well-Known Assets & Asset Registry Storage

For an asset to have its ED be correctly calculated, and therefore to exist on 
chain, it should have its `foreign_asset_ratio` defined in either our Well-Known 
assets list, or stored in Asset Registry.

## Implementation

### Updates to the Functionality of `multi_existential_deposits`

The current implementation of `multi_existential_deposits` does the following:
  
  1. **If an ED is defined for an asset in Asset Registry, return the ED as 
     is.**
  
  2. **Otherwise, pull a hard-coded ED from a `match` statement**.
  
  3. If no matches are found, return `Balance::MAX` which will prune unknown
     assets.
    
It should be updated as follows:

  1. **If an asset ratio is defined for an asset in Asset Registry or as a 
     well-known asset, calculate the standard ED as defined by this document.**
  
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
    ratio: Rational64, // Used to be optional
    decimals: Option<Exponent>,
  ) -> DispatchResultWithPostInfo;
  ```

## Questions

* Will inflation/deflation be a concern as assets enter and leave the chain if
  the ED of a foreign asset is lower/higher on our chain than it is on the
  assets native chain?
  
  > This is not really a solvable problem AFAIK given that we don't control
    validations done on other chains when transferring to Picasso.
  
* Can an RPC expose a function that only exists in the runtime? If not, we may
  need to move `multi_existential_deposits` (or at least its functionality) into
  Asset Registry.
  
  > Moving this functionality into Asset Registry will be necessary for 
  different runtimes to have different asset configurations.
  
* Do we already have an in-use storage for AssetRegistry in Picasso?

  > I think we don't use it at the moment but best assume that we use it, for 
    all purposes.
