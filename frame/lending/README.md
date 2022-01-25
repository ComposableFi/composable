

# [Overview](https://app.clickup.com/20465559/v/dc/kghwq-20761/kghwq-3621)

## What

Lending = Loans + Liquidation

Lending calls Oracle and Vault

Oracle = only Pairs with Prices are allowed

Vault = Vault provides and recalls liquidity into/from Lending via protocol.

Lending uses Vault for operation to withdraw assets and deposit back.

Liquidation uses DEX for swaps.

DEX depends on Lending for leverage

## Markets

Can create isolated pairs which are supported Vault and Oracle.

## Test

There is known way to setup oracle and vault via extrinsic, and there is way to mint proper amounts to test users, then can deposit/borrow/repay via lending according rules via extrinsics.

## Lending Vault

As of now Lending does not handles cases when vault changes its decisions during single block.

### Block level

Lending is executed after Vault.

On block initialization. If can withdraw, than withdraws. If can deposit back, deposits maximal part available. If must liquidate, than deposit back all.

### On borrow

If must liquidate or not enough funds(we got maximal amount during initialization), borrow fails.

### On repay

Repay, do nothing to vault. It actually allows to use some deposit asset by other transactions in same block to borrow (event if we should deposit back into vault).
