# Crowdloan Rewards

The Crowdloan Rewards pallet allows contributors to claim their rewards.

## Overview

### Unsigned Transactions

Before associating their account, users will not have the funds necessary to pay 
transaction fees. To get around this, unsigned transactions are allowed but must 
be validated.

For a transaction to be validated, these conditions must be met:

* Transaction contains a reward ID and valid proof

* The pallet has been initialized

* The reward account contained in the call has not been associated

* The remote account (from ETH or relay chain) is retrievable from the proof

* The reward account has a positive reward balance available to claim

## Rewards Workflow

After reward accounts have been populated and the pallet has been initialized, 
contributors can start to claim their rewards. A percentage of the reward is 
paid at the first claim, and the remaining percentage is vested over an 
arbitrary period of time split into Vesting Partitions.

**Rewards are claimed by the following steps**

1. An `AdminOrigin` sets up and populates the reward accounts, consisting of a 
  vector of (PublicKey, Amount, VestingPeriod). The PublicKey is either coming 
  from the relay chain (Kusama in this case) or from ETH.

2. An `AdminOrigin` initializes the pallet with the `initialize` or 
  `initialize_at` extrinsics

3. Since the users don't own any funds, the first claim has to be made using our 
  service so that we can `associate` and fund the account. The association 
  results in the remote account being associated with the reward account. This 
  association automatically triggers the first claim, the claim results in the 
  first payment being distributed to the newly associated Picasso account.

4. Once the first claim has been made, the user has to wait until the next 
  `VestingStep`. After having waited for the vesting partition. The user is able 
  to either `associate` a new account or directly `claim` using their already 
  associated Picasso account. This can be repeated until the contributor has 
  claimed all of their reward.

## Notes

* both `associate` and `claim` calls do not charge fees if successful.
