# Crowdloan Rewards

This pallet allow contributors to claim their rewards.

A % of the reward is paid at the first claim, and the remaining % is vested over an arbitrary period splitted in windows `VestingPartition`.

Rewards are claiming using the following scheme.

1. An `AdminOrigin` setup the reward accounts, consisting of a vector of triple (PublicKey, Amount, VestingPeriod)
   The PublicKey is either comming from the relay chain (Kusama in this case) or from ETH.
2. Since the users don't own any funds on Picasso, the first claim has to be made using our service so that we sign the `associate` transaction using Composable `AssociationOrigin`.
   The first claim result in the Picasso account being associated with the reward account.
   Also, this association automatically trigger a claim, resulting in the first payment done (fixed % if first claim, vested amount otherwise) to the newly associated Picasso account.
3. Once the first claim has been made, the user has to wait until the next `VestingPartition` (probably 1 week).
   After having waited for the vesting partition. The user is able to either `associate` a new account or directly `claim` using it's already associated Picasso account.
   This can be repeated until the contributor has claimed all its reward.

Note: both `associate` and `claim` calls are not charging fees if successful.
