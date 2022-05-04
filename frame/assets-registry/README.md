# Overview

Allows to map remote assets to local and back(bidirectional). Mapping can be created only by privileged origin.

Used for cross chain message transfers and payments.

## Basics

Each remote asset must have local identifier. This pallet calls [CurrencyFactory](../currecy-factory/README.md) to get that done.

Remote assets may have different than local decimals, so remote asset may be configured to have proper decimals.

## Weights and fees

If assets can be used to pay for execution of messages, it can be set with:

- Minimal fee in asset amount on target chain. So messages which will pay less than this fee will not be sent
- Allowing to pay fee for execution on this chain, by mapping asset amount to native. In case approved [DEX route](../dex-router/README.md) has appropriate pool, it used to override registry value.

## Assets' identifiers

Well known tokens, like relay native, are baked into codebase directly.

For remote location, canonical (shortest) representation should be used in case multiple locations are possible. 

## Governance

Remote assets can be added only by governance. Also assets may be locked to chain. No XCMP operation involving these will be possible.

Remote asset can be approved by other chain origin. Such assets can be Teleported to and from relevant chain.
