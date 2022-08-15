# Pablo
Pallet Pablo provides extensive functionality to set up an exchange, enabling users to create, trade with, and manage, liquidity pools.


## Overview
Pablo builds on four pillars the research papers published by Curve Finance and Balancer Labs as well as the DEX Router and Liquidity Bootstrapping pallet:
1. [Curve Finance](https://curve.fi/files/stableswap-paper.pdf) basic functionality and mechanisms for cross-market transactions involving stablecoins.
2. [Balancer AMM](https://balancer.fi/whitepaper.pdf) built on the constant product formula balancer functions similar to Uniswap but applies adjustable weights to set a price difference at pool initialization.
3. [Liquidity Bootstrapping](../../book/src/pallets/liquidity-bootstrapping.md) is based on Balancer AMM and brings the ability to launch a new token. After liquidity has been provided a specified start and ending period is set. The idea is to launch a new token, and after it's initial liquidity has been provided by its creator other users can add liquidity for the token.
4. [Dex-Router](../../book/src/pallets/dex-router.md) provides trading infrastructure for currency pairs with no direct pool in Pablo by combining different pools. Another function of the DEX-Router is wrapping a single pablo pool to mark it as verified therefore distinguishing it from user created pools.

## Workflow
We start by calling the `create` function to initiate pool creation. A pool can be created with either of three configurations: 
- Stableswap
- constant product
- liquidity bootstrapping

Once liquidity is provided to the pool, transactional functions become accessible.
Our transactional functions consist of `buy`, `sell`, `swap`, `Twap`(time-weighted-averaged-price) and basic liquidity pool management functionalities to `add_liquidity` and `remove_liquidity`.
Users can conduct specified swap operations by composing instructions with at least one currency pair.