# 2-assets-matching


Implementation of a Frequent batch auction with two assets. 
Three different solvers are implemented. 
A solver that maximizes arbitrage, an (altruistic) solver that maximizes volume, and a solver that has a target price willing to trade (we can think of he is able to arbitrage it in an off-chain market maker).

## Proposed usage

1. Solver solves Coincidence of Wants orders
2. Remaining amount solved, if possible, via cross chain routes  