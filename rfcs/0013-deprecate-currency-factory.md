# Depercation of Currency Factory - Assessment RFC

This assessment aims to gather all relevant details and consoquences of 
depercating Currency Factory and its replacments.

## Reasoning

There are several reasons we stand to benifit from replacing Currency Factory.
These reasons are detailed below:

### Currency Factory Ranges are Confusing
  
As detailed by Dzmitry in [this](https://composablefinance.slack.com/archives/C031G5NT0CA/p1667492928188269) 
thread, the Currency Factory ranges are a confusing abstraction that we don't 
stand to gain from. We really only have two types of currencies native 
(mintable) and non-native (external). However, we have many more ranges that 
enforce artificial limitations on various abstractions of these two currency 
types. This complicates the currency creation process and leads to confusion 
while providing no benefit to us.

### Currency Factory is Already Losing Functionality

Recently, we moved away from storing ED in Currency Factory. This leaves two 
uses for currency factory: reserving asset IDs and storing asset metadata. 

As we have already discussed, currency factory over complicates the asset ID 
resevation system. As for asset metadata, we don't have standards for formatting 
this nor do we utilize it.

Assuming we replace the reservation system, Currency Factory will be left 
without value.

## Solution Proposals

There are multiple ways we could go about replacing Currency Factory. These 
solutions are detailed below:

### Simplify & Move the Asset ID Reservation System

One way to remove the neeed for pallet-currency-factory is to move its 
functionality to pallet-asset-registry. If we also simplify the asset ID 
reservation system, this will be a minimal change.

Instead of reserving assset IDs via our current range system, we could simply 
have a nonce for reserving new asset IDs while ensuring a lack of collisions. 
This nonce can start at an arbritary but high number so that our hard-coded 
asset IDs are still safe (i.e `u32::MAX`). This would reduce the complexity of 
our current reservation system while still avoiding collisons.

### Use an Externally Maintained Asset ID Reservation System

Neither option presented seems to handle the reserving of asset IDs. Therefore, 
neither of these will fully replace currency factory and would more correctly 
replace our own pallet-assets. A replacement for pallet-assets may be a good 
idea but would require much more substantial changes A replacement for 
pallet-assets may be a good idea but would require much more substantial 
changes.

* **Parity's pallet-assets**
  
  #### Pros
  
  * Very nice UI compatability with Polkadot Dashboard
  
  * Maintained by Parity
  
  * Simple Interface
  
  #### Cons
  
  * No asset ID reservation system, only prevents duplicates
  
  * Hold functionality is not present (We depend on this in several cases)

* **Moonbeam's pallet-asset-manager**

  #### Pros
  
  * Maintained by moonbeam
  
  #### Cons
  
  * Partially specialized
  
  * No asset ID reservation system, only prevents duplicates

## Questions

* > As for asset metadata, we don't have standards for formatting this nor do we 
  utilize it.

  Is this true? A breif investigation revealed that outside of `scripts/polkadot-launch/initialization/src/interfaces/basilisk/definitions.ts` 
  there is no mention of the Asset Metadata structure.