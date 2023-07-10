# Overview

Allows to map remote assets to local and back(bidirectional). Mapping can be created only by privileged origin.

Used for cross chain message transfers and payments.

## Basics

Each remote asset must have a local identifier. This pallet uses [CurrencyFactory](../currency-factory/README.md) internally for that purpose.

## Assets' identifiers

Well known tokens, like relay native, are baked into codebase directly.

For remote location, canonical (shortest) representation should be used in case multiple locations are possible. 

## Governance

Remote assets can be added only by governance. Also assets may be locked to chain. No XCMP operation involving these will be possible.

Remote asset can be approved by other chain origin. Such assets can be Teleported to and from relevant chain.
