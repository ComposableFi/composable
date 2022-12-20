# How fees work on Pablo

## Overview
Transaction fees are paid by any user posting transactions on-chain.
Fees are calculated based on the “weight” of a transaction representing the computational load and storage cost.

### Buy, Sell & Swap Fees
Transactions through Pablo liquidity pools for asset swaps and buy/sell orders incur fees at
0.3% of the total amount traded and are paid by the trader in the input asset. 
The asset used for transaction fees can be changed via 
BYOG (Bring Your Own Gas) to a configured fee asset of the user's choice.
A percentage of the total order and swap fees are distributed as rewards to LPs for locking funds in liquidity pools. 
Accumulated fees are added back to the pool, resulting in a pro-rata redistribution to LPs 
based on the LP-token share they redeem and the total value of the pool.
In the event that a user adds liquidity to a pool in such a manner that it creates 
a certain degree of liquidity imbalance in the pool, the user is charged the standard 0.3% fee.

