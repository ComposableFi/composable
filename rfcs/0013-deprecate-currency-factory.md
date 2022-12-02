# Depreciation of Currency Factory - Assessment RFC

This assessment aims to gather all relevant details and consequences of 
depreciating Currency Factory and its replacements.

## Reasoning

There are several reasons we stand to benefit from replacing Currency Factory.
These reasons are detailed below:

### Currency Factory Ranges are Confusing
  
The Currency Factory ranges are a confusing abstraction that we don't 
stand to gain from. We really only have two types of currencies native 
(mintable) and non-native (external). However, we currently have many more 
ranges that enforce artificial limitations on various abstractions of these two 
currency types. This complicates the currency creation process and leads to 
confusion while providing no benefit to us. 
Since asset IDs aren't intended for human consumption, there is no requirement for them to be "easily consumable" - the end user should be concerned with the asset's _symbol_, not it's id.

### No Permissionless Asset Creation

> We do not have permissionless asset (non-payable/no sufficient) - it is 
impossible to create no marketable assets (which is needed for 
governance/airdrops).

### Currency Factory is Already Losing Functionality

Through RFCs 0009 and 0010, we decided to move away from storing ED in Currency 
Factory. After the removal of ED storage, Currency Factory was left with two 
uses: reserving asset IDs and storing asset metadata. 

As discussed, Currency Factory overcomplicates the asset ID reservation system. 
As for asset metadata, we don't have standards for formatting this nor do we 
utilize it. Assuming we replace the reservation system, Currency Factory will be 
left without value.

## Other Solutions

The goal of this section is to overview the solutions developed by other networks
in the ecosystem.

### Acala

Acala has four asset classifications:

* Foreign
* Stable
* Erc20
* Native

Acala has separate registration processes for each of these within their Asset 
Registry pallet. Of interest to is are the foreign and native asset registration
process.

#### Foreign Assets

Foreign assets are registered with a `MultiLocation` and `AssetMetadata`. The 
`MultiLocation` then mapped to a `ForeignAssetId` that is determined by a nonce. 
A mapping of `ForeignAssetId` to `MultiLocation` is also created so that either 
can be used to look up the other. The provided metadata provides the assets 
name, symbol, decimals, and minimum balance. The Metadata is indexed by the 
`ForeignAssetId`.

Foreign assets can then be updated by their `ForeignAssetId`, in which case
both the assets `MultiLocation` and `AssetMetadata` can be updated.

#### Native Assets

Native assets are registered with a `CurrencyId` and `AssetMetadata`. The 
`CurrencyId` maps to the `AssetMetadata` and this mapping is used to ensure that 
the asset does not already exist.

Native assets can then be updated by their `CurrencyId`, in which case
the assets `AssetMetadata` can be updated.

#### General Notes

* The `AssetMetadatas` storage map is used for all types and is quarried by the 
`AssetIds` `enum`.

* The `AssetIds` type is also used for routing regular currency functions (i.e. 
transfer, withdraw, deposit).

* For non-erc20 assets, Acala uses 'orml-tokens' for currency functions

#### Data Structures & Types

Note: `MultiLocation` is supplied by XCM.

```rust
pub type ForeignAssetId = u16;
```

```rust
pub struct AssetMetadata<Balance> {
  pub name: Vec<u8>,
	pub symbol: Vec<u8>,
	pub decimals: u8,
	pub minimal_balance: Balance,
}
```

```rust
pub enum AssetIds {
	Erc20(EvmAddress),
	StableAssetId(StableAssetPoolId),
	ForeignAssetId(ForeignAssetId),
	NativeAssetId(CurrencyId),
}
```

```rust
pub enum CurrencyId {
	Token(TokenSymbol),
	DexShare(DexShare, DexShare),
	Erc20(EvmAddress),
	StableAssetPoolToken(StableAssetPoolId),
	LiquidCrowdloan(Lease),
	ForeignAsset(ForeignAssetId),
}
```

## Requirements

If we are to deprecate Currency Factory, we will need to ensure the requirements
it fulfills are still met by some solution.

* Permissioned asset creation can be done without collision in the asset ID

* Asset metadata is available

Additionally, Currency Factory failed to meet some requirements that we should
enforce in a future solution.

* Permissionless asset creation can be done without collision in the asset ID

## Solution Proposals

There are multiple ways we could go about replacing Currency Factory. Possible 
solutions are not necessarily mutually exclusive (i.e. we could implement multiple). 
These solutions are detailed below:

### Simplify & Move the Asset ID Reservation System

One way to remove the need for pallet-currency-factory is to move its 
functionality to pallet-asset-registry. If we also simplify the asset ID 
reservation system to no longer use predefined ranges, this will be a minimal change.

