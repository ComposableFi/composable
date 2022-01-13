# Vaults Pallet

A batteries included vault module, usable as liquidity pools, yield farming vaults or embeddable
as core infrastructure.

## Overview

The Vault module provides functionality for asset pool management of fungible asset classes
with a fixed supply, including:

* Vault Creation.
* Deposits and Withdrawals.
* Strategy Re-balancing.
* Surcharge Claims and Rent.

To use it in your runtime, you need to implement the vault's [`Config`](crate::Config).

## Concepts

* Strategy: Accounts, smart contracts or pallets which may access funds in the vault for investment purposes. Each vault may have up to [MaxStrategies](Config::MaxStrategies) associated with it. Each strategy is trusted, and has access to the complete contents of the vault, although the vault does recommend how much it should withdraw, based on it's allocations.

* [CreationDeposit](Config::CreationDeposit): The minimum deposit needed by a user to create a vault. The deposit is also the reward for reaping the vault.

* [ExistentialDeposit](Config::ExistentialDeposit): Vaults created with at least existential deposit are never reaped in V1. Mainly used for common good vaults.

* Reaping: Each block, regular vaults pay rent for existing. Once the rent runs out, vaults are marked for deletion (tombstoned), and reaped after [TombstoneDuration](Config::TombstoneDuration) blocks. 

## Reusing the Vault

Pallets depending on the vault should use the [vault](composable-traits::vault) traits. When managing the reaping and deposits is too difficult due to the creation of many vaults, or prohibitively expensive; create the vault with an existential deposit. You should ensure that you delete the vault yourself once it is no longer required.

## Emergency Shutdown

Root is capable of completely shutting down a vault, disallowing deposits and withdrawals. This is intended as a mitigation for hacks.
