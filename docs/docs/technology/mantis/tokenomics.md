# MANTIS Tokenomics 

Previously known as LAYR, MANTIS acts as the lifeblood to the Mantis platform allowing for a wide range of utility for users and market participants. These functions include, solver onboarding, user staking, transaction fees & bidding, flash loans, and restaking value accrual.

## MANTIS use cases 

### Universal Gas Token
MANTIS serves as Composable’s native gas token amongst transaction fees and orderflow auctions. Transaction fees are collected by the network in order to complete a given transaction, while orderflow auctions serve as a gated process for which searchers bid for their transaction bundles to be included in a block by builders. 

An additional auction may occur between builders, where these providers send additional MANTIS to validators in order to have their block given priority for validation within the network. Under circumstances where a gas token aside from MANTIS is required to complete a transaction (eg. OSMO for a swap on Osmosis DEX), MANTIS may be swapped for the necessary token to complete the order.


### Solver Bonds
In order to onboard as a solver, a MANTIS deposit needs to be made by the market participant. The solver will then be approved to operate within the protocol and participate in orderflow auctions. Securing a bond from a solver acts as a form of safeguarding for the protocol in the event of malicious behavior, or inability to repay a borrow (more detail in the flash loan section). If either of these cases are encountered, a solver bond will be slashed and will need to be topped up to continue engagement within the protocol.

### MANTIS Staking
Users will be able to stake their MANTIS to create a pool accessible by solvers to allow for flash loans towards fees and tips. In return for staking their MANTIS, users receive access to vaults within the platform. Vaults will accrue revenue via protocol fees, MEV share, and restaking fees. Rewards will be claimable in the form of ETH.

### Flash Loans / Borrowing
To reduce overhead for orderflow participants, solvers will be granted access to flash loans from the MANTIS pool created by stakers. These loans may be taken by solvers to supplement gas fees and tips when sending transaction bundles to blockbuilders. Ideally, this reduces the need for solvers to hold active inventory of MANTIS on top of their already committed bond. 

MANTIS borrowed from the pool is to be repaid upon successful completion of a block from which the solver can direct a portion of any profit earned through MEV to repay the pool. In the event that a solver does not generate profit from a given set of transactions, or ends up with losing PnL from their solution, the solvers bond will be slashed to replenish any MANTIS that has been borrowed.

### Restaking
Composable supports network restaking, initially in the form of liquid staked DOT. LSDOT is Composable’s implementation of liquid staked DOT, a receipt token representing DOT staked through Composable. Tokens staked to a PoS network, such as Polkadot, receive rewards for validating blocks within the network. A portion of these rewards are charged as a fee and collected by Composable, from which X% are routed to MANTIS stakers. 

In the future, Composable will allow restaking of additional PoS tokens which will accrue block rewards that will be split with the protocol and distributed to MANTIS stakers in the same fashion as LSDOT rewards.

### Composable Polkadot Activities

In addition, MANTIS secures Composable Polkadot as a governance token and powering collator staking. Similar to OpenGov on Picasso, Composable Polkadot will adopt the OpenGov governance framework and powered by MANTIS will for voting on all governance decisions. Specific tracks, parameters and OpenGov behaviours will be released shortly before going live. Transaction fees generated on Composable Polkadot are split between network collators, the treasury, and MANTIS stakers. Of these fees, 25% are distributed to collators while the remaining 75% of those fees are split between MANTIS stakers and the Composable treasury. 


## Total supply and genesis token distribution

The total supply of 100 million MANTIS tokens are intended to be distributed and released according to the following:


![LAYR_distribution_diagram](./MANTIS-distribution-diagram.png)

**Team**: 25% (25,000,000 MANTIS) of the total token supply will be distributed to founders and the core Composable team as a reward as well as incentives for their continued actions in the best interest in the Composable ecosystem. The distribution of these tokens will have a six-month cliff with linear vesting of the remaining tokens over the subsequent two and a half years.

**Token purchasers**: 23.71% of the total token supply (23,710,772 MANTIS) will be distributed to early supporters and strategic backers. 13.36% (13,359,572 MANTIS) of this allocation will have a 20% unlock at TGE, with the rest vesting over 24 months. 10.36% (10,351,000 MANTIS) of this allocation will be locked for 6-month vesting period followed by linear vesting over 24 months.

**Emissions**: 10% (10,000,000 MANTIS) of the total token supply will be released from the protocol as rewards and incentives for a number of actions involved in the protocol. These are programmatic incentives to bootstrap network growth (block validators, decentralized application builders and token holders from other networks) on Composable as well as token liquidity (liquidity mining programs). The full amount will be allocated upon TGE. *

**Crowdloans**: 16% (16,000,000 MANTIS) of the total token supply will be allocated for the purposes of securing a parachain slot every two years. 16% is being utilized for the current batches of Polkadot auctions, with a 25% vesting on TGE, and the remainder vesting over two years.

**Treasury**: ~20.2% (20209228.31 MANTIS) of the total token supply will be allocated to the Composable Treasury. The full amount will be allocated upon TGE. *

**Polkadot vault strategy**: 5.08% (5,080,000 MANTIS) of the total token supply will be rewarded to participants in our Polkadot vault strategy. ~8.3% of this (423,333.33 MANTIS) will be released at TGE, with the remaining distributed over one year. 

## Release schedule

MANTIS release schedule is shown below:

![LAYR_release](./MANTIS-release-schedule.png)

*All terms related to token allocations are subject to change. [Legal disclosures apply.](https://docs.picasso.network/faqs/disclaimers-disclosures-for-composable-tokens/)
