# Overview

This pallet creates new sovereign (local consensus) currencies (assets) from thin air.

Each currency falls into one of the following categories:

- new currency issued by governance
- currency produced as consequences of a protocol execution that depends on one or more base currencies, usually named LP
- currency mapping the remote location of an asset to local identifiers associated with the foreign currency.


Adding well-known identifiers for assets occurs under the following conditions:

- baked into the runtime during compilation
- added as part of genesis chain specification
- created in runtime

Compile-time currencies are not registered in this pallet.
  
## Basics

In order to be used, We must be able to uniquely identify currencies. This pallet allows the production of new identifiers (ids) to fill that requirement.

A Currency needs an associated amount stored on accounts. To prevent spam, accounts are required to pay an ED (Existential Deposit). As such, each currency must have an [ED](../../docs/0002-rent-deposit.md) defined.

The ED could be zero, but in this case, the asset is locked into a protocol, and users cannot freely transfer.
In this case, users cannot freely create new accounts for this asset.
Such "assets" may account for funds without issuing new currency via this pallet.

All local currencies are normalized to 12 decimals.

## Metadata

In some cases, governance may add metadata to make a currency recognizable, such as:

- name
- `symbol`: A currency may have a human-readable symbol. For example, `XBTC`. This metadata is target for [governance](./governance.md) to prevent spam and fishing.

## Foreign integration

[AssetsRegistry](../assets-registry/README.md) uses this pallet to integrate other decimals and out-of-consensus locations.

Each foreign currency **must** have an entry in AssetsRegistry too.

## Approved governance currencies

Allows governance to approve or revoke currencies to participate in democracy.

## Ranges

This pallet provides the ability to split identifiers into some nominal numeric ranges to help identify assets easier and prevent conflicts.
