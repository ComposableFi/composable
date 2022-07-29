<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-06-25T22:31:58.471583366Z -->

# Vault Pallet Extrinsics

## Create

[`create`](https://dali.devnets.composablefinance.ninja/doc/pallet_vault/pallet/enum.Call.html#variant.create)

Creates a new vault, locking up the deposit. If the deposit is greater than the
`ExistentialDeposit` + `CreationDeposit`, the vault will remain alive forever, else it
can be `tombstoned` after `deposit / RentPerBlock `. Accounts may deposit more funds to
keep the vault alive.

### Emits

* [`Event::VaultCreated`](Event::VaultCreated)

### Errors

* When the origin is not signed.
* When `deposit < CreationDeposit`.
* Origin has insufficient funds to lock the deposit.

## Claim Surcharge

[`claim_surcharge`](https://dali.devnets.composablefinance.ninja/doc/pallet_vault/pallet/enum.Call.html#variant.claim_surcharge)

Substracts rent from a vault, rewarding the caller if successful with a small fee and
possibly tombstoning the vault.

A tombstoned vault still allows for withdrawals but blocks deposits, and requests all
strategies to return their funds.

## Add Surcharge

[`add_surcharge`](https://dali.devnets.composablefinance.ninja/doc/pallet_vault/pallet/enum.Call.html#variant.add_surcharge)

No documentation available at this time.

## Delete Tombstoned

[`delete_tombstoned`](https://dali.devnets.composablefinance.ninja/doc/pallet_vault/pallet/enum.Call.html#variant.delete_tombstoned)

No documentation available at this time.

## Deposit

[`deposit`](https://dali.devnets.composablefinance.ninja/doc/pallet_vault/pallet/enum.Call.html#variant.deposit)

Deposit funds in the vault and receive LP tokens in return.

### Emits

* Event::Deposited

### Errors

* When the origin is not signed.
* When `deposit < MinimumDeposit`.

## Withdraw

[`withdraw`](https://dali.devnets.composablefinance.ninja/doc/pallet_vault/pallet/enum.Call.html#variant.withdraw)

Withdraw funds

### Emits

* Event::Withdrawn

### Errors

* When the origin is not signed.
* When `lp_amount < MinimumWithdrawal`.
* When the vault has insufficient amounts reserved.

## Emergency Shutdown

[`emergency_shutdown`](https://dali.devnets.composablefinance.ninja/doc/pallet_vault/pallet/enum.Call.html#variant.emergency_shutdown)

Stops a vault. To be used in case of severe protocol flaws.

### Emits

* Event::EmergencyShutdown

### Errors

* When the origin is not root.
* When `vault` does not exist.

## Start

[`start`](https://dali.devnets.composablefinance.ninja/doc/pallet_vault/pallet/enum.Call.html#variant.start)

(Re)starts a vault after emergency shutdown.

### Emits

* Event::VaultStarted

### Errors

* When the origin is not root.
* When `vault` does not exist.

## Liquidate Strategy

[`liquidate_strategy`](https://dali.devnets.composablefinance.ninja/doc/pallet_vault/pallet/enum.Call.html#variant.liquidate_strategy)

Turns an existent strategy account `strategy_account` of a vault determined by
`vault_idx` into a liquidation state where withdrawn funds should be returned as soon
as possible.

Only the vault's manager will be able to call this method.

### Emits

* Event::LiquidateStrategy
