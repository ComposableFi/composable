# Technical Overview

The primary interaction between Parachains and the Polkadot Relay chain within the Polkadot ecosystem occurs through Cross-Chain Messaging (XCM). From the perspective of users on Composable Polkadot, the main concern is to stake and unstake DOT, without needing to worry about the underlying mechanics. 

The Liquid Staking pallet residing on Composable Polkadot is responsible for executing functions such as bonding, unbonding, and rebonding with the Polkadot Relay chain. The Relay chain updates validators in every Era, and the Liquid Staking pallet processes user requests and operates in sync with the Relay chain's Era updates. This functionality is facilitated by the MatchingPool module.

:::info
An Era refers to a (whole) number of sessions, which is the period that the validator set and each validator's active nominator set is recalculated and where rewards are paid out. An era equals one day on the Polkadot Relay chain.
:::

## DOT Distribution

To optimise APY and risk management, the Liquid Staking pallet utilises multiple derivative accounts on the Polkadot Relay chain, each of which acts as an independent stash account. These accounts are used to distribute funds effectively based on a sophisticated fund allocation algorithm. 

### Staking Reward Updates Mechanism
When staking rewards are updated on the Polkadot Relay chain, an off-chain relayer verifies these changes. The relayer submits Merkle storage proofs that the reward adjustment has occurred on the Relay chain. This process is fully decentralised as the relayer does not have access to any of the staked funds and is open for participation by any interested party, ensuring a distributed and trustless operation.

### Stake and Unstake Processing 

The Liquid Staking pallet manages all users' stake and unstake requests within a single period. It initiates the necessary bonding or unbonding actions on the Relay chain. In scenarios where the initial period involves unbonding and the subsequent period involves bonding, the pallet may prioritize rebonding in the first instance.

### Period Duration

Typically, one period aligns with the duration of one era. This synchronization with eras is essential as it allows the Relay chain to comprehensively update users' accounts at regular intervals.

### Era-Based Accumulation

In each era, the staked amounts contributed by all users are cumulatively aggregated into a parameter termed `total_stake_amount`, while the total of all users' unstaked amounts are compiled under the parameter `total_unstake_amount`. This assessment informs the decision-making process, determining whether actions such as unbonding, rebonding, or bonding should be executed in the subsequent era.


### `MaxMinDistribution Algorithm`

The `MaxMinDistribution` algorithm governs the allocation of funds for various operations:

- **`Bond`**: The account with the minimum active funds is selected to execute the bonding operation, ensuring efficient utilization of available assets.

- **`Unbond`**: The account with the maximum active funds is chosen to perform the unbonding operation, promoting risk mitigation through the use of ample resources.

- **`Rebond`**: For rebonding, the account with the maximum unlocking funds is selected. This approach ensures that unlocked funds are strategically reinvested into the staking ecosystem.

This allocation strategy helps maintain the balance between liquidity and network security while maximizing returns for participants in the liquid staking protocol.

### `AverageDistribution`

The `AverageDistribution` mechanism pertains to the execution of `Bond`, `Unbond`, and `Rebond` operations across all accounts within the network when their associated asset amounts fall within the average range.

The formula for the price of LSDOT is as follows:

$$
\text{Liquid Staked DOT Price Model} = \frac{\text{TotalActiveBonded} + \text{TotalStake} - \text{TotalUnstake}}{\text{sTokenIssuance}}
$$

The definitions of the components in this formula are as follows:

- `TotalActiveBonded`: This metric aggregates the active bonded amount across all derivative accounts on the Relay chain.
- `TotalStake`: The cumulative stake amount contributed by all network users throughout the current era.
- `TotalUnstake`: The total amount of funds that users have withdrawn or unstaked within the current era.
- `sTokenIssuance`: Denotes the latest issuance of LSDOT in the market.