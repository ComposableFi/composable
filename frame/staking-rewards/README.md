# Staking Rewards

This pallet allow us to reward users for staking certain assets.

# Features

- configuring an asset as being rewardable
- staking an amount of tokens in exchange for a NFT
- claiming reward at any time while the NFT has not expired
- unstaking early, resulting in penalty applied
- unstaking once the NFT expired
- tagging a NFT as being expired, resulting in the ability for third party to
  _steal_ rewards while the NFT is expired and not unstaked

## Configuring an asset as being rewardable

In order to allow the users to stake an asset, we first need to configure it. This
is done through the `configure` extrinsic, allowing us to provide staking
configurations such as:
- the set of staking durations along their reward multiplier (e.g. `[(WEEK,
  0.5), (MONTH, 0.8), (BIMONTH, 1.0)]`)
- the list of assets we can reward the stakers with (e.g. `[PICA, BTC, ETH]`)
- the early unstake penalty, applied on the staked asset when the user unstake
  early (before the end of the selected staking duration)
- the penalty beneficiary, the account where the penalty are going to be
  transferred to

Once this extrinsic has been dispatched, let's assume we want to reward `PICA`
stakers, the mapping will be `PICA => ([(WEEK, 0.5), (MONTH, 0.8), (BIMONTH,
1.0)], [PICA, BTC, ETH])`. Meaning that users will be able to stake `PICA` for
either a `WEEK` for a reward multiplier of `0.5` or a `MONTH` etc... The
incentive for the users to stake will be the fact that they will be able to
harvest `PICA` or `BTC` or `ETH` whenever a reward distribution occured.

## Staking

Assuming we have a configuration for `PICA` that allow us to stake for a month,
we are now able to submit a `stake` extrinsic for an arbitrary amount `X` with a
once month duration. The reward multiplier is acting as a penalty on the
computed share of the staking pool, the longer you stake, the higher your share.
Let's say we have a reward multiplier of `0.8` for a `MONTH`, if I stake `X
PICA`, my share will become `X * 0.8`.

Once we staked `X` tokens, the pallet will mint a `StakingNFT` representing our
position. This NFT will hold the data required to compute our share and the
reward we are able to claim at a time `t`. Like any other asset under
Composable, this NFT is tradable. The NFT has an expiry date, which is the date
at which it has been minted + the staking duration.

## Unstaking while the NFT is locked

This case is called `early unstake` and will result in a penalty applied to the
staked asset. Calling the `unstake` extrinsic on a NFT that is still locked will
unlock it in exchange for a fraction of the stake (configured for the staked
asset).

Assuming the penalty is defined as being `0.5`, if I staked `10_000
PICA`, unstaking the NFT will result in a penalty of  `5_000 PICA`. All
harvested rewards are still in possession of the user.

## Unstaking once the NFT expired

In this canonical case, the user is able to unstake and rapatriate it's staked
asset with no penalty.


## Claiming while the NFT is locked

Because of how the claiming mechanism has been implemented, the rewards are not
automatically transferred to the stakers, an they need to do a manual claim to
actually harvest the rewards. While the NFT is locked, any user is able to claim
on behalf of the owner. Some constraints are applied while claiming though:
- if the origin is the owner, he is able to provide the target account where the
  harvested reward are going to be transferred
- if the origin is not the owner, the rewards are directly transferred to the
  owner

Even if there is not limit in he number of `claim` a user is able to dispatch
for a given NFT, there is an incentive to claim periodically (every reward epoch
to be precise) that we later discuss in the technical details.

## Claiming once the NFT expired

Claiming rewards when a NFT expird is a special case.
As we previously mentioned, the user need to manually claim its rewards, which
can lead to a situation where the user has some rewards that he can claim, but
his NFT expired.

When this situation occur, we have two different scenarios.

The first one is when the NFT has been tagged for expiry (we later discuss in which case a
NFT can be tagged), the claimed rewards will be split betweed the tagger and the
owner. Since a NFT can only be tagged after expiry, the portion of the rewards
that are going to be transferred to the tagger is the extra rewards accumulated
after expiry, hence not expected to be harvested by the NFT owner. To circumvent
the tagger being a second account of the owner, we also penalize the reward
transferred to the tagger. If a user is trying to claim multiple time after
expiry, he will no longer harvest any rewards and in fact, will harvest for the
tagger only (as the first claim will probably allow him to harvest the portion
of rewards he accumulated before expiry).

The second case is more common, if a user is trying to claim after expiry and
his NFT has not been tagged by anyone, we are not able to determine the amount
of rewards that has been accumulated after expiry. Because of that, the user is
forced to restake for the initial duration again to be able to claim the extra
rewards.

## Taggging an expired NFT to harvest rewards

The incentive for third party to tag expired staking NFT is to be able to
harvest for rewards. Submitting a `tag` extrinsic is free of fee if the NFT has
not been tagged already. When tagging, the tagger is able to provide a
beneficiary account, used when claiming and transferring extra rewards
accumulated by the expired NFT. The tagger will be able to harvest rewards
accumulated after the tag.

## Rewarding stakers

Any protocol is able to rewards the stakers by calling the
`StakingRewards::transfer_reward` implementation of this pallet.

# Technical details

In order to avoid having to iterate over all the positions when rewarding, we
need a way to track how to compute the reward for a user, using the current
pallet state and the NFT data.

To track the collected rewards, we use an accumulator for each reward asset.
When a user is staking, we store a copy of the accumulators in the NFT.
When a user is claiming, we do the delta and compute it's shares given the reward multiplier.
To avoid instant dillution, a newly minted NFT shares is taken into account when
the current reward epoch ended (parameter to the pallet, let's say half week).
