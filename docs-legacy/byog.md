# Overview

Use cases of Bring your own gas (`BYOG`) on the Picasso parachain.

BYOG allows owners of non-native tokens to execute transactions by paying fees in foreign currencies.

Such mechanisms are built into the runtime on Picasso and [other Dotsama parachains](https://wiki.acala.network/learn/flexible-fees).

`On ramps` and [meta (gas less) transactions](https://docs.polygon.technology/docs/category/meta-transactions) handle such possibilities on other chains.

## Context

Users pay fees in the runtime's native currency for the execution, allowing the runtime to earn and prevents spam and DDoS attacks. BYOG extend payments to registered non native currencies.

A runtime configuration allows mapping amount of execution which can be bought for amount of native currency.

## Current state

Any cross-chain XCMP transactions can choose which assets to use to pay for the transaction.

Direct transaction can add header to pick up `payment` asset or accounts can configure default [payment asset](https://github.com/paritytech/substrate/discussions/12055).

`Payment` assets are configurable and configuration is shared.

In future, direct Pablo-based swaps from an asset to PICA will be available later.

Minting, burns, and airdrops are out of the scope of this feature.

Fees are small now, but later we will have a way to show the exact fee for a transaction paid in any possible asset.

**Example**
Currently, non-native currencies are mapped to native with some configurable ratio which in turn is mapped to a weight.

## How much does my transaction cost in Picasso?

When a transaction is executed, its `weight` is known and roughly equals the computational resources it consumes. Computational resources are benchmarked and updated with each runtime upgrade.

Weight is converted into an appropriate amount of PICA by the polynomial formula.

The formula dynamically changes depending on the desired target load of the network.
As the usage of this chain increases towards maximum capacity, the price of a unit of weight increases as well.

From this point onwards we say that transactions are paid directly in PICA for simplicity.

For details on fees, see the chapters below.

## How can one pay for a transaction when one has USD/KSM?

If a user account has enough PICA to pay for direct transactions and the user-specified PICA as payment assets in the case of XCMP,
the appropriate amount of PICA would be moved to the `native treasury.`

If a user has not enough PICA to pay and [keep the account alive](../rfcs/0002-rent-deposit.md) or a user specified USD/KSM as payment in case of XCMP,
an appropriate amount of foreign assets will be calculated and transferred to the treasury.

The amount would be defined by the configured or hardcoded ratio of PICA to USD/KSM.
If there is no ratio defined, a transaction will fail.

Later, Pablo governed and native governance approved DEX will be checked for direct swaps of PICA to USD/KSM.

In this case:

- The native treasury will get PICA.
- Pablo will get foreign assets.
- If there is no direct and approved mapping, configured or hardcoded ratios will be used as before.
- if slippage is unacceptable or the pool is empty the transaction will fail. It will not try to use configured or hardcoded ratio.

The currency in which users can pay on Picasso is `payment currency`.

## Direct native transaction specific

Later user will be able to configure his preferred payment token by default. If the user peeks at such a token, then direct transactions are paid only in this token. No other tokens are tried.

Switching configuration is paid in a token to which configuration is switched. The user's existential deposit(ED) must be enough to reset the configuration to default later.

## XCMP specific

If a user tried to pay in a currency that is not `payment currency`, it would be trapped in a binary blob.

If the user overpaid for a transaction, the remaining funds go to the user's account on Picasso. The whole amount goes to the user account if the payment is too small.

## IBC Specific

Packet trying to pay in no payment currency will not be acknowledged.

## Fee calculator

Here is an example of the fee for transferring some KSM from Karura:

1. Fee on Karura for XCMP.
2. Price for XCMP on Picasso. It consists of a base fee and some formula of cost per instruction.
3. Transaction fee on Picasso
4. DEX swap fee

All these fees change dynamically depending on network load, upgraded when the runtime is upgraded (and dependencies of runtime), and pool or native fee formulas configuration changes.

There is no unified view of all fees. So if there will be, given fees prediction should be less than one order of magnitude error.

## References

### Pallets

- [transaction-fee](../frame/transaction-fee)
- xcmp
- [pablo](../frame/pablo)
- [assets-registry](../frame/assets-registry)
- [currency-factory](../frame/currency-factory)
- <https://github.com/AcalaNetwork/Acala/blob/master/modules/transaction-payment/src/lib.rs>
- <https://github.com/paritytech/substrate/blob/master/frame/transaction-payment/asset-tx-payment/src/lib.rs>
