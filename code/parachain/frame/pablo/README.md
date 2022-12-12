# Pablo
Pallet Pablo provides extensive functionality to set up an exchange, 
enabling users to create, swap assets with, and manage liquidity pools.

## Overview
Pablo's is based on the balancer protocol in terms of liquidity pool mechanisms.
The balancer protocol at its core is an automated market maker and can be described in simple terms as a price balancing
asset exchange. 
[Balancer AMM](https://balancer.fi/whitepaper.pdf): built on the constant product formula, 
balancer functions similar to Uniswap but applies adjustable weights to set a price difference at pool initialization.


## Workflow

We start by calling the `create` function to initiate the configuration of a [constant product pool](https://balancer.fi/whitepaper.pdf).

Once liquidity is provided to the pool, the following transactional functions become accessible:
- `buy`
- `swap`
- `Twap` (time-weighted-averaged-price)

and basic liquidity pool management functions: 
- `add_liquidity`
- `remove_liquidity`

Users can also conduct specified swap operations by composing instructions with at least one currency pair.

## Time weighted averaged price

The TWAP is a [counter mechanism] aimed to prevent and discouraging malicious actors.
Specifically, from making large trades to manipulate the reflected price of the liquidity pool
and exploit the price momentum in smart contracts using the new price.

[counter mechanism]: https://en.wikipedia.org/wiki/Kernel_smoother