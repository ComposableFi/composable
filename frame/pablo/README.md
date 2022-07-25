# Pablo
Pallet Pablo provides extensive functionality to set up an exchange; enabling users to create, trade with, and manage, liquidity pools.


## Overview
Pablo builds on four pillars, previously distinguished pallets in their own right: 

- [Curve Finance](https://curve.fi/files/stableswap-paper.pdf) based functionality for cross market transactions involving stablecoins.

- [Balancer AMM](https://balancer.fi/whitepaper.pdf) built on the constant product formula it functions similar to Uniswap but applies adjustable weights to set a price difference at pool initialization.

- Liquidity Bootstrapping; is based on Balancer AMM and with a set start ending period. The idea is to use the AMM to launch a new token which after it's initial liquidity has been provided by its creator it becomes crowdfunded.

- DEX Router; provides trading infrastructure for currency pairs with no direct pool in Pablo by combining different pools 

This combination of pallets serves as a foundation making it possible for users to create liquidity pools for any given pair of assets not native to pablo.
> Please note: As part of Pablo's development the pallets Liquidity Bootstrapping and DEX Router have been consumed by the Pablo pallet.


## Workflow
We start by calling the `create` function to create an initial pool. Once liquidity is provided to the pool we can start calling basic transactional functions.
Our transactional functions consist of `buy`, `sell`, `exchange` and always reference at least one currency pair.

And basic liquidity pool management functionalities to `Provide liquidity` and `remove liquidity`.

Users can also conduct specific swap operations by composing instructions based on a quote asset in native currency and one or more given foreign assets.


## Use Cases
- Constant AMM exchange
- Delegated liquidity pools
- Liquidity pool index