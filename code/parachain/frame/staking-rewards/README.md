# Overview

This pallet allows protocols to reward users for staking assets.

Stakers are protected from dilution.

- [Overview](#overview)
  - [General flow](#general-flow)
  - [Protocol](#protocol)
  - [Pool](#pool)
    - [Configuration](#configuration)
    - [Rewarding](#rewarding)
    - [Rate based rewards](#rate-based-rewards)
    - [Routing](#routing)
  - [Positions](#positions)
    - [Staking](#staking)
    - [Staked asset](#staked-asset)
    - [Unstake](#unstake)
    - [Unstake while locked](#unstake-while-locked)
    - [Claiming](#claiming)
    - [Split position](#split-position)
    - [Extend amount](#extend-amount)
    - [Extend time](#extend-time)
    - [Expiration](#expiration)
    - [Compounding](#compounding)
  - [Notes](#notes)
  - [References](#references)

## General flow

After paying an existential deposit, any account can create a `staking pool`.

The staking pool has a configuration explaining which staking positions other accounts can have.

User transfers staked assets into the staking pool and choose what position he wants to take. She becomes a `staker`.

Protocol owner increases total rewards share as time goes.

Anybody can get and transfer rewarded assets into the pool. This process can be automated.

A `staker` may leave a position, claim rewards, stake more, etc.
Or she can make [a position to be fNFT](../fnft/README.md) and use it as an instrument in other protocols.


## Protocol

The `staking protocol` is configured with ED for having pools and positions on-chain.  

## Pool

A pool is where users put tokens and get shares of rewards or other powers. 

Any protocol can have one or more pools. The main characteristic of a pool is staked asset identifier(token).

Each pool is governed through a protocol that created it.

### Configuration

To allow the users to stake an asset, we first need to `configure` the pool.

Configuration of a pool defines:

Can/must user time lock tokens to get, possibly increased, `share`. What is the penalty for early unlock if there is a lock.

By default share does not decay with time and extending time subsumes old-time already passed in a lock. The lock gets smaller remaining time with a higher share.

Owner account of the pool. Usually other protocol or its "owner".

The rewards rate "grows" rewards as time goes. Rate can be dynamically updated.
  
**Example**,

The set of staking durations along with their reward multiplier (e.g. `[(WEEK,
  0.5), (MONTH, 0.8), (TWO_MONTH, 1.0)]`).
The early unstake penalty can be set `0.7`.

And asset to be PICA.

Above means that users will be able to stake `PICA` for
either a `WEEK` for a reward multiplier of `0.5` or a `MONTH` etc.
The incentive for the users to stake will be the fact that they will be able to
harvest `PICA` or `BTC` or `ETH` whenever a reward distribution occurred.

See details in `stake positions` operation on how these values influence users.

See pool rewards mechanics on how rewards can be distributed.

### Rewarding

Only the pool owner can transfer the rewards of new assets into a `pool rewards account`. But anybody can transfer rewards of assets that previously were rewarded.

There is a limit to the possible assets' identifiers transferred as rewards. Once reached, cannot add more variety.

Each total reward share increase tracks the amount which each position should get from that. New shares added do not take the amount from previous rewards. That is how `dilution protection`` works.

### Rate based rewards

If the `reward rates` are defined in pools' configurations, a batch of pools randomly is checked on each block if the inflation rate allows increasing the rewards share of specified assets in comparison with the previous increase. If that is the case, users' rewards are increased automatically.

On change of reward rate, up to current block rewards release executed before change applied.

There is a permissionless extrinsic to release rewards into a pool as these accumulated.  

A process of automatic release is capped by time, so if it was not leased with the new time, it stops operating.
It is possible to define a reward rate that does not stop until explicitly stopped by setting it to infinity.
If a pool owner sets the pool's rate to zero that stops automatic rewards releases to users.

A pool may be configured to mint tokens same time it rewards. The default configuration does not mint
This type of operation inflates tokens and should be used with care.

Alternatively, a pool can be configured to incentive desired interest rate (amount of tokens released to be divided by the amount total staked) and desired staking rate (amount of staked divided by total supply) to have desired balance of staked amounts and liquidity. In this case, a rate is automatically adjusted to steer users to the desired interest rate.  This can only be defined if staked asset is the same as a rewarded asset by definition.

**Examples**

The reward rate was set at 100 PICA a day, and  1000 PICA was transferred into the reward pool account.
Each day rewards for users in the pool will be increased by 100 PICA automatically.
In one day, users will be able to claim 100 PICA rewards according to their share.
On day two they will be able to claim up to 200 PICA in the pool.
Unclaimed rewards are accumulated.

### Routing

A pool's owners may define one inflation rate for currency and the proportion of rewards amid several pools.  
So rewards will be split into several pools according to proportions.

A transfer of rewards obeys the same split.

## Positions

A position is what a user (or other protocol) gets when stakes amount. So that one can get a `share` of rewards and other benefits.
Positions capture their configuration upon creation based on what the pool makes possible.
Positions may be updated only on behalf of users. In that case, they will capture a new pool configuration if that was changed.

### Staking

Assuming a user has a configuration for `PICA` that allows us to stake for a month, she can `stake` an amount `X` for once month duration with the amount larger than ED.
The reward multiplier elevates the computed share of the staking pool, the longer you stake, the higher your share.
Let's say we have a reward multiplier of `0.8` for a `MONTH`, if I stake `X PICA`, my share will become `X * 0.8`.

Once she staked `X` tokens, the pallet will create a `position` for an account.
This position allows computing the user's share and the
reward, that the user will be able to claim at any time.

Penalties and lock periods are optional depending on pool configuration.

A stake is transferred into the protocol treasury.

Initially, on a nonzero time lock nonzero penalties no time decay but expirable positions are supported.

### Staked asset

User share amount, potentially elevated, issued as a new token onto asset account owned by staking position account.

Each staking pool has its asset.

Position with all amounts can be wrapped into [fNFT](../fnft/README.md).

### Unstake

A user may unstake position after maturity (lock period ended).
That transfers amounts of shares and rewards to user accounts.

The position must hold enough stake token to burn as it was minted on creation.

### Unstake while locked

A user may leave a position before maturity (if it was defined in `lock duration`), it would likely pay a penalty.
An early unstake penalty applied on the staked asset when the user unstake early (before the end of the selected staking duration).

This case is called `early unstake` and will result in a penalty applied to the
staked asset. A user will be returned only with part of the share.
The remaining will go to the treasury.

**Examples**

Assuming the penalty is defined as being `0.5`, if I staked `10000
PICA`, unstaking will result in a penalty of  `5000 PICA`.
All harvested rewards are still in possession of the user and returned not penalized.

It means that if a user will unstake before a lock ends, he will get half of the locked amount.
The other half will go into the treasury.

### Claiming

A user can claim his pending rewards at any point in time. 
Rewards will be transferer to his account.
A user may leave a reward nominated in the same asset as the share to get compounding.

### Split position

Allows splitting positions without paying a penalty at any time.

The share and rewards are split.

Shares fraction obtained by split must be more than ED.

**Examples**

An owning user has a position with MONTH lock total, with 2 weeks already passed with 100 tokens.
She may split it into 30 and 70 tokens each locked for one MONTH.


It can split the position into several parts 20, 30, and 40 tokens.
Each of which will be the same lock duration and time lock passed.

### Extend amount

A user may add some amount to her stake and increase its share.

In case there is a time lock with share configuration not decreasing with time with early unstake penalty and reward multiplier, then the remaining time lock change incentives user to extend amount in existing position.

In other cases, the time is set to what was provided as a new lock period by the user.

Shares are recalculated accordingly.

**Examples**

```python
original_amount = 100
new_amount = 10000
duration = 100
passed = 50
after_penalized = 0.5 # part of amount remaining after early withdraw
total = after_penalized * original_amount + new_amount
remaining = (after_penalized * original_amount * (duration-passed) +  new_amount * duration  ) / total # captures value that already was in position
print(remaining) # reduced penalized remaining time, so it is better than create new position but not as good as if it was staked originally so much 
```

### Extend time

Position owner may increase lock time.

It can set configure to the same time or larger possible.

She can decrease a time lock only after a position was expired.

In case of share is non-decreasing with time, extension captures the value of time it was locked before.
So duration is increased, but less.

Depending on pull configuration, time may be fully renewed.

If a share is configured to decay as time goes time until a lock ends and extending time starts from zero elapsed.

**Example**

If a position was MONTH and passed two weeks. Can extend it to MONTH or longer.
In case of extending to MONTH, two weeks are zeroed.
In case of extending to YEAR, 2 weeks passed retained in position.

```python
previous_lock = 10
new_lock = 20
passed_time = 2
rolling = min(new_lock - previous_lock, passed_time)
print(rolling) # time it moves to new lock
```

### Expiration

If there is a time-locked position.

After the position expired, a user can unstake without penalty.

If a stake expired, any update on a position will remove its part of the share from the total.

There is a special function that allows anybody to detect expired positions and get a reward for that. Part of ED of the position is transferred to a reporter.

### Compounding

If a position has staked asset to be the same as a rewarding asset and the pool is configured, that reward asset  is subject to compounding.

The reward is staked too and increases users' share to earn more reward.

The user may claim the reward without penalty any time he wants.
So rewards are neither time-locked nor elevated.
The rewarded amount is not subject to a multiplier until locked.

Anybody can run call `compound` to make rewards to increase the position's share.

A user may extend share with time lock too.

**Examples**

`10000 PICA` staked. After one month, the position holds `1000 PICA` rewards.
A user may increase their shares to `11000` PICA.

## Notes

Potentially no implemented (yet) features:

- zero time locks or zero penalty locks (likely works because of math, but not tested)
- ED for pool and positions' state, so that only permissioned creation is possible
- compounding
- automatic inflation adjustment like in Polkadot NPos staking
- routing
- no inflation, only reward pool transfer by governance automatic reward
- decay of leverage if lock duration decreases like Gauges

## References

- https://curve.fi/files/CurveDAO.pdf
- https://github.com/open-web3-stack/open-runtime-module-library/blob/master/rewards/README.md
- https://wiki.polkadot.network/docs/learn-staking
- https://github.com/paritytech/substrate/tree/master/frame/staking
- https://resources.curve.fi/reward-gauges/understanding-gauges
- https://research.web3.foundation/en/latest/polkadot/overview/2-token-economics.html