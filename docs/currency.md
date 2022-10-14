---
tags:
  - assets
  - tokens
  - funds
lastmod: 2022-08-23T15:33:24.815Z
---

# Overview

This document describes various aspects of currency as they relate to composable finance. All these aspects are, in some form, embedded into our protocols.

In this document, we use the following terms interchangeably:

- `currency`, `asset`, `token`
- `this parachain`, `consensus`

## What is currency?

We express the identity of a currency as a positive integer. Any positive integer may be a currency id, but not all integers are currency ids.

The [CurrencyFactory](../frame/currency-factory) pallet manages all currencies created in the runtime. For clarification on the low-level basics, look into the pallet readme.


### Amounts

Each currency has a non-negative total issuance. Given an identifier, we can ask for the total supply of the respective currency in consensus with the ledger(in this case, the parachain). A Currency is considered fungible because an amount of 10 of said currency equals another 10 of the same currency on another account.

### Decimals

A Currency may be associated with a `unit` and `minimal amount`. `Unit` usually comes in an amount of 10 to some power like 6, 9, or 12. The units are typically priceable and comprehendible for users. `minimal amount` can be used to operate micro-transactions and help minimize rounding errors.  

Most pallets operate without knowledge of decimals and execute all mathematical operations on whole numbers.

Only pallets that care about out-of-consensus metadata operate with decimals, such as the oracle and bridge protocols. These protocols normalize all incoming amounts to 12 decimals based on knowledge of remote decimals.

Ledgers operate only in decimal currencies. However, currencies with zero decimals are viable too.

Pallets use larger numbers internally in order to keep rounding effectively non-existent on a per-protocol basis.

**Example**

Currency A has 12 decimals. 420 units of A would equal to `420*10^12`. Currency B has 9 decimals, so 420 units would be equal to `420*10^09`.
In a decimals-agnostic pallet, amount of A and be B would be equal to `42*10^13`.

## Remote or local

All currencies are eventually local with local identifiers.

Any remote currency bridged to our parachain is minted in an equal amount to what was bridged.
Remote currency transferred into and out of the parachain is minted and burned in equal amounts.

Remote tokens on our chain can be local or remote on other chains. 
For example, ETH is remote on Acala and AUSD is local on Acala. Both are remote on this parachain as we consider them to both be from Acala.

In the local consensus, bridged tokens semantically are just protocol tokens with their associated risks.

For details on registry mapping, check the [AssetsRegistry](../frame/assets-registry) pallet.

### Remote currencies equivalence

Generally, a remote currency transferred via one bridge is not necessarily the same as the same currency transferred via another bridge.

For example, USDC transferred via Mosaic is not equal to the same amount transferred via XCMP from Acala.

There are several solutions to equalize currencies:

- Create a DEX for the currency pair and use a router to swap them. For example, if the currencies are equal in price, the swap would be 1 to 1. Market defined is best for high-risk bridges. 
- A risky approach is to embed direct trust to bridges via the configured ratio of transferred currencies to be 1 to 1.
- Use trustless decentralized bridging. With this solution, there is proof of equivalence, making it the lowest risk. For example, IBC MMR or XCMP `Reserver Transfer`.

For more details, read XCVM documentation regarding security and the risks of choosing bridges (relayers).

**Example**

ETH was transferred from Ethereum to Acala and then to this parachain as AETH. At the same time, METH was transferred from Moonbeam. Moonbeam was then hacked, and the hacker minted a large amount of unbacked METH.
In this the case DEX (market) would react and skew the price of AETH to be higher than METH.
More mild form of non equivalence would be because of network congestion.
Imagine Acala transfer prices are several time more of Moonbeam. In this case  AETH will be less liquid than METH, so it price may be lower.

See [other examples here](./xcvm_denominations_tokens.dot))

### Remote currency identifiers

Any remote currency has a bidirectional map from and to local.
Bidirectional mapping allows sending the number of tokens from and to local consensus.

Remote currencies usually have their respective bridge identifiers attached in the format "kind/network(chain)". 

**Example**

You may find example in XCM and XCVM documentation.

## Currencies and gas fees

See [BYOG](./byog.md) for details.

Cross-chain interactions over bridges may involve metered networks which allow setting limits on execution price and calculating the possible number of execution resources in the runtime. For example, Ethereum.

Within the protocols built on top of the parachain, there are "native" protocol currencies that are used for governance and revenue distribution, these may be used to pay gas fees if configured that way.

### Bridged gas fee (`bring your own gas`)

See [BYOG](./byog.md) for details.

Users who want to have assets on our chain can either be issued native currency by some on-chain protocol or be allowed to pay for cross-chain transactions in some non-native currency.

## Tokenomics

All currencies are eventually local - they must have identity on this chain.
Minting of local representation is governed by a protocol.

The most straightforward protocol is initial minting by `root` or `governance voting`.

Other more complicated protocols are `bridges`, `staking`, or `LP`, and are more complicated.

Protocols form various economical facets of tokens. 
Here are several examples.

### Inflation

In the context of currency, we define inflation as when an amount of an asset is worth less than it was previously, and as such can no longer be swapped for as much of another asset(s) as it could before.

Inflation may occur when:
- Users exchange substantial amount of inflating currency into other currencies.
- An excessive amount of tokens are minted, causing the value of the asset to decrease. 
- A previously locked currency is released to the market.
- A currency is being shorted 
- Off-chain issues like bridging bugs or misleading off-chain oracle information

Some protocols, such as stable coins and staking, try to maintain a specific price level or level of liquidity by continuously minting tokens.

Burning and locking tokens will in general cause deflation, as the total available supply has been decreased.

### Dilution

Some protocols allow users to provide their tokens to the protocol to issue other tokens, representing the total share of the protocol.
Meaning new users coming into the protocol and issuing more tokens leads to a reduction of the total user share in the protocol diluting their tokens.

### Risk categories

Some assets may be considered more volatile/riskier than others.

Categories represent governance and oracle-like information on the chain.

They are used in protocols like lending or credit scores.

**Example**

Assets may be sore risky, so it is not used as collateral in lending.
It may depreciate in price fast and there is no market to sell it when that happens.

See Angular documentation for more examples.

## Relevant pallets and contracts

- xTokens
- tokens
- balances
- unknownTokens
- currencyFactory
- assets
- assetsRegistry
- mosaic