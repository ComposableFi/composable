# Overview

Allows to map remote assets to local and back(bidirectional). Mapping can be created only by privileged origin.

Used for cross chain message transfers and payments.

## Basics

Each remote asset must have a local identifier. This pallet uses [CurrencyFactory](../currency-factory/README.md) internally for that purpose.


## Decimals

Remote assets may have different decimals than local ones, so remote assets may need to be configured to have proper decimals. As an example, remotely BTC has eight decimals, while locally we use 12.

This mapping can be used by out-of-consensus protocols, such as oracles and bridges.

When a transfer happens, we should know what the given `Amount` transferred means for that currency on our local network. We also need to know the minimal amount and number of decimals the currency has remotely.

Mishandling may lead to precision loss and loss/gain of currency; in the worst case, a round-trip transfer will "print money".


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