This retains the functionality of Currency Factory, and does not require a new pallet. However, this does not enable permissionless asset creation without the
need for more design.

#### Technical Implementation

Instead of reserving asset IDs via our current range system, we could simply 
have one nonce for reserving new asset IDs, which would (by the definition of a nonce) ensure no collisions. 
This nonce can start at an arbitrary but high number so that our hard-coded 
asset IDs are still safe. This would reduce the complexity of our current 
reservation system while still avoiding collision.

#### Consequences

* If we already have assets in various ranges, the runtime migration process
will become more complicated as we translate them to existing in one range.
  
### Remove Asset ID Reservation

> We do not need [asset ID reservation]. We just runtime config assets to be 
permissioned. When we will have permisionless assets they may already have 
reservation (any way making fork with reservation is easy later).

Asset reservation helps us automate new asset IDs. However, for the scope of 
release two - this may not be necessary. We could instead hard-code asset IDs 
for LP tokens and other instances.

Given we can currently manage without automated asset creation, as we can forgo
the asset creation requirements for release two and ensure that all asset data
is still made available.

#### Consequences

* Minor runtime migration will be needed for deleting Currency Factory's 
storage.

### Use an Externally Maintained Asset ID Reservation System

Neither option presented seems to handle the reserving of asset IDs. Therefore, 
neither of these will fully replace currency factory and would more correctly 
replace our own pallet-assets. A replacement for pallet-assets may be a good 
idea but would require much more substantial changes.

* **Parity's pallet-assets**
  
  Forking this pallet and maintaining extra features we need (hold) is an 
  option.
  
  > It is not clear from RFC why substrate assets are good. IMHO it should be 
  clarified - year of man work to make UI/QA/FE/users happy is what we lack. 
  IMHO better to have fork of assets and collaborate with ecosystem to tune 
  wallets. Do not write own wallets.
  
  #### Pros
  
  * Very nice UI compatibility with Polkadot Dashboard

  * Maintained by Parity

  * Simple Interface

  * Can run two pallets at same time for two purposes (external and mintable) 
  (Demonstrated by Moonbeam)

  * Handles WASM contracts addresses as IDs

  * Permissionless when needed

  * Supported by Polkadot wallets
  
  * Supports asset metadata
  
  #### Cons
  
  * No asset ID reservation system, only prevents duplicates

  * Hold functionality is not present (We depend on this in several cases)

* **Moonbeam's pallet-asset-manager**

Note: Not looking at using or forking this directly, but instead how they use 
different instances of the same pallet for mintable and external assets.

  #### Pros
  
  * Maintained by moonbeam
  
  #### Cons
  
  * Partially specialized

  * No asset ID reservation system, only prevents duplicates

#### Consequences

* Any of these options will require a difficult runtime migration as we move 
existing storage for pallet-assets/pallet-asset-registry/pallet-currency-factory 
to an alternative.

## Conclusion

After analyzing possible solutions, the best course of action to deprecate 
Currency Factory is to maintain a fork of Parity's pallet-assets, embrace the 
design Moonbeam uses to maintain two instances of their pallet, and to remove 
our asset ID reservation system (for release two).

This can be accomplished in two main phases:

1. Create a fork of Parity's pallet-assets and add the features we need

2. Replace our pallet-assets and pallet-currency-factory with two instances of
Parity's pallet-assets.

Parts of either of these phases can be scaled down for release two.

### Fork of Parity's pallet-assets

1. Create a non-traditional fork of Parity's pallet-assets

2. Implement `frame_support::traits::tokens::fungibles::MutateHold` for 
pallet-assets

3. Translate calls into our old pallet-assets and pallet-currency-factory to 
our new pallet-assets

### Manage Two Instances of pallet-assets

1. Configure instance for mintable assets

2. Configure instance for external assets

3. Route calls between the two instances

## Questions

* > As for asset metadata, we don't have standards for formatting this nor do we 
  utilize it.

  Is this true? A brief investigation revealed that outside of `scripts/polkadot-launch/initialization/src/interfaces/basilisk/definitions.ts` 
  there is no mention of the Asset Metadata structure.
  
# References

* [Slack thread detailing issues with Currency Factory Ranges](https://composablefinance.slack.com/archives/C031G5NT0CA/p1667492928188269)

* [Parity's pallet-assets](https://github.com/paritytech/substrate/tree/master/frame/assets)

* [Acala's Asset Registry](https://github.com/AcalaNetwork/Acala/tree/master/modules/asset-registry)

* [Moonbeam's asset-manager](https://github.com/PureStake/moonbeam/tree/master/pallets/asset-manager)
