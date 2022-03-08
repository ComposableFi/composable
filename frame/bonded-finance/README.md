# Bonded Finance

A pallet providing means of submitting and maintaining bond offers.

## Overview

The bonded finance pallet enables users to create bond offers, buy bonds from 
other users, and cancel existing bond offers via admin intervention. Theses 
bonds can be listed with varoius assets and can offer a different asset as the 
reward.

One bound offer may contain multiple identically priced bounds. Each bound will 
offer an equal part of the reward to buyers. Buyers can opt to buy multiple 
bounds to increase their share of the reward. Depending on the offer, buyers 
may or may not get there initial investment on the bond back. If there is a 
reward for the bond, buyers will always get their share of the reward.

## Sample Use Case

Alice creates a new bound offer with some number of bounds each priced at the 
same asset value. At the same time she provides reward assets which will be 
vested into the accounts which take the bond offers. She then locks some native 
currency to register the offer.

Bob buys part of the bounds from Alice's offer by transfering some asset amount 
desired by Alice. Bob will be vested the reward amount after the reward maturity 
period. If the offer maturity period is infinite, Bob will not be vested his 
initial invested amount.

Alice may cancel the offer and prevent new bonds on the offer. Once canceled she 
gets her native tokens back. All existing maturity periods continue to be
executed.

## Bond Offer Workflow

### Creating Offers

The workflow of a bond offer starts with the `offer` extrinsic. Once an offer 
has been made, other users can decide to buy bonds from the offer with the 
`bond` extrinsic.

An offer defines some critical information:

* The number of bonds

* The price per bond 

* The maturity period (This can be finite or infinite)

* The reward

If the offer maturity period is finite, the liquidity of the bond(s) will be 
returned to the buyer at the end of the maturity period. Otherwise, the 
beneficiary will own the liquidity. The reward is defined with its own maturity 
period. The reward maturity period can only be finite. Both maturity periods 
are measured from when the bond(s) are bought.

The reward is distributed proportionally to buyers based on the number of bonds 
they own.

### Buying Bonds

Bonds can be purchased with the `bond` extrinsic. Buyers will indicate the 
number of bonds they wish to buy. If the number number of bonds they wish to 
buy is higher than the number of available bonds in the contract, the 
transaction will not go through.

Buying bonds will start the offer and reward maturity periods at the current 
block.

Once all bounds are purchased, the stake paid by the offer creator will be 
refunded.

### Canceling Offers

Bond offers can be canceled with the `cancel` extrinsic. This can only be 
successfully called by the `AdminOrigin`.

Once canceled, the stake and liquidity will be returned to the offer creator. 
However, this will not cancel currently vested rewards.

## Technical Notes

* This pallet implements the `composable_traits::bonded_finance::BondedFinance`.

