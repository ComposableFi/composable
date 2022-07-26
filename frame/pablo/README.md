# Pablo
Pallet Pablo provides extensive functionality to set up an exchange, enabling users to create, trade with, and manage, liquidity pools.


## Overview
Pablo builds on four pillars the research papers published by Curve Finance and Balancer Labs as well as the DEX Router and Liquidity Bootstrapping pallet:
1. [Curve Finance](https://curve.fi/files/stableswap-paper.pdf) basic functionality and mechanisms for cross-market transactions involving stablecoins.
2. [Balancer AMM](https://balancer.fi/whitepaper.pdf) built on the constant product formula balancer functions similar to Uniswap but applies adjustable weights to set a price difference at pool initialization.
3. DEX Router; provides trading infrastructure for currency pairs with no direct pool in Pablo by combining different pools.
   Another function of the DEX-Router is wrapping a single pablo pool to mark it as verified therefore distinguishing it from user created pools.
4. Liquidity Bootstrapping; is based on Balancer AMM and brings the ability to launch a new token. After liquidity has been provided a specified start and ending period is set.
   The idea is to launch a new token, and after it's initial liquidity has been provided by its creator other users can add liquidity for the token.

## Workflow
We start by calling the `create` function to create an initial pool. Once liquidity is provided to the pool we can start calling basic transactional functions.
Our transactional functions consist of `buy`, `sell`, `swap` and basic liquidity pool management functionalities to `add_liquidity` and `remove_liquidity`. They always reference a currency pair .
Users can also conduct specific swap operations by composing instructions based on a quote asset in native currency and one or more given foreign assets.
