# PICA Use Cases

Picasso was born on Kusama. The vision was to introduce the world to cross-ecosystem IBC, and build a flourishing ecosystem within the Polkadot ecosystem. However, whilst on this journey to bring IBC to multiple ecosystems, [Composable Cosmos](../composable-cosmos.md) was launched, to facilitate the flow of assets beyond the Polkadot verse. 

And so, PICA was born a natively cross-chain token. Powering multiple chains. Additional value will accrue to the token based on the success of IBC's ongoing cross-pollination efforts. PICA is the native token of [Picasso](../picasso-parachain-overview.md) and [Composable Cosmos](../composable-cosmos.md).

A concerted effort has been made to ensure that the PICA token holds as much utility as possible by incorporating various value accrual methods and governance features. While the PICA token provides rewards for participating within the ecosystem, it is also fundamental for the operation of collators, validators, oracles, and other cross-ecosystem strategies.

## What is $PICA used for?
A summary of PICA use cases is as follows:

- Collator staking on Picasso 
- Apollo staking
- Validator Staking
- Liquid staking (coming soon)

## Tri-staking token model

Within the Picasso ecosystem there are multiple staking avenues that can be utilized to earn yield in PICA. This tri-staking model strengthens the security of the network while maintaining liquidity and distributing maximum value back to Picasso token holders.

### Oracle staking​
Apollo is a permissionless, MEV-resistant oracle solution. Anyone can run an oracle node on Picasso by providing a PICA stake.

### Collator staking​ 
25% of fees on Picasso are distributed to collators, with the remaining 75% going directly to the community-governed treasury. Collators on Picasso are required to put down a stake to produce blocks on our parachain, as with most proof of stake networks. 

### Composable Cosmos staking
PICA is also used to secure the Composable Cosmos chain. This is the first instance of a token being utilized for validation within both the Kusama and Cosmos ecosystems and highlights the critical role PICA plays within cross-ecosystem communication. The Delegation Program sources 1bn PICA tokens from the Picasso Treasury to validators to ensure a robust and secure network while providing a ~10% APR in PICA.

## Polkadot Liquid Staking
Composable is launching Liquid Staked DOT (LSDOT) in the ecosystem. Revenue will flow to PICA, providing yield emissions from liquid staking, but only while stakings are locked.  The value accrual for liquid staking is a 1% fee + 75% of 10% of the yield generated. 

## Staking Reward Curve for Bridging Fees

Users may stake their PICA for a % share of revenue generated from bridging fees. The PICA staking rewards curve acts as a method of rewarding users who have staked their PICA over a threshold duration of time. Users who hit a maximum duration of 90-days staked will receive a proportional share of the 20% of bridge revenue allocated towards PICA stakers. 

| Days | % of Fee-Split | Redistributed % |
| ---- | -------- | -------- |
|  1    |   1.11%       |    -------      |
|  7     |  7.78%        |   -------       |
|  14     |   15.56%       | -------         |
|  30     |   33.33%       |  15%        |
|  60     |   66.67%       |  35%        |
| 90  | 100%       | 50% |

Per the table above, a user accrues a greater % of their revenue share linearly over the course of their staking period until reaching the maximum of 90-days, at which point, they will begin to accrue their full share of the 20% revenue split. Users who have not reached the full 90-day threshold, will have their remaining share redistributed upward across the curve. 

**Example**

As a simple example - take two users that have staked their PICA. User A has staked for 90-days and user B has staked for 1-Day. 

User A Rev Share = 100% * (PICA_Staked / Total_PICA_Staked) * (20% Bridging Revenue), whereas,

User B Rev Share = 1.11% * (PICA_Staked / Total_PICA_Staked) * (20% Bridging Revenue)

Further, 98.89% of User B’s Rev Share would be allocated to users further up the curve (in this case, there is only User A)
Share of redistributed revenue up the curve for longer-duration stakers will only be split amongst the 30/60/90 day threshold as follows:

Share of redistributed revenue up the curve for longer-duration stakers will only be split amongst the 30/60/90 day threshold as follows:

- 30 Days <= x < 60 Days : receive 15% of redistributed revenue from stakers lower in the curve
- 60 Days <= x < 90 Days : receive 35% of redistributed revenue from stakers lower in the curve
- 90 Days <= x : receive 50% of redistributed revenue from stakers lower in the curve

## The first cross-ecosystem token
As Picasso plays a pivotal role in both our Kusama and Cosmos strategies, PICA will be utilized across both ecosystems as we continue to explore new use cases and integrations.

### Gas (network usage)​
The PICA token is uniquely positioned as the gas token at the center of Picasso, powering Composable’s efforts to enhance blockchain interoperability. PICA will also act as the gas token for the CosmWasm dApps deployed on the `ccw-vm` on Picasso. Notably, in order to further support users from other ecosystems Picasso offers a feature called “bring your own gas” (BYOG), which allows users to pay their gas fee in any supported tokens.

All fees may change dynamically depending on network load and pool or protocol fee formulas. The most fundamental factor for gas fees is the computational resources it consumes which is represented as the transaction's "weight". The weight of a transaction is converted into an appropriate amount of PICA by the polynomial formula which changes dynamically depending on the target load of the network. This means as the usage of the chain increases towards maximum capacity, the price of a unit of weight increases as well.

### Primary pairing on Pablo​

Pablo is the native DEX of the Picasso ecosystem and is integrated directly into the runtime of our parachain as a pallet. As such, a primary trading pair on Pablo will be PICA. You can also expect various liquidity incentives with 15% of PICA’s supply being allocated towards liquidity programs. 

### Powering advancements in the Cosmos
New use cases will continue to be established as we move forward with HRMP channel openings and expansion into and beyond the Cosmos.

## Governance

Picasso is waging war on centralization with a vision of a seamlessly interoperable, trustless future for DeFi. As such the PICA token will play an important role in helping to realize this vision, as governance will soon be powered by phase 2 of Picasso's [OpenGov](./governance.md).

## Where is PICA available?

Since its launch, the PICA token has been made available across a number of different blockchain ecosystems thanks to Composable IBC. Therefore, PICA can be acquired on Composable’s own [Pablo DEX](https://app.pablo.finance/) on Picasso and the [Osmosis DEX](https://app.osmosis.zone/).
