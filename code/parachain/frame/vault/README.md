# Vaults Pallet

A batteries included vault module, usable as liquidity pools, yield farming 
vaults or embeddable as core infrastructure.

## Overview

The Vault module provides functionality for asset pool management of fungible 
asset classes with a fixed supply, including:

* Vault Creation.
* Deposits and Withdrawals.
* Strategy Re-balancing.
* Surcharge Claims and Rent.

To use it in your runtime, you need to implement the vault's [`Config`](crate::Config).

## Concepts

* Strategy: Accounts, smart contracts or pallets which may access funds in the 
  vault for investment purposes. Each vault may have up to [MaxStrategies](Config::MaxStrategies) 
  associated with it. Each strategy is trusted, and has access to the complete 
  contents of the vault, although the vault does recommend how much it should 
  withdraw, based on it's allocations.

* [CreationDeposit](Config::CreationDeposit): The minimum deposit needed by a 
  user to create a vault. The deposit is also the reward for reaping the vault.

* [ExistentialDeposit](Config::ExistentialDeposit): Vaults created with at least 
  existential deposit are never reaped in V1. Mainly used for common good 
  vaults.

* Reaping: Each block, regular vaults pay rent for existing. Once the rent runs 
  out, vaults are marked for deletion (tombstoned), and reaped after 
  [TombstoneDuration](Config::TombstoneDuration) blocks. 

## Workflow

Vaults are initially created with the `create` extrinsic. It's here that the 
lifetime of the vault is determined. Vaults that are created with an initial 
deposit greater than `ExistentialDeposit + CreationDeposit` will remain alive 
forever. Otherwise, vaults will be `tombstoned` after the `RentPerBlock` has 
depleted the funds in the vault.

While alive, vaults can be deposited to and have surcharge claimed. When 
`claim_surcharge` is called on a vault, its existing rent is paid and the caller 
is rewarded a small fee in turn. The `withdraw` extrinsic can always be called 
on vaults as long as they have not been deleted.

To avoid becoming `tombstoned`, users can deposit more funds to afford the 
`RentPerBlock`. If a vault does become `tombstoned`, it can be revived with the 
`add_surcharge` extrinsic before it is deleted. Once a vault has been 
`tombstoned`, it can be deleted with the `delete_tombstoned` extrinsic. Once 
deleted, the remaining balance of the vault will be returned. 

## Reusing the Vault

Pallets depending on the vault should use the [vault](composable-traits::vault) 
traits. When managing the reaping and deposits is too difficult due to the 
creation of many vaults, or prohibitively expensive; create the vault with an 
existential deposit. You should ensure that you delete the vault yourself once 
it is no longer required.

## Emergency Shutdown

Root is capable of completely shutting down a vault, disallowing deposits and 
withdrawals. This is intended as a mitigation for hacks. After an emergency 
shutdown, a vault can be restarted by root.
