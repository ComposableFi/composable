# Design Proposal: Pablo Fees & Staking Rewards Distribution

Table of Contents

-   [1. Abstract](#1-abstract)
-   [2. Background](#2-background)
    -   [2.1. PBLO Token Initial
        Distribution](#21-pblo-token-initial-distribution)
    -   [2.2. Pool Fees](#22-pool-fees)
        -   [2.2.1. LP Fee Distribution](#-lp-fee-distribution)
-   [3. Use Cases](#3-use-cases)
-   [4. Requirements](#4-requirements)
    - [4.1. Pablo Liquidity Providers](#41-pablo-liquidity-providers)
    - [4.2. PBLO Stakers](#42-pblo-stakers)
    - [4.3. PICA Stakers](#43-pica-stakers)
    - [4.4. Pablo Governance](#44-pablo-governance)
    - [4.5. PICA Governance](#45-pica-governance)
    - [4.6. Technical Requirements](#46-technical-requirements)
    - [4.7. Financial NFT Requirements](#47-financial-nft-requirements)
-   [5. Method](#5-method)
    -   [5.1. System Overview](#51-system-overview)
    -   [5.2. Pallet-Pablo](#52-pallet-pablo)
        -   [5.2.1. FeeConfig](#521-feeconfig)
        -   [5.2.2. LP Trading Fee
            Distribution](#522-lp-trading-fee-distribution)
        -   [5.2.3. PBLO Staker Pool
            Creation](#523-pblo-staker-pool-creation)
        -   [5.2.4. PBLO Staker Trading Fee
            Distribution](#524-pblo-staker-trading-fee-distribution)
    -   [5.3. Pallet Staking Rewards - LP/PICA/PBLO/Other Token Staking
        Reward
        Pools](#53-pallet-staking-rewards-lppicapbloother-token-staking-reward-pools)
        -   [5.3.1. Analysis of Reward
            Calculations](#531-analysis-of-reward-calculations)
            -   [5.3.1.1. When adding a new staker <span
                class="image"><img src="0005-pablo-distribution-assets/images/stem-0b46f732c83c0e66067b0e50c2156089.png" width="29" height="8" alt="stem 0b46f732c83c0e66067b0e50c2156089" /></span>,
                existing stakers(<span
                class="image"><img src="0005-pablo-distribution-assets/images/stem-55a049b8f161ae7cfeb0197d75aff967.png" width="9" height="6" alt="stem 55a049b8f161ae7cfeb0197d75aff967" /></span>)
                reward would
                be,](#5311-when-adding-a-new-staker-n1-existing-stakers-reward-would-be)
            -   [5.3.1.2. When removing a staker(Claim/Unstake) from the pool the above
                addition step has to be
                reverted](#5312-when-removing-a-stakerclaimunstake-from-the-pool-the-above-addition-step-has-to-be-reverted)
            -   [5.3.1.3. When adding a new reward to the pool the
                calculations remain the same other than increasing the
                reward pool as
                follows,](#5313-when-adding-a-new-reward-to-the-pool-the-calculations-remain-the-same-other-than-increasing-the-reward-pool-as-follows)
            -   [5.3.1.4. When extending an existing
                position](#5314-when-extending-an-existing-position)
            -   [5.3.1.5. When splitting an existing
                position](#5315-when-splitting-an-existing-position)
        -   [5.3.2. Data Structures](#532-data-structures)
        -   [5.3.3. Staking](#533-staking)
        -   [5.3.4. Extend Position](#534-extend-position)
        -   [5.3.5. Split Position](#535-split-position)
        -   [5.3.6. Claim/Unstake](#536-claimunstake)
         -   [5.3.7. Reward Pool Governance](#537-reward-pool-governance)
            -   [5.3.7.1. Update Reward Allocation Per
                Pool](#5371-update-reward-allocation-per-pool)
            -   [5.3.7.2. Update Reward Pool](#5372-update-reward-pool)
        -   [5.3.8. RewardAccumulationHook](#538-rewardaccumulationhook)
        -   [5.3.9. Claim](#539-claim)
-   [6. Implementation](#6-implementation)
    -   [6.1. Pallet Pablo: LP Fee + Staking
        Changes](#61-pallet-pablo-lp-fee-staking-changes)
    -   [6.2. Pallet Staking Rewards: PICA/PBLO Staking Related
        Changes](#62-pallet-staking-rewards-picapblo-staking-related-changes)
-   [Appendix A: Trading Fee Inflation to Avoid Dilution of
    LPs](#appendix-a-trading-fee-inflation-to-avoid-dilution-of-lps)
-   [Appendix B: Fee Distribution Q&A](#appendix-b-fee-distribution-qa)

## 1. Abstract

This document proposes the Pablo distribution(token and pool trading
fees) mechanism while considering various options and capturing the
discussions about the subject.

`TODO summarize the mechanism`

## 2. Background

### 2.1. PBLO Token Initial Distribution

**NOTE**: PBLO token does not have a set release date and its details
are not finalized.


The farming rewards are incentives for liquidity provider(LP)s who stake
their LP tokens for Pablo pools. How much reward is allocated as
incentive for each pool is to be decided by governance.

### 2.2. Pool Fees

Composable intends to distribute some percentage of the
swap(transaction) fees captured by the pools in Pablo dex pallet as
rewards to users who stake their `PBLO` tokens using the staking-rewards
pallet interface. The idea is to incentivize the continuous owning of
the staked `PICA` and `PBLO` to earn these yields which increases the
value of the ecosystem overall by increasing the desirability of the
staked assets.

At the time of writing Pablo has the following fee parameters other than
for liquidity bootstrapping pools(LBP) which do not charge fees,

1.  LP Fee - A percentage of the trading fee that is distributed to
    liquidity providers based on the number of liquidity provider(LP)
    tokens they minted at the time of providing the liquidity.

2.  Pool Owner Fee - A percentage of the trading fee that is distributed
    to the pool owner.

#### 2.2.1. LP Fee Distribution

This is yet to be implemented in Pablo, hence the idea is that it can be
addressed in the context of this proposal.

## 3. Use Cases

Following is a summary of use cases omitting the UI specific use cases
for brevity.

<img src="0005-pablo-distribution-assets/images/images/pablo-distribution-users.png" width="523" height="1182" alt="pablo distribution users" />

## 4. Requirements

### 4.1. Pablo Liquidity Providers

1.  LPs MUST be able to stake their LP tokens to earn rewards allocated
    for a particular pool.

    1.  Rewards can be in terms of PBLO, PICA or any other tokens.

    2.  Same pool can receive multiple types of tokens as rewards.

2.  The system MUST support accumulating the LP share of Pablo trading
    fees.

3.  Pablo trading fees(LP fee part) MUST be disbursed according to LP
    token share of each LP. Fees are accumulated towards increasing
    liquidity in a pool while allowing LPs to redeem the fee share with
    their LP tokens at a preferred time.

### 4.2. PBLO Stakers

1.  System MUST allow staking of PBLO. This must be implemented through
    the fNFT mechanism with multiple time period unlocks being possible
    for users.

2.  The system MUST accumulate the rewards share for PBLO holders who
    stake PBLO token, out of the PBLO supply allocated for them.

3.  The system MUST support accumulating the (stakers) reward part of
    the Pablo trading fees.

4.  The system must support rewards being distributed on granular
    basis - e.g every 6 or 12 hours.

5.  The users MUST be able to claim the rewards once distributed.

6.  The system SHOULD support rewards in the form of fNFTs.

### 4.3. PICA Stakers

1.  System MUST allow staking of PICA. This must be implemented through
    the fNFT mechanism with multiple time period unlocks being possible
    for users.

2.  The system MUST accumulate the rewards share for PICA holders who
    stake PICA token, out of the PICA supply allocated for them.

3.  The system MUST support accumulating any token rewards other than
    PICA for PICA stakers.

4.  The system must support rewards being distributed on granular
    basis - e.g every 6 or 12 hours.

5.  The users MUST be able to claim the rewards once distributed.

6.  The system SHOULD support rewards in the form of fNFTs.

### 4.4. Pablo Governance

1.  Governance MUST be able to set the PBLO token reward allocation.

2.  Governance MUST be able to set the Pablo LP reward proportion for
    each Pablo LP token(i.e Pool) out of PBLO or other token reward
    allocation. This is to incentivize providing liquidity to required
    pools as decided by governance.

3.  Governance MUST be able to adjust the PBLO reward rate(eg: daily)
    based on the incentivization strategy.

4.  Pablo pool protocol fees(for rewarding protocol stakers) SHOULD be
    configurable as a percentage of the pool owner fee.

### 4.5. PICA Governance

1.  Governance MUST be able to set the PICA token reward allocation.

2.  Governance MUST be able to adjust the PICA reward rate based on the
    incentivization strategy.

### 4.6. Technical Requirements

1.  The system MUST allow accumulation and mapping of rewards shares of
    multiple assets types(Eg: PBLO, KSM) to staked position(fNFT) type
    defined by another asset type(eg: PICA).

2.  The system MUST support transfer of rewards using staking-rewards
    pallet to necessary fNFT types.

3.  The system SHOULD support converting a reward accumulated in one
    asset type to another based on a preferred reward asset type
    configuration. Eg: Given a reward accumulated is in Acala it should
    be able to convert that to one of PBLO or PICA using the Pablo DEX
    pools.

    -   This is to handle cases where a Pablo pool fees are in a
        different asset type than what is preferred.

### 4.7. Financial NFT Requirements

1.  Each staked position MUST be represented as a
    [fNFT](https://github.com/ComposableFi/composable/blob/main/rfcs/0006-financial-nft.md).

2.  Owning a PBLO staked position fNFT(xPBLO) MUST allow voting for
    protocol governance based on the xPBLO granted.

3.  Each staked position plus its rewards MUST be transferable by
    transferring the ownership of its NFT including the voting rights.

## 5. Method

### 5.1. System Overview

<img src="0005-pablo-distribution-assets/images/images/pablo-distribution-overview.png" width="977" height="807" alt="pablo distribution overview" />

TODO: What to do for part of protocol fees that should be transferred to
treasury eventually as treasury does not stake it’s PBLO?

### 5.2. Pallet-Pablo

In order to 1. support LP staking 2. LP trading fee distribution and 3.
PBLO staking reward using trading fees, following changes are proposed
for
[Pallet-Pablo](https://github.com/ComposableFi/composable/tree/main/code/parachain/frame/pablo).

#### 5.2.1. FeeConfig

Each pool in Pablo defines a fee percentage to be charged for each
trade.Except for LBPs other pools also define an owner fee that is a
percentage out of the main trading fee. The `FeeConfig` is a new
abstraction over all fees that could be charged on a pool to allow for
extension. At this time a 100% of the owner fee should be defined as a
new field `protocol_fee`.

[FeeConfig](https://github.com/ComposableFi/composable/blob/abf9b87c57856b4e83aac66eaca2734bd2d99044/code/parachain/frame/composable-traits/src/dex.rs#L189)

Given this,

    fee = // calculation depends on the pool type: based on the fee_rate
    owner_fee = fee * owner_fee_rate * (1 - protocol_fee_rate);
    protocol_fee = owner_fee * protocol_fee_rate;

For all pools launched at the Picasso launch following values would be
set for these configs

    owner_fee_rate = 20%
    protocol_fee_rate = 100% // all owner fees goes to composable to be distributed as rewards

#### 5.2.2. LP Trading Fee Distribution

LPs trading fees are calculated and kept as part of the pool liquidity
in Pablo. When LPs remove liquidity from the pool the trading fees are
automatically redeemed according their pool LP ratio, check
[reference](https://hackmd.io/@HaydenAdams/HJ9jLsfTz#Fee-Structure).
This results in trading fee share being diluted overtime for smaller
pools as follows.


After <img src="0005-pablo-distribution-assets/images/stem-55a049b8f161ae7cfeb0197d75aff967.png" width="9" height="6" alt="stem 55a049b8f161ae7cfeb0197d75aff967" /></span>
trades and <img src="0005-pablo-distribution-assets/images/stem-0e51a2dede42189d77627c4d742822c3.png" width="13" height="6" alt="stem 0e51a2dede42189d77627c4d742822c3" /></span>
liquidity additions,

trading fees <img src="0005-pablo-distribution-assets/images/stem-82f81e776a24846a08157aa3f917012b.png" width="45" height="12" alt="stem 82f81e776a24846a08157aa3f917012b" /></span>

total liquidity <img src="0005-pablo-distribution-assets/images/stem-0f1df372dc50dc67fc225a13b75dd233.png" width="45" height="12" alt="stem 0f1df372dc50dc67fc225a13b75dd233" /></span>

fees and liquidity returned for an LP amount <img src="0005-pablo-distribution-assets/images/stem-2daffc703b015a8c1fc11715b5e9a27d.png" width="142" height="19" alt="stem 2daffc703b015a8c1fc11715b5e9a27d" /></span>

<img src="0005-pablo-distribution-assets/images/stem-35912508e8bf41c1a7f94b93abcec3aa.png" width="98" height="19" alt="stem 35912508e8bf41c1a7f94b93abcec3aa" /></span>

trading fees received <img src="0005-pablo-distribution-assets/images/stem-acbc3160f2b6a5977e6ac719418e0581.png" width="106" height="19" alt="stem acbc3160f2b6a5977e6ac719418e0581" /></span>

"When pool size <img src="0005-pablo-distribution-assets/images/stem-c8165429df4fe2a9cc08c1a6949ead7c.png" width="30" height="12" alt="stem c8165429df4fe2a9cc08c1a6949ead7c" /></span>
increases the amount of trading fees received <img src="0005-pablo-distribution-assets/images/stem-332cc365a4987aacce0ead01b8bdcc0b.png" width="9" height="6" alt="stem 332cc365a4987aacce0ead01b8bdcc0b" /></span>
reduces for a particular LP position.

For large pool sizes of <img src="0005-pablo-distribution-assets/images/stem-c8165429df4fe2a9cc08c1a6949ead7c.png" width="30" height="12" alt="stem c8165429df4fe2a9cc08c1a6949ead7c" /></span>
(steady state) this effect is negligible, hence it’s a good enough
strategy to distribute fees.

But if required this effect can be negated by increasing the trading fee
by a <img src="0005-pablo-distribution-assets/images/stem-e64be84a4eef601683d61de156018075.png" width="24" height="10" alt="stem e64be84a4eef601683d61de156018075" /></span>
while at the same time subtracting it from the total fees paid out
already to liquidity providers. Refer [Trading Fee Inflation to Avoid
Dilution of LPs](#appendix-a-trading-fee-inflation-to-avoid-dilution-of-lps).

#### 5.2.3. PBLO Staker Pool Creation

When creating new Pablo pool, the creator should have option to create a
PBLO staking pool. This newly created staking pool will receive rewards
from trading fees from Pablo pool as mention in section 5.2.4

#### 5.2.4. PBLO Staker Trading Fee Distribution

This is the reward a `PBLO` staker receives from the trading fees of
Pablo pools. It is equal to the protocol fee charged on Pablo pools.
This can be accomplished by calling the already existing
`StakingReward.transfer_reward` interface as follows. According to
product there is also a need to convert whatever the fee asset in to
PBLO to create a demand/additional value for PBLO.

<img src="0005-pablo-distribution-assets/images/images/pablo-fNFT-pblo-staking-fee-distro.png" width="510" height="346" alt="pablo fNFT pblo staking fee distro" />

### 5.3. Pallet Staking Rewards - LP/PICA/PBLO/Other Token Staking Reward Pools

This section covers how the staking rewards are distributed using the
[staking rewards
pallet](https://github.com/ComposableFi/composable/tree/main/code/parachain/frame/staking-rewards).

#### 5.3.1. Analysis of Reward Calculations

In order to create the necessary reward pool as well as the rewarding
rate for stakers the following model can be used. It tries to address
the following constraints,

1.  Allow <span id="rate">specification of the reward rate for a
    pool</span> (even setting a dynamically changing rate)

2.  Allow addition of new stakers at anytime to a pool, start earning
    immediate rewards

3.  Allow more realtime calculation of rewards on-demand for a given
    pool for a given user.

4.  Allow shorter reward pool calculation epoch with the use of the
    reward rate.

5.  Allow expansion of rewards pools realtime.

6.  Allow extending of staked position in time and amount.

7.  Allow splitting of staked position into smaller positions.

8.  \[Postponed\] Allow compounding of staked position when the rewarded
    asset is the same as staked. Not handled at the moment. Though it is
    possible for users to just re-stake their earned assets.

To analyze the requirement fully, let’s define the following terms for a
given staking reward pool,

Pre-defined reward rate (say per second) <img src="0005-pablo-distribution-assets/images/stem-6fb32a8803a6d58cd54908033a2556f9.png" width="23" height="6" alt="stem 6fb32a8803a6d58cd54908033a2556f9" /></span>

Pre-defined reward calculation epoch in seconds <img src="0005-pablo-distribution-assets/images/stem-6184b58307a1dc90934a6a7051a42ceb.png" width="22" height="8" alt="stem 6184b58307a1dc90934a6a7051a42ceb" /></span>

Reward per calculation epoch <img src="0005-pablo-distribution-assets/images/stem-b219ff7e7a0df744c99c2e11229a1ded.png" width="33" height="8" alt="stem b219ff7e7a0df744c99c2e11229a1ded" /></span>

Previous total reward pool before the current epoch <img src="0005-pablo-distribution-assets/images/stem-53fadade13e71b863963af9a23b28b71.png" width="25" height="8" alt="stem 53fadade13e71b863963af9a23b28b71" /></span>

Assuming there is a per epoch calculation which adds to the pool, the
total reward pool for the current epoch,

<img src="0005-pablo-distribution-assets/images/stem-667bfb2c3da043fcfff3288c44c1cc6e.png" width="103" height="10" alt="stem 667bfb2c3da043fcfff3288c44c1cc6e" /></span>

Reward pool shares for <img src="0005-pablo-distribution-assets/images/stem-55a049b8f161ae7cfeb0197d75aff967.png" width="9" height="6" alt="stem 55a049b8f161ae7cfeb0197d75aff967" /></span>
stakers,

<img src="0005-pablo-distribution-assets/images/stem-df3438a6dae343911942f03a3f3e1150.png" width="52" height="12" alt="stem df3438a6dae343911942f03a3f3e1150" /></span>

Where <img src="0005-pablo-distribution-assets/images/stem-39e8c7852cdbd74b28d331353778e128.png" width="21" height="9" alt="stem 39e8c7852cdbd74b28d331353778e128" /></span>
staker share is <img src="0005-pablo-distribution-assets/images/stem-aabe1517ce1102595512b736cbf264bb.png" width="14" height="7" alt="stem aabe1517ce1102595512b736cbf264bb" /></span>

Existing <img src="0005-pablo-distribution-assets/images/stem-39e8c7852cdbd74b28d331353778e128.png" width="21" height="9" alt="stem 39e8c7852cdbd74b28d331353778e128" /></span>
staker reward,

<img src="0005-pablo-distribution-assets/images/stem-e1359ae7d0fae29ebf9e42efcaa5536e.png" width="111" height="18" alt="stem e1359ae7d0fae29ebf9e42efcaa5536e" /></span>

##### 5.3.1.1. When adding a new staker <span class="image"><img src="0005-pablo-distribution-assets/images/stem-0b46f732c83c0e66067b0e50c2156089.png" width="29" height="8" alt="stem 0b46f732c83c0e66067b0e50c2156089" /></span>, existing stakers(<span class="image"><img src="0005-pablo-distribution-assets/images/stem-55a049b8f161ae7cfeb0197d75aff967.png" width="9" height="6" alt="stem 55a049b8f161ae7cfeb0197d75aff967" /></span>) reward would be,

<img src="0005-pablo-distribution-assets/images/stem-569c4bf984a23f18046277fd561e89a3.png" width="126" height="20" alt="stem 569c4bf984a23f18046277fd561e89a3" /></span>

As this is less than what is expected above, an adjustment <img src="0005-pablo-distribution-assets/images/stem-40ae34b20ee5f0d16c68d77473e0be24.png" width="19" height="9" alt="stem 40ae34b20ee5f0d16c68d77473e0be24" /></span>
to total reward pool can be made to allow realtime reward calculations,

<img src="0005-pablo-distribution-assets/images/stem-8e958a64c877dcda40b652878c6c6768.png" width="119" height="20" alt="stem 8e958a64c877dcda40b652878c6c6768" /></span>

<img src="0005-pablo-distribution-assets/images/stem-38d917fea7c6a7a47eb1aa77edd4da97.png" width="169" height="20" alt="stem 38d917fea7c6a7a47eb1aa77edd4da97" /></span>

<img src="0005-pablo-distribution-assets/images/stem-5fcfbc0bc69ee8b8f356ce2bbfb42002.png" width="190" height="22" alt="stem 5fcfbc0bc69ee8b8f356ce2bbfb42002" /></span>

<img src="0005-pablo-distribution-assets/images/stem-e1359ae7d0fae29ebf9e42efcaa5536e.png" width="111" height="18" alt="stem e1359ae7d0fae29ebf9e42efcaa5536e" /></span>

**Therefore, the existing staker receives the same reward as before**

To compensate for this new adjustment, a reduction <img src="0005-pablo-distribution-assets/images/stem-7c4ec4f9c189cb8f3edb39740e43c33f.png" width="16" height="10" alt="stem 7c4ec4f9c189cb8f3edb39740e43c33f" /></span>
(equal to <img src="0005-pablo-distribution-assets/images/stem-40ae34b20ee5f0d16c68d77473e0be24.png" width="19" height="9" alt="stem 40ae34b20ee5f0d16c68d77473e0be24" /></span>)
of reward for each staker needs to be tracked,

<img src="0005-pablo-distribution-assets/images/stem-828ec270409cb6ff5cfc583587d0eae9.png" width="142" height="18" alt="stem 828ec270409cb6ff5cfc583587d0eae9" /></span>
← (1)

In general,

<img src="0005-pablo-distribution-assets/images/stem-0f2f030a4f8a3c172e968af2768a3ec8.png" width="349" height="11" alt="stem 0f2f030a4f8a3c172e968af2768a3ec8" /></span>

##### 5.3.1.2. When removing a staker(Claim/Unstake) from the pool the above addition step has to be reverted

The n+1 stakers claim <img src="0005-pablo-distribution-assets/images/stem-ae267f55aab2b9494bdb7556432e63b6.png" width="31" height="8" alt="stem ae267f55aab2b9494bdb7556432e63b6" />
is given by (1). With the reward rate based rewards added in after time
<img src="0005-pablo-distribution-assets/images/stem-4ac53ea916c290c6cbd381dd25a30dd7.png" width="16" height="9" alt="stem 4ac53ea916c290c6cbd381dd25a30dd7" />
and replacing <img src="0005-pablo-distribution-assets/images/stem-a11a5700a172e5aa22cd3b0d99686ed1.png" width="95" height="11" alt="stem a11a5700a172e5aa22cd3b0d99686ed1" />
and substituting <img src="0005-pablo-distribution-assets/images/stem-7c4ec4f9c189cb8f3edb39740e43c33f.png" width="16" height="10" alt="stem 7c4ec4f9c189cb8f3edb39740e43c33f" />,

<img src="0005-pablo-distribution-assets/images/stem-af68a152e83453497a7fa996704fda6e.png" width="327" height="23" alt="stem af68a152e83453497a7fa996704fda6e" />
<br/>
<img src="0005-pablo-distribution-assets/images/stem-5543ef4608a9962063915b6081c7087a.png" width="119" height="23" alt="stem 5543ef4608a9962063915b6081c7087a" />

Therefore, the adjustment made above for the total reward pool works as
expected for claims for the all the stakers. As this relationship holds
for any number of stakers the total reward pool need not be adjusted
when removing a staker.

##### 5.3.1.3. When adding a new reward to the pool the calculations remain the same other than increasing the reward pool as follows,

<img src="0005-pablo-distribution-assets/images/stem-26132ac9393fe54200c2208dc9244ea4.png" width="160" height="11" alt="stem 26132ac9393fe54200c2208dc9244ea4" /></span>

Since already claimed rewards(<img src="0005-pablo-distribution-assets/images/stem-7c4ec4f9c189cb8f3edb39740e43c33f.png" width="16" height="10" alt="stem 7c4ec4f9c189cb8f3edb39740e43c33f" /></span>)
are tracked for each staker, they can always claim the new reward share
from <img src="0005-pablo-distribution-assets/images/stem-32efe856de4078991a47242cc1d89349.png" width="43" height="11" alt="stem 32efe856de4078991a47242cc1d89349" /></span>
later.

##### 5.3.1.4. When extending an existing position

Extension of an existing staker position can be treated in the same way
as adding a new staker as the following relationship holds with the new
stake <img src="0005-pablo-distribution-assets/images/stem-9849cee8ec3e29bf6d2ea80a64d995dd.png" width="23" height="10" alt="stem 9849cee8ec3e29bf6d2ea80a64d995dd" /></span>
and the corresponding inflation <img src="0005-pablo-distribution-assets/images/stem-e7d319c4dcb739d8e91edd37454e20e8.png" width="25" height="10" alt="stem e7d319c4dcb739d8e91edd37454e20e8" /></span>,

new staker to add <img src="0005-pablo-distribution-assets/images/stem-1b75a50a55357d9a7a8d3ecbb06df470.png" width="180" height="20" alt="stem 1b75a50a55357d9a7a8d3ecbb06df470" /></span>
← (2)

Now with (1) + (2),

<img src="0005-pablo-distribution-assets/images/stem-6a000622e842e98de57502915826da7b.png" width="245" height="20" alt="stem 6a000622e842e98de57502915826da7b" /></span>

Therefore, same computation as before with <img src="0005-pablo-distribution-assets/images/stem-9849cee8ec3e29bf6d2ea80a64d995dd.png" width="23" height="10" alt="stem 9849cee8ec3e29bf6d2ea80a64d995dd" /></span>
number of shares added to the staker position works as expected.

##### 5.3.1.5. When splitting an existing position

As the total reward pool is not affected the splitting is just creating
a new position using some ratio. If the ratio is <img src="0005-pablo-distribution-assets/images/stem-603de94498e154610e3066ec63603017.png" width="25" height="9" alt="stem 603de94498e154610e3066ec63603017" /></span>
From (1)

First position <img src="0005-pablo-distribution-assets/images/stem-ed2c175456fa8dcae30f92f61b3694ff.png" width="151" height="20" alt="stem ed2c175456fa8dcae30f92f61b3694ff" /></span>

Second position <img src="0005-pablo-distribution-assets/images/stem-706fbeec167ddb5aeb84ef0c7bde2f57.png" width="192" height="20" alt="stem 706fbeec167ddb5aeb84ef0c7bde2f57" /></span>

Summing these positions would give the original position(equation 1) as
the ratio terms cancel out.

As this method uses a reward pooling based approach to calculate the
rewards for each staker out of it on-demand, rest of the document refers
to this as the "reward pooling(**RP**) based approach".

#### 5.3.2. Data Structures

Staking rewards pallet already uses the following data structure
representing a staking position,

[Stake](https://github.com/ComposableFi/composable/blob/abf9b87c57856b4e83aac66eaca2734bd2d99044/code/parachain/frame/composable-traits/src/staking/mod.rs#L210)

Which is referred to in the algorithms in the following sections.

Now in order to allow redeeming the above staking position, following
data structures is to be tracked in the staking rewards pallet,

[Reward](https://github.com/ComposableFi/composable/blob/abf9b87c57856b4e83aac66eaca2734bd2d99044/code/parachain/frame/composable-traits/src/staking/mod.rs#L20)

[RewardPool](https://github.com/ComposableFi/composable/blob/abf9b87c57856b4e83aac66eaca2734bd2d99044/code/parachain/frame/composable-traits/src/staking/mod.rs#L113)

Following sections describe the algorithms for various operations on the
rewards pool based on these data structures.

#### 5.3.3. Staking

<img src="0005-pablo-distribution-assets/images/images/staking.png" width="684" height="873" alt="staking" />

#### 5.3.4. Extend Position

<img src="0005-pablo-distribution-assets/images/images/extend-position.png" width="684" height="994" alt="extend position" />

#### 5.3.5. Split Position

<img src="0005-pablo-distribution-assets/images/images/split-position.png" width="426" height="344" alt="split position" />

#### 5.3.6. Unstake

<img src="0005-pablo-distribution-assets/images/images/unstake.png" width="500" height="695" alt="claim" />

#### 5.3.7. Reward Pool Governance

##### 5.3.7.1. Update Reward Allocation Per Pool

Each reward pool would have its own reward pot account.

-   Lock the assets in the pool account so that funds can be claimed
    only when unlocked.

-   Reward accumulation logic would just release funds from the pool
    account according to the reward rate.

-   In order to add funds to the pool account, an extrinsic is needed as
    follows:

    -   Input: rewardPoolId, AssetId, Balance

        For governance proposals, one can query storage to get the
        reward pool ID and create a proposal to call the above
        extrinsic.

<img src="0005-pablo-distribution-assets/images/images/transfer-funds-extrinsic.png" width="431" height="302" alt="transfer funds extrinsic" />

##### 5.3.7.2. Update Reward Pool

<img src="0005-pablo-distribution-assets/images/images/update-reward-pool.png" width="497" height="647" alt="update reward pool" />

#### 5.3.8. RewardAccumulationHook

Following algorithm should be part of the block hook in the pallet.

<img src="0005-pablo-distribution-assets/images/images/staking-rewards-reward-accumulation-hook.png" width="509" height="501" alt="staking rewards reward accumulation hook" />

#### 5.3.9. Claim

<img src="0005-pablo-distribution-assets/images/images/claim.png" width="684" height="642" alt="claim" />

## 6. Implementation

### 6.1. Pallet Pablo: LP Fee + Staking Changes

-   ❏ Implement [FeeConfig](#521-feeconfig) on pallet-pablo across all 3
    types of pools.

-   ❏ Implement [PBLO Staker Trading Fee
    Distribution](#523-pblo-staker-trading-fee-distribution).

### 6.2. Pallet Staking Rewards: PICA/PBLO Staking Related Changes

-   ❏ Implement [RewardAccumulationHook](#538-rewardaccumulationhook).

## Appendix A: Trading Fee Inflation to Avoid Dilution of LPs

New trading fee <img src="0005-pablo-distribution-assets/images/stem-88ffccf5d7e5534d6a1c8255ea6f8491.png" width="203" height="19" alt="stem 88ffccf5d7e5534d6a1c8255ea6f8491" /></span>


For <img src="0005-pablo-distribution-assets/images/stem-64bf6f450600e539b13faa38cda05cdd.png" width="20" height="9" alt="stem 64bf6f450600e539b13faa38cda05cdd" /></span>
liquidity provider,

<img src="0005-pablo-distribution-assets/images/stem-361b0e678ae955263b9781486d18e96a.png" width="120" height="22" alt="stem 361b0e678ae955263b9781486d18e96a" /></span>

<img src="0005-pablo-distribution-assets/images/stem-b7581568f93412c6c936184a45f8ac21.png" width="324" height="23" alt="stem b7581568f93412c6c936184a45f8ac21" /></span>

<img src="0005-pablo-distribution-assets/images/stem-baea3c4f49ab8e93ff2c4cd2067b5364.png" width="78" height="20" alt="stem baea3c4f49ab8e93ff2c4cd2067b5364" /></span>

With this adjusted value all later additions to LP shares have been
negated when receiving fees for earlier LPs.

## Appendix B: Fee Distribution Q&A

Based on the current setup following questions arise when deciding on
the distribution of these fees to relevant liquidity providers, owners
and stakers.

1.  A Protocol Fee for all pools in Pablo (or even protocol pallets
    other than Pablo)?

    Does it make sense to define a protocol fee percentage on top of the
    pool owner fees of the pools so that the protocol fee can be used as
    the pot out of which the stakers are rewarded? Initially the
    Protocol Fee = Pool Owner Fee as the pools are owned by Composable.
    Assumption here is that the stakers would indeed still get a reward
    out of third party created pool fees.

    **Comment:** While having a protocol funding mechanism is valuable,
    initially the protocol fees should zero or minimal.

2.  How does the system reward PICA stakers? Wouldn’t the Pablo protocol
    needs some parameter to define how much of its swap fee or protocol
    fee as referred to above would go to PICA holders? Or do we assume
    that PICA stakers do not get a reward out of the Pablo pool fees?

    1.  If Pablo does reward PICA stakers, the system might need a
        common interface that directs those funds out of Pablo.

    2.  If Pablo does reward PICA stakers, the system might need to have
        a treasury parameter that defines the percentage that goes out
        to PICA holders that can be adjusted overtime.

        **Comment:** PICA stakers would not be rewarded from the Pablo
        fees. PICA stakers are rewarded in newly minted PICA(or PBLO
        later), Mechanism to transfer the PICA tokens for stakers does
        not exist, need to be built.

3.  Does it make sense to define a Pool Owner Fee(Protocol Fee as
    referred to above) for LBPs that goes out to Pablo holders reward
    pool?

**Comment:** Pool fees could be swapped to PBLO token before
distributing to fNFT holders unless those fees are in some pre-defined
set of currencies(eg: KSM, DOT), which creates a demand for PBLO since
the system is buying back PBLO. But for this there should be a market
for PBLO/the other token that is being earned as fees.

**Comment:** LP fees can be distributed based on the fNFT. Minting the
fNFT at the time of LP event might make sense. i.e fNFT represents the
LP position on the pool as well as the rewards position for PBLO tokens
for LPs.

Last updated 2023-01-17 01:45:40 +0200
