# Staking Rewards

This pallet allow us to reward users for staking certain assets.

## Features

- configuring an asset as being rewardable
- staking an amount of tokens in exchange for a NFT
- claiming reward at any time while the NFT has not expired
- unstaking early, resulting in penalty applied
- unstaking once the NFT expired

### Configuring an asset as being rewardable

In order to allow the users to stake an asset, we first need to configure it. This
is done through the `configure` extrinsic, allowing us to provide staking
configurations such as:

- the set of staking durations along their reward multiplier (e.g. `[(WEEK,
  0.5), (MONTH, 0.8), (TWOMONTH, 1.0)]`)
- the list of assets we can reward the stakers with (e.g. `[PICA, BTC, ETH]`)
- the early unstake penalty, applied on the staked asset when the user unstake
  early (before the end of the selected staking duration)
- the penalty beneficiary, the account where the penalty are going to be
  transferred to

Once this extrinsic has been dispatched, let's assume we want to reward `PICA`
stakers, the mapping will be `PICA => ([(WEEK, 0.5), (MONTH, 0.8), (TWOMONTH,
1.0)], [PICA, BTC, ETH])`. Meaning that users will be able to stake `PICA` for
either a `WEEK` for a reward multiplier of `0.5` or a `MONTH` etc... The
incentive for the users to stake will be the fact that they will be able to
harvest `PICA` or `BTC` or `ETH` whenever a reward distribution occured.

### Staking

Assuming we have a configuration for `PICA` that allow us to stake for a month,
we are now able to submit a `stake` extrinsic for an amount `X` with a
once month duration with amount larger than existential deposit(ED).
The reward multiplier is acting as a penalty on the
computed share of the staking pool, the longer you stake, the higher your share.
Let's say we have a reward multiplier of `0.8` for a `MONTH`, if I stake `X
PICA`, my share will become `X * 0.8`.

Once we staked `X` tokens, the pallet will mint a staking class  `fNFT` representing our
position. 
This NFT will hold the data required to compute our share and the
reward we are able to claim at a time `t`. Like any other asset under
Composable, this NFT is tradable. The NFT has an expiry date, which is the date
at which it has been minted + the staking duration.

### Unstaking while the NFT is locked

This case is called `early unstake` and will result in a penalty applied to the
staked asset. Calling the `unstake` extrinsic on a NFT that is still locked will
unlock it in exchange for a fraction of the stake (configured for the staked
asset).

Assuming the penalty is defined as being `0.5`, if I staked `10_000
PICA`, unstaking the NFT will result in a penalty of  `5_000 PICA`. All
harvested rewards are still in possession of the user.

### Unstaking once the NFT expired

In this canonical case, the user is able to unstake and rapatriate it's staked
asset with no penalty.

### Claiming

A user is able to claim his pending rewards at any point in time.
As the protocol is rewarding the NFT itself, all pending rewards are tied to the NFT only.
Meaning that if a user trade a NFT including pending rewards, the new owner will be able to unlock them.

### Rewarding stakers

Any protocol is able to rewards the stakers by calling the
`StakingRewards::transfer_reward` implementation of this pallet.
