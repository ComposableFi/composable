# Bonded Finance

*A pallet providing means of submitting and maintaining bond offers.*

---

## Overview

The bonded finance pallet enables users to create bond offers, buy bonds from 
other users, and cancel existing bond offers via admin intervention. These bonds 
can be listed with various assets and can offer a different asset as the reward.

One bound offer may contain multiple identically priced bounds. Each bound will 
offer an equal part of the reward to buyers. Buyers can opt to buy multiple 
bounds to increase their share of the reward. Depending on the offer, buyers may 
or may not get their initial investment on the bond back. If there is a reward 
for the bond, buyers will always get their share of the reward.

## Bond Workflow

### Creating Offers 

The workflow of a bond offer starts with the [`offer`](#offer) extrinsic. Once 
an offer has been made, other users can decide to buy bonds from the offer with 
the [`bond`](#bond) extrinsic.

An offer defines some critical information:

* The number of bounds

* The price per bond

* The maturity period (This can be finite or infinite)

* The reward

If the offer maturity period is finite, the liquidity of the bond(s) will be 
returned to the buyer at the end of the maturity period. Otherwise, the 
beneficiary will own the liquidity. The reward is defined with its own maturity 
period. The reward maturity period can only be finite. Both maturity periods are 
measured from when the bond(s) are bought.

The reward is distributed proportionally to buyers based on the number of bonds 
they own.

### Buying Bonds

Bonds can be purchased with the [`bond`](#bond) extrinsic. Buyers will indicate 
the number of bonds they wish to buy. If the number number of bonds they wish to 
buy is higher than the number of available bonds in the contract, the 
transaction will not go through.

Buying bonds will start the offer and reward maturity periods at the current 
block.

Once all bounds are purchased, the stake paid by the offer creator will be 
refunded.

### Canceling Offers

Bond offers can be canceled with the [`cancel`](#cancel) extrinsic. This can 
only be successfully called by the [`AdminOrigin`](#adminorigin).

Once canceled, the stake and liquidity will be returned to the offer creator. 
However, this will not cancel currently vested rewards.

## Pallet Extrinsics

### Offer

`offer`

Create a new offer to be [`bond`](#bond) to later.

The reward must be greater than or equal to [`MinReward`](#minreward).

The origin for this call must be signed and the sender must have the appropriate 
funds.

The sender can request that their account be kept alive using the `keep_alive` 
parameter of this extrinsic.

### Bond

`bond`

Buy bonds from a bond offer.

The sender should provide the number of contracts they are willing to buy. Once 
an offer is complete, the [`Stake`](#stake) put by the sender is refunded.

The origin for this call must be signed and the sender must have the appropriate 
funds.

The sender can request that their account be kept alive using the `keep_alive` 
parameter of this extrinsic.

### Cancel

`cancel`

Cancel a running offer.

This will block more bonds to the offer, but not the currently vested reward. 
The Stake put is refunded.

The origin for this call must be signed by an [`AdminOrigin`](#adminorigin).

## Pallet Configuration

### Event

### NativeCurrency

The native currency used for the stake that is required to create an offer.

### Currency

The currency system offers are based on.

### Vesting

The dependency managing vesting transfer of rewards.

### BoundOfferId

The ID of a bond offer.

### Convert

The dependency managing conversions of a balance to the unit required for reward 
computation.

### PalletId

The Pallet ID. 

Required to create sub-accounts used by offers.

### Stake

The liquidity a user has to put to create an offer.

### MinReward

The minimum reward value for offers to be listed.

### AdminOrigin

The origin that is allowed to [`cancel`](#cancel) bond offers.

### WeightInfo

The implementation of extrinsic weights.
