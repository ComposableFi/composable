# Liquidity Pools on Launch

On launch, Pablo was the only platform for acquiring PICA through dual asset constant product pools, 
based on the BalancerFi weighted math implementation of (x * y = K) for:

- KSM/USDT
- PICA/USDT
- PICA/KSM

Since of the launch of Centauri, we have added the following pools:

- PICA/DOT
- DOT/KSM
- DOT/USDT


Liquidity providers(LPs) directly benefit from the success and adoption of Pablo.
LPs are provided LP-tokens relative to the funds they add to a liquidity pool on Pablo.
A percentage of the total buy/sell order and swap fees are distributed to liquidity providers.
Then the accumulated fees are added back to the pool, effectively resulting in a pro-rata redistribution to LPs
based on the LP-token share they redeem and the total value of the pool.

Pablo is uniquely positioned as the hub for cross-chain transactions in DotSama, and community backed liquidity. 
The initial pools are a mere first step on our path to deep cross-chain liquidity 
and will be followed by stableswap and liquidity Bootstrapping pools with improved trading opportunities and customization.

## How fees work on Pablo

### Transaction Fees 
Transaction fees are paid by any user posting transactions on-chain.
All Fees are calculated based on the “weight” of a transaction representing the computational load and storage cost. The asset used for fees can be changed via BYOG (Bring Your Own Gas) to a configured fee asset of the user's choice.

### Swap Fees
Transactions through Pablo native liquidity pools for asset swaps and buy/sell orders incur fees at
0.3% of the total amount traded and are paid by the trader in the input asset.

### Initialization of pools 
According to this passed proposal at the following link: https://picasso.subscan.io/council/2, the initial pools were seeded by the treasury at $150m FDV for PICA with $50,000 of liquidity, and the 30 day price of KSM for the KSM/USDT pool with $50,000 of liquidity as well. On Tuesday, the 27th of December, trading began at 19:00 UTC (14:00 EST) on Pablo. 
