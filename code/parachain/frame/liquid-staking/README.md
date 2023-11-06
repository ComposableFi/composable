# Overview

General operation allows user to stake `relay token`s and get `derivative token`.
Derivate token later used for unstaking. 

## From user perspective

When user stakes, next happens:
- relay token, after some fees deduction, gets into accounting of this pallet
- user is minted, after ration conversion, with derivative token


When user unstakes,
- his derived token is burn
- his unstake request recorded


Later on, user claims his staked token.

If unstake did not happened yet, user can cancel his request. 

## From operation perspective