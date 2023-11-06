# Overview

General operation allows user to stake relay `staking token`s and get `derivative token`.
Derivate token later used for unstaking. 

Requires general understanding of relay staking https://wiki.polkadot.network/docs/learn-staking-index

## From user perspective

When user stakes, next happens:
- staking token, after some fees deduction, gets into accounting of this pallet
- user is minted, after ration conversion, with derivative token


When user unstakes,
- his derived token is burn
- his unstake request recorded


Later on, user claims his staked token.

If unstake did not happened yet, user can cancel his request if he used fast unstake.
Fast unstake matches user who want to stake with users who want to unstake inside pallet.

## From operation perspective

Era and block of relay are delivered to this pallet in trustless way.

Fast unstaking requests (in this era) reduce ratio of staking to derived token, deincetivicing to unstake.