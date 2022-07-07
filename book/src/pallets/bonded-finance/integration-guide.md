# [name] Integration Guide

[Pallet Overview & Workflow](../bonded-finance.md)

## Integration Status

| Dali | Picasso | Composable |
| ---- | ------- | ---------- |
| ?    | ?       | ?          |

## Setup / Configuration
Offers are created and managed by users and can be canceled via admin intervention. The creator of an offer has access to configuration methods. An Offer has 3 states: created, enabled, disabled. These transition linearly.

During the created state all configuration operations can be conducted. During the enabled state, all transactional operations can be conducted. During the disabled state the offer account and its relevant information in pallet storage will be deleted.

Automatic state transitions can occur under two conditions. 1) The number of bonds offered has been bought and reward maturity has been reached 2) The offer was canceled by admin intervention.
## RPC & Data Retrieval

*RPCs w/ links to cargo docs?*

## Subsquid Data Retrieval

N/A

## Locally Consumed Types

### Types
* `NativeCurrency` - Numeric type used to represent the currency required to create an offer
* `Currency` - Numeric type used to represent the currency offers are based on
* `Vesting` - Function for managing vesting transfer of rewards
* `BondOfferId` - Numeric type used to uniquely identify bond offers
* `Convert` - Function for converting balances required for reward computation
  
### Constants

* `PalletId` - Unique ID of the pallet
* `Stake` - The amount required to create an offer
* `MinReward` - The minimum reward for an offer
* `AdminOrigin` - The origin that is allowed to cancel bond offers
* `WeightInfo` - Weights
  

## Calculations & Sources of Values

N/A

## Extrinsic Parameter Sources

See the Bonded Finance pallet extrinsics [documentation](./extrinsics.md). 

## Pricing Sources

N/A