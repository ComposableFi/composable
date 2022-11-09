# Crowdloan Rewards Integration Guide

[**Pallet Overview & Workflow**](https://github.com/ComposableFi/composable/blob/0fa1f1986ab91ad5bdbd69437bd90f47e077983f/code/parachain/frame/crowdloan-rewards/README.md)<!-- TODO replace this with a valid monorepo link -->

## Integration Status

| Dali | Picasso | Composable |
|------|---------|------------|
| Yes  | Yes     | No         |

## Setup / Configuration

<!-- *Include any notes about pallet lifecycle or states. A state diagram that notes
transition requirements if you're feeling fancy* -->

Crowdloans are created and managed by admins. Therefore, only admins can access the life cycle methods of the crowdloan.

Crowdloans have three states: created, initialized, and unlocked. These states transition linearly:
During the created state, management transactions can be conducted to `populate` the crowdloan with funds and a vesting 
schedule. After transferring enough funds, the crowdloan can be initialized.
During the initialized state, users can associate their contributor account with a reward account by providing a proof.
During the unlocked state, contributors can claim the initial reward percentage and afterward claim rewards with each 
vesting step.

Automatic state transition can occur if the crowdloan was provided with a timestamp to `initialize_at` and that time has 
come, the crowdloan will be initialized.

## RPC & Data Retrieval

<!-- *RPCs w/ links to cargo docs?* -->


## Subsquid Data Retrieval

<!-- *Not required yet since we have no subsquid yet* -->


## Locally Consumed Types

### Types

- `Balance` - Numeric type used to represent some amount of tokens
- `RewardAsset` - The `RewardAsset` used to transfer the rewards
- `Moment` -  Numeric type used to express a time stamp
- `Time` - The Time provider
- `Admin Origin` - The origin that is allowed to `initialize` the pallet
- `Convert` - Function for converting between `Moment` and `Balance`
- `RelayChainAccountId` - Numeric type used to uniquely identify relay chain accounts
- `Weight Info` - Provider for extrinsic transaction


### Constants

- `InitialPayment` - The upfront liquidity unlocked at first claim
- `OverFundedThreshold` -  The percentage of excess funds required to trigger the `OverFunded` event
- `VestingStep` - The time you have to wait to unlock another part of your reward
- `Prefix` - The arbitrary prefix used in proofs
- `PalletId` - The unique identifier of this pallet
- `LockId` - The unique identifier for locks maintained by this pallet
- `LockByDefault` - Configuration if claimed amounts should be locked by the pallet


## Calculations & Sources of Values

<!-- *"Provide calculations of APY or APR if any and mention the source of all values
that need to be fetched from the chain/backend/subsquid or any other data source"* -->

<!-- MartinK - QUESTION: Should we include a calculation for the reward percentage upon first claim here? -->


## Extrinsic Parameter Sources

<!-- *Document sources of extrinsic parameters, hard coded, calculated on the front end, user provided* -->

| Extrinsic          | Parameters        | Type                                                     | Source        |
|--------------------|-------------------|----------------------------------------------------------|---------------|
| initialize_at      | at                | U64                                                      | Sudo provided |
|                    |                   |                                                          |               |
| populate           | rewards           | Vec(PalletCrowdloanRewardsModelsRemoteAccount, u128,u64) | Hardcoded     |
|                    | - RemoteAccountOf | PalletCrowdloanRewardsModelsRemoteAccount                | Sudo provided |
|                    | - RewardAmountOf  | u128                                                     | Calculated    |
|                    | - VestingPeriodOf | u64                                                      | Sudo provided |
|                    |                   |                                                          |               |
| associate          | rewardAccount     | AccountId32                                              | User provided |
|                    | proof             | PalletCrowdloanRewardsModelsProof                        | User provided |
|                    |                   |                                                          |               |
| unlock_rewards_for | AccountId         | AccountId32                                              | Sudo provided |


## Pricing Sources

<!-- *"Pricing sources are a must have if any Zeplin designs show users values in USD $"* -->