# Overview

Currency pallet allows to create new sovereign(local consensus) currencies(assets) from thin air.

Each currency falls into one of next categories:

- new currency issued by governance
- currency produced as consequences of execution of protocol and depends on some base currency(ies), usually named LP
- currency mapping remote location of asset to local identifiers, foreign currency
  
## Basics

Currency must be identified, so factory allows to produce new identifiers.

Currency to be useful must have amounts. Amounts must be stored on accounts. Accounts must pay ED to prevent spam.
So each currency must have ED defined.

In rare edge cases, ED could be zero, but in this case amount is locked into protocol and users cannot freely transfer amounts. In some cases such amounts could be stored in storage without issuing new currency.

All local currencies are normalized to 12 decimals.

## Metadata

In some cases governance may add metadata to make currency recognizable, such as:

- name
- symbol

## Foreign integration

[AssetsRegistry](../assets-registry/README.md) use this pallet to make integrating other decimals and out of consensus locations. 

Each foreign currency MUST have entry in AssetsRegistry too.

## Well known currencies

Are optimized and baked into codebase directly.

## Approved governance currencies

Allows governance to approve or revoke currencies to participate in democracy.
