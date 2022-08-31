# Bonded Finance

A pallet providing means of submitting and maintaining bond offers.

## Overview

The bonded finance pallet enables users to create bond offers, buy bonds from 
other users, and cancel existing bond offers via admin intervention. These 
bonds can be listed with various assets and can offer a different asset as the 
reward.

One bond offer may contain multiple identically priced bonds. Each bond will 
offer an equal part of the reward to buyers. Buyers can opt to buy multiple 
bonds to increase their share of the reward. Depending on the offer, buyers 
may or may not get their initial investment on the bond back. If there is a 
reward for the bond, buyers will always get their share of the reward.

## Use cases

- Staking. User locks amount for some period of time, gets reward in the end or vested. Stake
  returns to user in the end.
- Vesting. User offers amount and ensure that recipients have 'ticket' tokens to bond to get
  reward.
- Sell/Exchange/Trade. User makes bond offer, other users takes offer in exchange for other
  amount.

## Sample Use Case

Alice creates a new bond offer with some number of bonds each priced at the 
same asset value. At the same time she provides reward assets which will be 
vested into the accounts which take the bond offers. She then locks some native 
currency to register the offer.

Bob buys part of the bonds from Alice's offer by transferring some asset amount 
desired by Alice. Bob will be vested the reward amount after the reward maturity 
period. If the offer maturity period is infinite, Bob will not be vested his 
initial invested amount.

Alice may cancel the offer and prevent new bonds on the offer. Once canceled she 
gets her native tokens back. All existing maturity periods continue to be
executed.

## Concerns

Protocol is not protected from sniper bots, whales and other attackers.
Could lock amounts after into time locked fNFTs, vested, or offer to depend on time and already
taken amount.

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
number of bonds they wish to buy. If the number of bonds they wish to 
buy is higher than the number of available bonds in the contract, the 
transaction will not go through.

Buying bonds will start the offer and reward maturity periods at the current 
block.

Once all bonds are purchased, the stake paid by the offer creator will be 
refunded.

### Canceling Offers

Bond offers can be canceled with the `cancel` extrinsic. This can only be 
successfully called by the `AdminOrigin`.

Once canceled, the stake and liquidity will be returned to the offer creator. 
However, this will not cancel currently vested rewards.

## Technical Notes

* This pallet implements the `composable_traits::bonded_finance::BondedFinance` trait.
