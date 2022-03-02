# Crowdloan Rewards

The Crowdloan Rewards pallet allows contributors to claim their rewards.

## Rewards

A percentage of the reward is paid at the first claim, and the remaining 
percentage is vested over an arbitrary period of time split into
`VestingPartition`s.

Rewards are claimed using the following steps:

1. An `AdminOrigin` sets up the reward accounts, consisting of a vector of 
   (PublicKey, Amount, VestingPeriod).  
   The PublicKey is either comming from the relay chain (Kusama in this case) or
   from ETH.

2. Since the users don't own any funds on Picasso, the first claim has to be 
   made using our service so that we sign the `associate` transaction using 
   Composable's `AssociationOrigin`.  
   The first claim results in the Picasso account being associated with the 
   reward account. This association automatically triggers a claim, the claim 
   results in the first payment being distributed to the newly associated 
   Picasso account.

3. Once the first claim has been made, the user has to wait until the next 
   `VestingPartition`.  
   After having waited for the vesting partition. The user is able to either 
   `associate` a new account or directly `claim` using their already associated 
   Picasso account.  
   This can be repeated until the contributor has claimed all of their reward.

## Unsigned Transactions

To enable unsigned transactions while still preventing spam, we implement 
'ValidateUnsigned'. Our implementation insures the following.

1. Transaction payloads contain the elements needed for validation (reward 
   account ID and proof).

2. The pallet hes been initialized.

3. The reward account exist and has not been associated.

4. The proof is valid for the reward account and a remote account are accessible 
   from the proof.

5. There is some reward to claim for the reward account.

## Notes

* both `associate` and `claim` calls do not charge fees if successful.
