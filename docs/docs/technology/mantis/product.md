# UX flow




`I want to get 100 of A for 90 of B` is `intent`.


Solvable by:
1. Liquidity pools with curve (lending, CFMM, stable pools, staking, provide liquidity) 
2. Orders - other user wants opposite.

### Orderbooks

1. FIFO which optimizes for fairness
2. Optimized for volume `max(A * B)` - Coincidence of Wants solved by Batch Auctions (many orders solved at a time).

### CoW Batch Auction order book

`intent` on chain is `order` with order identifier and some conditions:
- timeout after which order is canceled
- partial fill of order is okey or not. example, I am ok to solve 50% of my order.

`Order` of user is solved until timeout or full fill.

### Order price

We do not use on chain oracles to form limits (examples are price A/B or slippages).

But FE must suggest user some limit using indexer of TWAP Oracles out there.

Example, `A/B` is `price` on Osmosis is `0.50`. 
Than user default order formed by frontend can be like `0.45`.
That defines limit like slippage. 
It prevents user from being totally rugged. 
But more tight limit is more chances order to be timeout.


### How actually it is swapped?

Background trustless solvers run by some random people,
find overlapping prices and post `solutions` if orders match.

Examples, order `K` A to B price is `0.5` and order `L` price is B to A `0.45`.
If `K<L` than `solution` can salify limits and match orders.

If there are many many orders and several solutions, than solution with maximum volume is accepted by contract.

It tries to make sure user gets more than limit.

Solver compete for highest volume so that they improve upon user limits.

### On chain and cross chain parts

What is there is not satisfying counter order? 
And there liquidity on order contract chain.

In this case solvers form CVM cross chain routing program which swaps on pools.

It also solves just transfer order, because transfer is just "bridge pool minus bridge fees".

In this case amount in user order is not settled in single block,
but should be observer in multiple blocks on several chain.  

Many orders can be grouped in single CVM batch,
so that gas fees are shared.

# TODO: bind in event order to solution

