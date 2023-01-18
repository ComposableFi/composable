Design Proposal: Oracle Rewards & General Inflation
===================================================

Table of Contents

-   [1. Abstract](#_abstract)
-   [2. Background](#_background)
-   [3. Requirements](#_requirements)
-   [4. Method](#_method)
    -   [4.1. Governance Adjustable Parameters for Deciding Oracle
        Rewards](#_governance_adjustable_parameters_for_deciding_oracle_rewards)
    -   [4.2. Calculating per Asset Type Oracle Reward per
        Block](#_calculating_per_asset_type_oracle_reward_per_block)

## 1. Abstract

This proposal lays out agreements on [Apollo
oracle](https://docs.composable.finance/products/the-picasso-parachain/the-picasso-tech-stack/apollo)
rewards based on discussions for the task [Control Oracle
inflation](https://app.clickup.com/t/27xkb7w).

## 2. Background

Composable parachains Picasso in Kusama (now), Composable in Polkadot
(later) depends on external price Oracles(Apollo) to provide reliable
prices for chain internal components such as DeFi pallets and smart
contracts. In order to incentivize the provision of accurate, regular
assets for required asset class Composable intends to reward the Oracles
regularly based on a tokenomics model.

## 3. Requirements

1.  An Oracle provider MUST be able to earn an economically viable
    reward for submitting a price for a given pre-configured asset type.

2.  Governance MUST be able to adjust the possible reward based on the
    economic factors such as,

    1.  Oracle provider viability based on their infrastructure costs.

    2.  Competition and demand for the provision of Oracle services.

    3.  Opportunity cost of capital in staking for Oracles.

3.  Governance MUST be able to set the maximum amount of rewards earned
    by all oracles across all asset types.

4.  The system MUST NOT run out of funds for rewarding Oracles at any
    point in time as this is a critical service for all other DeFi
    protocols within the chain. This sets a hard requirement on being
    able to inflate the supply for Oracle rewards.

## 4. Method

To formalize and make the analysis easier one can follow the model
established in the following sections for Oracle rewards.

### 4.1. Governance Adjustable Parameters for Deciding Oracle Rewards

An adjustable parameter is needed to adjust rewards for Oracles as well
as managed the inflation. Following a similar relationship outlined in
[Polkadot Token
Economics](https://research.web3.foundation/en/latest/polkadot/overview/2-token-economics.html#inflation-model),
one can come up with the following relationship for the total
inflation(I) rate in runtime based on the current implementation,

<img src="images/stem-e29aba55d1303dd5b79b86dcc2f686e2.png" width="186" height="12" alt="stem e29aba55d1303dd5b79b86dcc2f686e2" />
- (1)

Here,

<img src="images/stem-504341fca0c81a36b768da13098c0cd8.png" width="70" height="12" alt="stem 504341fca0c81a36b768da13098c0cd8" />
Any amount that is minted through governance to fund the treasury.
Generally to be kept close to zero for all runtimes.

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr class="odd">
<td><div class="title">
Note
</div></td>
<td>Above equation ignores any slashing mechanics as the slashed amount is not burnt in the current implementation</td>
</tr>
</tbody>
</table>

At the moment the following code decides the transaction fee
distribution ratio,

    /// Logic for the author to get a portion of fees.
    pub struct ToStakingPot<R>(sp_std::marker::PhantomData<R>);
    impl<R> OnUnbalanced<NegativeImbalance<R>> for ToStakingPot<R>
    where
        R: balances::Config
            + collator_selection::Config
            + treasury::Config<Currency = balances::Pallet<R>>,
        <R as frame_system::Config>::AccountId: From<polkadot_primitives::v2::AccountId>,
        <R as frame_system::Config>::AccountId: Into<polkadot_primitives::v2::AccountId>,
        <R as frame_system::Config>::RuntimeEvent: From<balances::Event<R>>,
        <R as balances::Config>::Balance: From<u128>,
    {
        fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
            // Collator's get half the fees
            let (to_collators, half) = amount.ration(50, 50);
            // 30% gets burned 20% to treasury
            let (_pre_burn, to_treasury) = half.ration(30, 20);

            let staking_pot = <collator_selection::Pallet<R>>::account_id();
            <balances::Pallet<R>>::resolve_creating(&staking_pot, to_collators);
            <treasury::Pallet<R> as OnUnbalanced<_>>::on_unbalanced(to_treasury);
        }
    }

Therefore,

<img src="images/stem-dd2b4f723f2f41a06af9e0033c88111a.png" width="88" height="11" alt="stem dd2b4f723f2f41a06af9e0033c88111a" />
30% of the transaction fees. It is recommended here that this percentage
be defined as a governance parameter(<img src="images/stem-190083ef7a1625fbc75f243cffb9c96d.png" width="7" height="9" alt="stem 190083ef7a1625fbc75f243cffb9c96d" />)
to serve as possible mechanism to balance inflation.

Inflation for rewarding Oracles, <img src="images/stem-d5a7eb822709b9a7c5191cfb6efc422f.png" width="33" height="11" alt="stem d5a7eb822709b9a7c5191cfb6efc422f" />
is the main topic of concern here, and it can be defined to be governed
as an annual ratio(<img src="images/stem-8cd34385ed61aca950a6b06d09fb50ac.png" width="8" height="6" alt="stem 8cd34385ed61aca950a6b06d09fb50ac" />)
of the total issuance of the base currency(<img src="images/stem-2f118ee06d05f3c2d98361d9c30e38ce.png" width="10" height="8" alt="stem 2f118ee06d05f3c2d98361d9c30e38ce" />)
of the runtime to serve as a clear APY indicator for Oracles.

With this taken into account the final equation for <img src="images/stem-21fd4e8eecd6bdf1a4d3d6bd1fb8d733.png" width="6" height="8" alt="stem 21fd4e8eecd6bdf1a4d3d6bd1fb8d733" />
becomes,

<img src="images/stem-c8330df2487664a0e6b19a2a58998da6.png" width="173" height="13" alt="stem c8330df2487664a0e6b19a2a58998da6" />
- (2)

Here for Oracle economics to work,

<img src="images/stem-c668a522da1ff1825f72755051f25110.png" width="91" height="11" alt="stem c668a522da1ff1825f72755051f25110" />
- (3)

<img src="images/stem-e2b731bd4147e0d1167a3e0e6ba025e9.png" width="39" height="9" alt="stem e2b731bd4147e0d1167a3e0e6ba025e9" />
expected average annual cost of running an Oracle for a given year.

<img src="images/stem-8d03e348339c7c4e027620b5ed3a755f.png" width="68" height="11" alt="stem 8d03e348339c7c4e027620b5ed3a755f" />
ideal number of oracles.

So the governance of Oracle reward inflation can be defined as adjusting
<img src="images/stem-8cd34385ed61aca950a6b06d09fb50ac.png" width="8" height="6" alt="stem 8cd34385ed61aca950a6b06d09fb50ac" />
as,

<img src="images/stem-5fd7d52ac02bdc173abd9a84d608f026.png" width="71" height="20" alt="stem 5fd7d52ac02bdc173abd9a84d608f026" />
- (4)

It also follows from (3) that annual profit(<img src="images/stem-df5a289587a2f0247a5b97c1e8ac58ca.png" width="10" height="8" alt="stem df5a289587a2f0247a5b97c1e8ac58ca" />)
per Oracle is,

<img src="images/stem-31edf916f4bbe7e2e1ac2174f668831d.png" width="69" height="17" alt="stem 31edf916f4bbe7e2e1ac2174f668831d" />

With <img src="images/stem-f9d81717cb1703249842ccd6bba262fa.png" width="40" height="8" alt="stem f9d81717cb1703249842ccd6bba262fa" />
the instantaneous number of Oracles replacing <img src="images/stem-6fd4b3f62fbac0c0d71ea15692bc5287.png" width="39" height="11" alt="stem 6fd4b3f62fbac0c0d71ea15692bc5287" />.
With <img src="images/stem-8cd34385ed61aca950a6b06d09fb50ac.png" width="8" height="6" alt="stem 8cd34385ed61aca950a6b06d09fb50ac" />
varying based on governance and <img src="images/stem-cbfb1b2a33b28eab8a3e59464768e810.png" width="11" height="8" alt="stem cbfb1b2a33b28eab8a3e59464768e810" />
varying when Oracles join and leave following is the graph visualization
of the equation (Note <img src="images/stem-2f118ee06d05f3c2d98361d9c30e38ce.png" width="10" height="8" alt="stem 2f118ee06d05f3c2d98361d9c30e38ce" />
as it’s annual and <img src="images/stem-9b325b9e31e85137d1de765f43c0f8bc.png" width="10" height="9" alt="stem 9b325b9e31e85137d1de765f43c0f8bc" />
as it’s mostly infra cost are assumed largely constant).

Assuming <img src="images/stem-6b717b63effa3dd420599925b5e4748e.png" width="51" height="9" alt="stem 6b717b63effa3dd420599925b5e4748e" />,
<img src="images/stem-30e08c9fbd279df39e6310c3e08d926b.png" width="50" height="10" alt="stem 30e08c9fbd279df39e6310c3e08d926b" />
and <img src="images/stem-b37e1d79efb233c502e0aa9ce1fa12d8.png" width="46" height="10" alt="stem b37e1d79efb233c502e0aa9ce1fa12d8" />,
the graph becomes,

![p vs x](images/p-vs-x.png)

Here <img src="images/stem-df5a289587a2f0247a5b97c1e8ac58ca.png" width="10" height="8" alt="stem df5a289587a2f0247a5b97c1e8ac58ca" />
goes to zero as <img src="images/stem-cbfb1b2a33b28eab8a3e59464768e810.png" width="11" height="8" alt="stem cbfb1b2a33b28eab8a3e59464768e810" />
approaches 10, making 10 the ideal number of Oracles at the limit. But
this can not be considered the ideal as the Oracles still need to make a
profit for them to be incentivized.

Therefore, from (4) we can deduce the following equation to adjust <img src="images/stem-8cd34385ed61aca950a6b06d09fb50ac.png" width="8" height="6" alt="stem 8cd34385ed61aca950a6b06d09fb50ac" />
as,

<img src="images/stem-de622ccafa0d27daf3e2c6b33be3e4d0.png" width="105" height="22" alt="stem de622ccafa0d27daf3e2c6b33be3e4d0" />
- (5)

Here <img src="images/stem-9b325b9e31e85137d1de765f43c0f8bc.png" width="10" height="9" alt="stem 9b325b9e31e85137d1de765f43c0f8bc" />(cost
per Oracle) as well as <img src="images/stem-af9f88546f39f2e65ce188f544720b36.png" width="74" height="12" alt="stem af9f88546f39f2e65ce188f544720b36" />(number
of Oracles to support) can be considered governance inputs to the
rewarding model and based on that the model can calculate the necessary
inflation <img src="images/stem-8cd34385ed61aca950a6b06d09fb50ac.png" width="8" height="6" alt="stem 8cd34385ed61aca950a6b06d09fb50ac" />
as it already can track the total issuance for the past year <img src="images/stem-2f118ee06d05f3c2d98361d9c30e38ce.png" width="10" height="8" alt="stem 2f118ee06d05f3c2d98361d9c30e38ce" />.
By adjusting <img src="images/stem-efe14b3862d4b69d10f4f4b1957fc944.png" width="20" height="9" alt="stem efe14b3862d4b69d10f4f4b1957fc944" />
or <img src="images/stem-9b325b9e31e85137d1de765f43c0f8bc.png" width="10" height="9" alt="stem 9b325b9e31e85137d1de765f43c0f8bc" />
governance can make sure that running an Oracle is viable at all
expected conditions such as opportunity cost of capital.

In order to allow for accurate accounting of the inflation while
adjusting it based on the governance requirements, the total amount of
already rewarded inflation(<img src="images/stem-0622dee298640a65c96c5a01aa667471.png" width="33" height="12" alt="stem 0622dee298640a65c96c5a01aa667471" />)
needs to be kept track of in the system. Then the total amount allowed
to be rewarded is,

<img src="images/stem-f2790ace228ad55609a45d52a7a692a2.png" width="125" height="12" alt="stem f2790ace228ad55609a45d52a7a692a2" />
- (6)

System must not allow adjustment of <img src="images/stem-8cd34385ed61aca950a6b06d09fb50ac.png" width="8" height="6" alt="stem 8cd34385ed61aca950a6b06d09fb50ac" />
further if the <img src="images/stem-d5a7eb822709b9a7c5191cfb6efc422f.png" width="33" height="11" alt="stem d5a7eb822709b9a7c5191cfb6efc422f" />
becomes negative or zero as then the Oracles would not get rewarded
further for the year. With (5) and (6) total allowed inflation is,

<img src="images/stem-bd802611cfebc951382b02fbed41bbd1.png" width="194" height="13" alt="stem bd802611cfebc951382b02fbed41bbd1" />
- (7)

Now the algorithm for adjusting <img src="images/stem-8cd34385ed61aca950a6b06d09fb50ac.png" width="8" height="6" alt="stem 8cd34385ed61aca950a6b06d09fb50ac" />
becomes (substituting <img src="images/stem-572722359a515c7cd2c33095abec0368.png" width="101" height="12" alt="stem 572722359a515c7cd2c33095abec0368" />),

<img src="images/images/adjusting-inflation-rate.png" width="578" height="397" alt="adjusting inflation rate" />

### 4.2. Calculating per Asset Type Oracle Reward per Block

Given the total reward that is possible to be rewarded per year, the
system must calculate the reward per Oracle for a given asset type.
Obvious way to do this is to divide the per\_block\_reward by the number
of asset types defined for pricing. But in order the make this a bit
more customizable, the reward per asset type can be defined as a
weighted ratio of the per\_block\_reward. Then the algorithm for
calculating the per asset type reward is as follows,

<img src="images/images/per-asset-type-reward.png" width="697" height="269" alt="per asset type reward" />

Last updated 2022-07-07 05:57:10 +0200
