

https://composablefinance.atlassian.net/wiki/spaces/COM/pages/2916374/Lending

## What

Lending = Loans + Liquidation

Lending calls Oracle and Vault

Oracle = only Pairs with Prices are allowed

Vault = Vault provides and recalls liquidity into/from Lending via protocol.

Lending uses Vault for operation to withdraw assets and deposit back.

Liquidation uses DEX for swaps.

DEX depends on Lending for leverage


## Markets

Can create isolated pairs which are suported  Vault and Oracle.


## Test

There is known way to setup oracle and vault via extrinsic, and there is way to mint proper amounts to test users, then can deposit/borrow/repay via lending according rules via extrinsics.

## Lending Vault


Withdraws on market creation, if cannot withdraw, market creation fails.

### Block level

Lending is executed after Vault.

On block initialization, if can with draw, than withdraws.

On block finalization, if can deposit back, deposits. If must liquidate, than deposit back all.

### On borrow

If must liquidate or not enough funds, borrow fails.

### On repay

Repay, do nothing to vault.
