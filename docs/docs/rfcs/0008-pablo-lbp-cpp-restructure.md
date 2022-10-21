Design Proposal: Pablo Formalization and Restructure
====================================================

Table of Contents

-   [1. Abstract](#1-abstract)
-   [2. Background](#2-background)
-   [3. Requirements](#3-requirements)
-   [4. Method](#4-method)
    -   [4.1. Fee Math Updates](#41-fee-math-updates)
    -   [4.2. Liquidity Provider Token (LPT) Math
        Updates](#42-liquidity-provider-token-lpt-math-updates)
    -   [4.3. Validation: Pool Asset (Pair) Validation Must be at the
        Top Level of `Amm` trait
        Implementation](#43-validation-pool-asset-pair-validation-must-be-at-the-top-level-of-amm-trait-implementation)
    -   [4.4. Validation: the Asset Ratio When Adding
        Liquidity](#44-validation-the-asset-ratio-when-adding-liquidity)
    -   [4.5. Refactoring: `CurrencyPair`
        Usage](#45-refactoring-currencypair-usage)
    -   [4.6. Unit Test Updates](#46-unit-test-updates)
    -   [4.7. Algorithm:
        `constant_product::compute_out_given_in`](#47-algorithm-constant_productcompute_out_given_in)
    -   [4.8. Algorithm:
        `constant_product::compute_in_given_out`](#48-algorithm-constant_productcompute_in_given_out)
    -   [4.9. Algorithm:
        `constant_product::compute_deposit_lp`](#49-algorithm-constant_productcompute_deposit_lp)
    -   [4.10. Algorithm:
        `constant_product::compute_redeemed_for_lp`](#410-algorithm-constant_productcompute_redeemed_for_lp)
    -   [4.11. Algorithm: `Amm::currency_pair` →
        `Amm::assets`](#411-algorithm-ammcurrency_pair--ammassets)
    -   [4.12. Algorithm:
        `Amm::get_exchange_value`](#412-algorithm-ammget_exchange_value)
    -   [4.13. Algorithm: `Amm::exchange` →
        `Amm::swap`](#413-algorithm-ammexchange--ammswap)
    -   [4.14. Algorithm: `Amm::sell`](#414-algorithm-ammsell)
    -   [4.15. Algorithm: `Amm::buy`](#415-algorithm-ammbuy)
    -   [4.16. Algorithm:
        `Amm::add_liquidity`](#416-algorithm-ammadd_liquidity)
    -   [4.17. Algorithm:
        `Amm::remove_liquidity`](#417-algorithm-ammremove_liquidity)
    -   [4.18. Fail Safes](#418-fail-safes)
        -   [4.18.1. At the protocol level, where the entire Pablo
            protocol in multiple pallets is
            affected.](#4181-at-the-protocol-level-where-the-entire-pablo-protocol-in-multiple-pallets-is-affected)
        -   [4.18.2. At the pallet level, where a particular Pablo
            pallet is
            affected.](#4182-at-the-pallet-level-where-a-particular-pablo-pallet-is-affected)
        -   [4.18.3. At the pool type level where all pools of a
            particular pool type like LBP is
            affected.](#4183-at-the-pool-type-level-where-all-pools-of-a-particular-pool-type-like-lbp-is-affected)
        -   [4.18.4. At the individual pool level where a single pool is
            affected](#4184-at-the-individual-pool-level-where-a-single-pool-is-affected)
        -   [4.18.5. At the functionality level where a particular
            functionality for example like "removing liquidity" is
            affected.](#4185-at-the-functionality-level-where-a-particular-functionality-for-example-like-removing-liquidity-is-affected)
-   [5. Implementation](#5-implementation)
    -   [5.1. Stage 1: Cutting Down on Non-useful Parts for
        Launch](#51-stage-1-cutting-down-on-non-useful-parts-for-launch)
    -   [5.2. Stage 2: Consistently Implement the Balancer Based CPP
        Equations](#52-stage-2-consistently-implement-the-balancer-based-cpp-equations)
    -   [5.3. Stage 3: Change/Implement Algorithms for
        CPP](#53-stage-3-changeimplement-algorithms-for-cpp)
-   [6. Quality Assurance](#6-quality-assurance)
-   [7. Audit](#7-audit)
-   [8. Questions](#8-questions)
-   [Appendix A: Proof of Fee for
    "In-given-out"](#appendix-a-proof-of-fee-for-in-given-out)
-   [Appendix B: Proof of Proportional LPT Calculation for Liquidity
    Added in a Single Pool
    Asset](#appendix-b-proof-of-proportional-lpt-calculation-for-liquidity-added-in-a-single-pool-asset)
-   [Appendix C: Proof of Proportional LPT Calculation for Liquidity
    Added in Pool Weight
    Ratio](#appendix-c-proof-of-proportional-lpt-calculation-for-liquidity-added-in-pool-weight-ratio)

## 1. Abstract

This document is a review of current Pablo implementation and proposes
updates to better align the constant product pool(CPP) and liquidity
bootstrapping pool(LBP) code with the [Balancer
protocol](https://docs.balancer.fi/concepts/math/weighted-math) which
those implementations are based on.

## 2. Background

Pablo [constant product pool (analogous to Uniswap)](http://link) and
the liquidity bootstrapping pool implementations are based on the
balancer weighted math that is laid out
[here](https://dev.balancer.fi/resources/pool-math/weighted-math).
However, due to historical evolution of the code the implementations are
right now separate and sometimes seems to be inconsistent. Specially the
fee calculations that does not seem to follow a standard formula
specially when it comes to "in given out" scenario.

## 3. Requirements

1.  Fees calculations on Pablo MUST align with the math for Balancer
    pools.

2.  Liquidity provider token calculations on Pablo MUST align with the
    math for Balancer pools (taking into account weights).

3.  Code MUST be re-organized to unify CPP to avoid divergence and to
    allow easier maintenance.

4.  Remove LBP and StableSwap implementations since they are not needed
    for the MVP.

5.  MUST assess risks and propose fail safes.

In addition to these hard requirements the idea is also to simplify the
code as much as possible to avoid confusion specially regarding the
`CurrencyPair` swaps.

## 4. Method

Following is an overview of the proposed changes to the Pablo CPP and
LBP implementation.

TODO

### 4.1. Fee Math Updates

Given the definition of symbols,

<img src="0008-pablo-lbp-cpp/images/stem-4ebf880807deff5796460f39aea46f80.png" width="16" height="11" alt="stem 4ebf880807deff5796460f39aea46f80" />
: Amount (`A`) of input (`i`) token

<img src="0008-pablo-lbp-cpp/images/stem-4c14314a2056e57fb583cb593c713440.png" width="29" height="10" alt="stem 4c14314a2056e57fb583cb593c713440" />
: Amount of input token sent by the user

<img src="0008-pablo-lbp-cpp/images/stem-8cf31bbe4a0e4717dbe33abd7e8c9c21.png" width="18" height="10" alt="stem 8cf31bbe4a0e4717dbe33abd7e8c9c21" />
: Amount of output token (`o`)

<img src="0008-pablo-lbp-cpp/images/stem-72f4aab7f49593ada1f6b406b90a8a94.png" width="14" height="11" alt="stem 72f4aab7f49593ada1f6b406b90a8a94" />
: Balance (`B`) of input token in the pool

<img src="0008-pablo-lbp-cpp/images/stem-8968395d885ceb605cf51f2f8ab4e3c2.png" width="16" height="10" alt="stem 8968395d885ceb605cf51f2f8ab4e3c2" />
: Balance of output token in the pool

<img src="0008-pablo-lbp-cpp/images/stem-c2a29561d89e139b3c7bffe51570c3ce.png" width="16" height="8" alt="stem c2a29561d89e139b3c7bffe51570c3ce" />
: weight (`w`) of input token in the pool

<img src="0008-pablo-lbp-cpp/images/stem-9f02f0502218eb5335008691e747a970.png" width="18" height="7" alt="stem 9f02f0502218eb5335008691e747a970" />
: weight of output token in the pool

<img src="0008-pablo-lbp-cpp/images/stem-6f9bad7347b91ceebebd3ad7e6f6f2d1.png" width="7" height="6" alt="stem 6f9bad7347b91ceebebd3ad7e6f6f2d1" />
: spot price

<img src="0008-pablo-lbp-cpp/images/stem-190083ef7a1625fbc75f243cffb9c96d.png" width="7" height="9" alt="stem 190083ef7a1625fbc75f243cffb9c96d" />
: total swap fee

[Balancer white paper](https://balancer.fi/whitepaper.pdf) derives the
following formulae for calculating,

1.  Spot price

    <img src="0008-pablo-lbp-cpp/images/stem-926c112dc3fdde348c7b6a70fb4bbf86.png" width="41" height="44" alt="stem 926c112dc3fdde348c7b6a70fb4bbf86" />

    When taking the fee into account the formula is adjusted as,

    <img src="0008-pablo-lbp-cpp/images/stem-c2c466d1e97a99513801d441d5a86f58.png" width="80" height="44" alt="stem c2c466d1e97a99513801d441d5a86f58" />
    ← (1)

2.  Out-given-in

    <img src="0008-pablo-lbp-cpp/images/stem-b6ad4bbfa22553df2ddd9b26aef884ef.png" width="157" height="37" alt="stem b6ad4bbfa22553df2ddd9b26aef884ef" />
    Here as they recommend the application of fee on the "way-in" with
    the following approach,

    <img src="0008-pablo-lbp-cpp/images/stem-4760da3535b96aa8a9d1091a9036f300.png" width="200" height="37" alt="stem 4760da3535b96aa8a9d1091a9036f300" />
    ← (2)

3.  In-given-out

    <img src="0008-pablo-lbp-cpp/images/stem-c6e4980a995a55835b82c87f96700e50.png" width="153" height="37" alt="stem c6e4980a995a55835b82c87f96700e50" />

    Given that the fee should not affect the amount that the user would
    like to receive, One can derive the following formula for the "In"
    amount given the fee (refer [Proof of Fee for
    "In-given-out"](#_proof_of_fee_for_in_given_out)).

    <img src="0008-pablo-lbp-cpp/images/stem-56f7aed19955aa080dfc6dddc087c1db.png" width="188" height="37" alt="stem 56f7aed19955aa080dfc6dddc087c1db" />
    ← (3)

The CPP and LBP implementations can directly use (1), (2) and (3) with
fees included (as opposed to the current implementation where fees are
not part of the core equations). The code at
`code/parachain/frame/composable-maths/src/dex/constant_product.rs` must
be adjusted for this.

This change could be done in stages,

1.  Change the functions in the code to include the fee percentage as an
    input, set the fee as 0 for all uses of those functions.

2.  Adjust the uses of these functions to provide the actual fee
    percentage.

### 4.2. Liquidity Provider Token (LPT) Math Updates

Original balancer protocol [requires the pool weight of a given currency
to be taken into
account](https://metavision-labs.gitbook.io/balancerv2cad/code-and-instructions/balancer_py_edition/weightedpool.py#calc_token_in_given_exact_bpt_out)
when calculating LPT out given the input of a given amount of liquidity
in that currency. Rationale being that providing liquidity the amount of
received is proportional to the movement of pool invariant (value
function). Current LPT math based on [Uniswap
v2](https://uniswap.org/whitepaper.pdf) though accurate for a 50/50
pool, it does not work when the pool weights are different.

Given the additional symbol definitions,

<img src="0008-pablo-lbp-cpp/images/stem-bda53125ba72ee8a17cc536d43e5d471.png" width="36" height="11" alt="stem bda53125ba72ee8a17cc536d43e5d471" />
: LPT tokens issued

<img src="0008-pablo-lbp-cpp/images/stem-a5e57768b3ceca6ca99427b511f61b65.png" width="37" height="13" alt="stem a5e57768b3ceca6ca99427b511f61b65" />
: Existing supply of LPT tokens

<img src="0008-pablo-lbp-cpp/images/stem-e886e70e952a65ae93f5123733379039.png" width="52" height="11" alt="stem e886e70e952a65ae93f5123733379039" />
: Redeemed LPT tokens

<img src="0008-pablo-lbp-cpp/images/stem-8daec2445e7b537498820d34172b49d0.png" width="18" height="11" alt="stem 8daec2445e7b537498820d34172b49d0" />
: Deposit (`D`) of token `k`.

As per the requirement of having differentially weighted pools for
Pablo, the LPT math needs to be corrected as follows.

1.  LPT received for deposited liquidity in each pool asset according to
    the weight ratio (must be validated in code),

    <img src="0008-pablo-lbp-cpp/images/stem-66e7386f2786e17c7601a6ebac485e8d.png" width="124" height="23" alt="stem 66e7386f2786e17c7601a6ebac485e8d" />

    This ensures that the increase of LPT is proportional to the
    increase of the value function(invariant). The concept of an LP
    tax(equal to swap fee percentage) is introduced to counter the
    behavior of swapping without fees using add/remove liquidity
    operations (refer [Proof of Proportional LPT Calculation for
    Liquidity Added in Pool Weight
    Ratio](#_proof_of_proportional_lpt_calculation_for_liquidity_added_in_pool_weight_ratio)).

    <img src="0008-pablo-lbp-cpp/images/stem-ea0f1d60fb5c7dc02809347af8715cad.png" width="153" height="24" alt="stem ea0f1d60fb5c7dc02809347af8715cad" />
    ← (4)

2.  LPT received for deposited liquidity in a single pool asset (`k`),

    <img src="0008-pablo-lbp-cpp/images/stem-1e7c57b39463a3ceaf7a9cd5c0a3e1e9.png" width="200" height="23" alt="stem 1e7c57b39463a3ceaf7a9cd5c0a3e1e9" />

    When taking into account LP tax,

    <img src="0008-pablo-lbp-cpp/images/stem-103d99977ca241164b8dc0414681840b.png" width="229" height="24" alt="stem 103d99977ca241164b8dc0414681840b" />
    ← (5)

    One could see this formula is a generalization of the formula (4)
    when <img src="0008-pablo-lbp-cpp/images/stem-2449cc95b045878ba2dfb0f7d33a56d7.png" width="41" height="11" alt="stem 2449cc95b045878ba2dfb0f7d33a56d7" />
    (sum of all weights). Therefore, equation (5) can be used for both
    cases to get the amount of LPT issued.

3.  A sensible default must be derived for the issued LPT for the
    initial deposit in a pool as otherwise it would always be zero
    according to above formulae. Here [balancer
    uses](https://github.com/balancer-labs/balancer-v2-monorepo/blob/master/pkg/pool-weighted/contracts/BaseWeightedPool.sol#L192)
    the following formula which keeps the LPT supply consistent across
    pools.

    <img src="0008-pablo-lbp-cpp/images/stem-6343f5961f2798a1a812e2bfe7f63d84.png" width="108" height="14" alt="stem 6343f5961f2798a1a812e2bfe7f63d84" />
    ← (6)

    The [current
    implementation](https://github.com/ComposableFi/composable/blob/main/code/parachain/frame/composable-maths/src/dex/constant_product.rs#L131)
    based on Uniswap must be adjusted to be consistent here.

4.  Tokens received in each of the assets when withdrawing each type of
    asset available in a pool

    <img src="0008-pablo-lbp-cpp/images/stem-2105a4f6bba56e4de332240db91e8b0e.png" width="122" height="27" alt="stem 2105a4f6bba56e4de332240db91e8b0e" />
    ← (7)

5.  Tokens(`k`) received when withdrawing a single asset from a pool
    (refer [Proof of Proportional LPT Calculation for Liquidity Added in
    a Single Pool
    Asset](#_proof_of_proportional_lpt_calculation_for_liquidity_added_in_a_single_pool_asset))

    <img src="0008-pablo-lbp-cpp/images/stem-5474ba9fff4cc7058617f4df8aba3fe0.png" width="194" height="39" alt="stem 5474ba9fff4cc7058617f4df8aba3fe0" />
    ← (8)

    One could see this formula is a generalization of the formula (7)
    when <img src="0008-pablo-lbp-cpp/images/stem-2449cc95b045878ba2dfb0f7d33a56d7.png" width="41" height="11" alt="stem 2449cc95b045878ba2dfb0f7d33a56d7" />
    (sum of all weights). Therefore, equation (8) can be used for both
    cases to get the amount of tokens received. For all assets case the
    result must be used as the amount for all pool assets to be
    disbursed.

As per the derivations above , equations (5), (6) and (8) are the only
ones that need to be implemented at
`code/parachain/frame/composable-maths/src/dex/constant_product.rs`.
Then they must be integrated with relevant flows.

### 4.3. Validation: Pool Asset (Pair) Validation Must be at the Top Level of `Amm` trait Implementation

Currently significant amount of logic is executed upfront without
validating that the pool contains the given currencies for an operation
such as a swap.

### 4.4. Validation: the Asset Ratio When Adding Liquidity

The added liquidity must follow the same ratio as the pool weight
distribution according to balancer formulae. Currently, there is no such
validation.

### 4.5. Refactoring: `CurrencyPair` Usage

In the existing [pool data
structure](https://github.com/ComposableFi/composable/blob/main/code/parachain/frame/composable-traits/src/dex.rs#L269),
using `CurrencyPair` with "base" and "quote" naming creates confusion
when it comes to actual swap logic. A base or quote naming applies to a
currency only at the point of a trade. Specially when considering
possible multi-asset pools that a balancer based pool supports.

The proposal here is to use a list(vector) of maximum length of 2
(possibly allowing for future expansion) in the pool data structure for
both CPP and LBP.

    pub struct ConstantProductPoolInfo<AccountId, AssetId> {
        /// Owner of pool
        pub owner: AccountId,
        /// Swappable assets map asset_id => weight
        pub assets: Map<AssetId, Permill>,
        /// AssetId of LP token
        pub lp_token: AssetId,
        /// Amount of the fee pool charges for the exchange
        pub fee_config: FeeConfig,

    }

    // Remove
    pub struct LiquidityBootstrappingPoolInfo<AccountId, AssetId, BlockNumber> {
        /// Owner of the pool
        pub owner: AccountId,
        /// Asset pair of the pool along their weight.
        /// Base asset is the project token.
        /// Quote asset is the collateral token.
        pub assets: Map<AssetId, Permil>,
        /// Sale period of the LBP.
        pub sale: Sale<BlockNumber>,
        /// Trading fees.
        pub fee_config: FeeConfig,
    }

### 4.6. Unit Test Updates

1.  Introduce unit tests for
    `code/parachain/frame/composable-maths/src/dex/constant_product.rs`.

2.  All unit tests include fees (verified according to the math), with 0
    fees being the exception.

3.  Sufficient amount of cases to cover pools with differential weights
    according to the math.

4.  Sufficient amount of cases to cover LPT issued according to the
    math.

### 4.7. Algorithm: `constant_product::compute_out_given_in`

These are the modifications to be made to the existing
[function](https://github.com/ComposableFi/composable/blob/main/code/parachain/frame/composable-maths/src/dex/constant_product.rs#L59).

    pub fn compute_out_given_in<T: PerThing>(
        w_i: T,
        w_o: T,
        b_i: u128,
        b_o: u128,
        a_sent: u128,
        // f=0 for getting "out" without taking into account the fee
        f: T
    ) -> Result<(/* Out */ u128, /*Fee*/ u128), ArithmeticError> {
        // Calculate according to section 4.1 Eqn: 2
    }

### 4.8. Algorithm: `constant_product::compute_in_given_out`

These are the modifications to be made to the existing
[function](https://github.com/ComposableFi/composable/blob/main/code/parachain/frame/composable-maths/src/dex/constant_product.rs#L96).

    pub fn compute_in_given_out<T: PerThing>(
        wi: T,
        wo: T,
        bi: u128,
        bo: u128,
        ao: u128,
        // f=0 for getting "in" without taking into account the fee
        f: T
    ) -> Result<(/* In */ u128, /*Fee*/ u128), ArithmeticError>
    where
        T::Inner: Into<u32>,
    {
        // Calculate according to section 4.1 Eqn: 3
    }

### 4.9. Algorithm: `constant_product::compute_deposit_lp`

These are the modifications to be made to the existing
[function](https://github.com/ComposableFi/composable/blob/main/code/parachain/frame/composable-maths/src/dex/constant_product.rs#L148).

    pub fn compute_deposit_lp<T: PerThing>(
        lp_total_issuance: u128,
        num_asset_types_in_pool: u128,
        d_k: u128,
        b_k: u128,
        // w_k = 1 when providing liquidity in pool weight ratio for all assets
        w_k: T,
        // f=0 for getting "in" without taking into account the fee
        f: T
    ) -> Result<(/* LPT */ u128, /* fee */ u128), ArithmeticError> {
        let first_deposit = lp_total_issuance.is_zero();
        if first_deposit {
            // Calculate `lp_to_mint` according to section 4.2 Eqn: 6
            Ok(lp_to_mint, fee)
        } else {
            // Calculate `lp_to_mint` according to section 4.2 Eqn: 5
            Ok(lp_to_mint, fee)
        }
    }

### 4.10. Algorithm: `constant_product::compute_redeemed_for_lp`

This is a new function to be implemented as the previous version was
less specific.

    pub fn compute_redeemed_for_lp<T: PerThing>(
        lp_total_issuance: u128,
        lp_redeemed: u128,
        b_k: u128,
        // w_k = 1 when providing liquidity in pool weight ratio for all assets
        w_k: T,
    ) -> Result</* a_k */ u128, ArithmeticError> {
        // Calculate `a_k` according to section 4.2 Eqn: 8
    }

### 4.11. Algorithm: `Amm::currency_pair` → `Amm::assets`

This is a renaming plus a reorganization of this logic to better match
the `CurrencyPair` refactoring.Because of the [Refactoring:
`CurrencyPair` Usage](#_refactoring_currencypair_usage), this function
should just return the list of assets in the pool.

    pub trait Amm {
        // ....

        fn assets(pool_id: Self::PoolId) -> Result<Vec<AssetId>, DispatchError>;

        // ....
    }

<img src="0008-pablo-lbp-cpp/images/images/pablo-amm-currencies.png" width="289" height="211" alt="pablo amm currencies" />

### 4.12. Algorithm: `Amm::get_exchange_value`

Having the fee not taken into account here causes the fees to be
calculated in non-formal ways.Therefore, the proposal is to always take
into account the fee input as a parameter and return the fee as a
separate output.This also means that this function shall not be used to
calculate a quote amount for buy operations.

    pub struct AssetAmount<AssetId, Balance> {
        pub asset_id: AssetId,
        pub amount: Balance
    }

    pub struct ExchangeValue<AssetId, Balance> {
        value: AssetAmount<AssetId, Balance>,
        fee: AssetAmount<AssetId, Balance>,
    }

    pub trait Amm {
        // ....

        /// Return the exchange value out asset given in asset.
        fn get_exchange_value(
            pool_id: Self::PoolId,
            in_asset: AssetAmount<Self::AssetId, Self::Balance>,
            out_asset: Self::AssetId,
        ) -> Result<ExchangeValue<Self::AssetId, Self::Balance>, DispatchError>;

        // ....
    }

<img src="0008-pablo-lbp-cpp/images/images/pablo-amm-get-exchange-value.png" width="382" height="471" alt="pablo amm get exchange value" />

### 4.13. Algorithm: `Amm::exchange` → `Amm::swap`

Given the previously defined `Amm::get_exchange_value` function this
method can be simplified.

    pub trait Amm {
        // ....

        /// Performs an exchange to transfer the given
        /// quote amount to the pool while disbursing
        /// the calculated base amount according to the pool logic.
        /// Returns the disbursed value in base and fee charged.
        fn swap(
            who: &Self::AccountId,
            pool_id: Self::PoolId,
            in_asset: AssetAmount<Self::AssetId, Self::Balance>,
            min_receive: AssetAmount<Self::AssetId, Self::Balance>,
            keep_alive: bool,
        ) -> Result<ExchangeValue<Self::AssetId, Self::Balance>, DispatchError>;

        // ....
    }

<img src="0008-pablo-lbp-cpp/images/images/pablo-amm-exchange.png" width="377" height="497" alt="pablo amm exchange" />

### 4.14. Algorithm: `Amm::sell`

This would be removed to keep the interface simple as `Amm::swap`
satisfies the requirement.

### 4.15. Algorithm: `Amm::buy`

This function exists to provide a way for a user to buy a given amount
of an asset from the AMM.

    pub trait Amm {
        // ....

        /// Note: min_receive has been removed as the amount specified is considered the amount to be bought
        fn buy(
            who: &Self::AccountId,
            pool_id: Self::PoolId,
            in_asset: Self::AssetId,
            out_asset: AssetAmount<Self::AssetId, Self::Balance>,
            keep_alive: bool,
        ) -> Result<ExchangeValue<Self::AssetId, Self::Balance>, DispatchError>;

        // ....
    }

<img src="0008-pablo-lbp-cpp/images/images/pablo-amm-buy.png" width="385" height="578" alt="pablo amm buy" />

### 4.16. Algorithm: `Amm::add_liquidity`

LPs use this functionality to provide liquidity. It requires some
adjustments.

    pub trait Amm {
        // ....

        fn add_liquidity(
            who: &Self::AccountId,
            pool_id: Self::PoolId,
            // Bounds for the Vec can be specified here to based on a pallet config.
            // The details can be figured out in the implementation
            assets: Vec<AssetAmount<Self::AssetId, Self::Balance>>,
            min_mint_amount: Self::Balance,
            keep_alive: bool,
        ) -> Result<(), DispatchError>;

        // ....
    }

<img src="0008-pablo-lbp-cpp/images/images/pablo-amm-add-liquidity.png" width="959" height="819" alt="pablo amm add liquidity" />

### 4.17. Algorithm: `Amm::remove_liquidity`

This allows LPs to claim their liquidity back with possible profits.
Here also we need some adjustments.

    pub trait Amm {
        // ....

        fn remove_liquidity(
            who: &Self::AccountId,
            pool_id: Self::PoolId,
            lp_amount: Self::Balance,
            min_amounts: Vec<AssetAmount<Self::AssetId, Self::Balance>>,
        ) -> Result<(), DispatchError>;

        // ....
    }

<img src="0008-pablo-lbp-cpp/images/images/pablo-amm-remove-liquidity.png" width="1018" height="777" alt="pablo amm remove liquidity" />

### 4.18. Fail Safes

Fail safes can be categorized based on the level they act on,

#### 4.18.1. At the protocol level, where the entire Pablo protocol in multiple pallets is affected.

#### 4.18.2. At the pallet level, where a particular Pablo pallet is affected.

#### 4.18.3. At the pool type level where all pools of a particular pool type like LBP is affected.

#### 4.18.4. At the individual pool level where a single pool is affected

For LBPs,

1.  There needs to be a way to pause trading in situations where the
    trading activity is not favourable for the launch

#### 4.18.5. At the functionality level where a particular functionality for example like "removing liquidity" is affected.

## 5. Implementation

### 5.1. Stage 1: Cutting Down on Non-useful Parts for Launch

1.  Remove Stableswap(Curve) implementation together with tests, while
    keep the interfaces same.

2.  Remove LBP implementation together with tests, while keeping the
    interfaces the same.

### 5.2. Stage 2: Consistently Implement the Balancer Based CPP Equations

Implement the equations outlined in the [Method](#_method) in
`code/parachain/frame/composable-maths/src/dex/constant_product.rs`.

1.  Implement [Unit Test Updates](#_unit_test_updates)

2.  `Out-given-in` with Fee (eq: 2 + [Algorithm:
    `constant_product::compute_out_given_in`](#_algorithm_constant_productout_given_in))

3.  `In-given-out` with Fee (eq: 3 + [Algorithm:
    `constant_product::compute_in_given_out`](#_algorithm_constant_productcompute_in_given_out))

4.  `LPT received for deposited liquidity in a single pool asset` (eq:
    5 + [Algorithm:
    `constant_product::compute_deposit_lp`](#_algorithm_constant_productcompute_deposit_lp))

5.  `Tokens received for redeemed LPT` (eq: 8 + [Algorithm:
    `constant_product::compute_redeemed_for_lp`](#_algorithm_constant_productcompute_redeemed_for_lp))

### 5.3. Stage 3: Change/Implement Algorithms for CPP

1.  Implement [Refactoring: `CurrencyPair`
    Usage](#_refactoring_currencypair_usage).

2.  Implement [Algorithm: `Amm::currency_pair` →
    `Amm::assets`](#_algorithm_ammcurrency_pair__ammassets).

3.  Implement [Algorithm:
    `Amm::get_exchange_value`](#_algorithm_ammget_exchange_value).

4.  Implement [Algorithm: `Amm::exchange` →
    `Amm::swap`](#_algorithm_ammexchange__ammswap).

5.  Implement [Algorithm: `Amm::buy`](#_algorithm_ammbuy).

6.  Implement [Algorithm:
    `Amm::add_liquidity`](#_algorithm_ammadd_liquidity).

7.  Implement [Algorithm:
    `Amm::remove_liquidity`](#_algorithm_ammremove_liquidity).

8.  Ensure normalized weights are used everywhere.

### 5.4 Stage 4: Front-end Changes

1. Remove FE components that were built for LBP.
2. Remove FE components that were built for StableSwap.
3. Re-generate data types for any extrinsic/RPC changes and integrate.

## 6. Quality Assurance

QA could possibly just use the existing test cases and suites build for
Uniswap pools in this case. Though more effort needs to be put into
coming up with test cases that would cover LPT calculations and fees.

## 7. Audit

Audit can be conducted taking into account the specification of the
protocol outlined here according to balancer math. Any weaknesses found
on the original balancer protocol should be taken into account in the
audit.

## 8. Questions

1.  Would LBPs need to be converted to normal CPP after the sale has
    ended?

    1.  If so need to combine CPP and LBP data structures

    2.  Answer: No, as we can just create a new LP

## Appendix A: Proof of Fee for "In-given-out"

Fees are calculated on the "way-in" so starting with formula (2) in the
"Fee Math" section we have,

<img src="0008-pablo-lbp-cpp/images/stem-4760da3535b96aa8a9d1091a9036f300.png" width="200" height="37" alt="stem 4760da3535b96aa8a9d1091a9036f300" />

We can arrange this to know how to send(<img src="0008-pablo-lbp-cpp/images/stem-4c14314a2056e57fb583cb593c713440.png" width="29" height="10" alt="stem 4c14314a2056e57fb583cb593c713440" />)
given the out, we can isolate it,

<img src="0008-pablo-lbp-cpp/images/stem-cd9c13d4c86a93ab9161123e015c4ca2.png" width="164" height="30" alt="stem cd9c13d4c86a93ab9161123e015c4ca2" />

<img src="0008-pablo-lbp-cpp/images/stem-08ea09e92c9ab8dbbd651857b3bc4696.png" width="161" height="30" alt="stem 08ea09e92c9ab8dbbd651857b3bc4696" />

<img src="0008-pablo-lbp-cpp/images/stem-1251f8e404a5aa73c0f49793e30519e4.png" width="153" height="46" alt="stem 1251f8e404a5aa73c0f49793e30519e4" />

<img src="0008-pablo-lbp-cpp/images/stem-1251f8e404a5aa73c0f49793e30519e4.png" width="153" height="46" alt="stem 1251f8e404a5aa73c0f49793e30519e4" />

<img src="0008-pablo-lbp-cpp/images/stem-01e4e42483b3056afa34c8450739b00b.png" width="161" height="30" alt="stem 01e4e42483b3056afa34c8450739b00b" />

Now we get equation (3),

<img src="0008-pablo-lbp-cpp/images/stem-31ac5dca12391e047d736dc745781539.png" width="161" height="28" alt="stem 31ac5dca12391e047d736dc745781539" />

proven.

## Appendix B: Proof of Proportional LPT Calculation for Liquidity Added in a Single Pool Asset

Pool invariant is given by,

<img src="0008-pablo-lbp-cpp/images/stem-15bd89ddb08d995121bff6016bc1d892.png" width="66" height="15" alt="stem 15bd89ddb08d995121bff6016bc1d892" />
←(a)

Given liquidity provided for the token `k`, We would like to issue <img src="0008-pablo-lbp-cpp/images/stem-bda53125ba72ee8a17cc536d43e5d471.png" width="36" height="11" alt="stem bda53125ba72ee8a17cc536d43e5d471" />
such that movement of `c` is proportional to it,

<img src="0008-pablo-lbp-cpp/images/stem-0f5e2dfda7a5756d6a950d479d71afc6.png" width="70" height="25" alt="stem 0f5e2dfda7a5756d6a950d479d71afc6" />
←(b)

after increasing the balance (deposit) of k by <img src="0008-pablo-lbp-cpp/images/stem-1f0aa5770083d7bade7ac8aafcbfc008.png" width="19" height="11" alt="stem 1f0aa5770083d7bade7ac8aafcbfc008" />
using the invariant above we have,

<img src="0008-pablo-lbp-cpp/images/stem-164d505b6f4a7e8452b74d33a3d06b3b.png" width="181" height="15" alt="stem 164d505b6f4a7e8452b74d33a3d06b3b" />
←(c)

with (c) / (a), we have,

<img src="0008-pablo-lbp-cpp/images/stem-f2d8b725a1ed71d10a0994e3bb23addf.png" width="117" height="23" alt="stem f2d8b725a1ed71d10a0994e3bb23addf" />

Now with (b),

<img src="0008-pablo-lbp-cpp/images/stem-fbb647b18fd987c8baa1830086393dc1.png" width="135" height="25" alt="stem fbb647b18fd987c8baa1830086393dc1" />

<img src="0008-pablo-lbp-cpp/images/stem-0143e8b4f6c8f1b8f3d7b3c91b78cdc1.png" width="139" height="25" alt="stem 0143e8b4f6c8f1b8f3d7b3c91b78cdc1" />

With fees on the way in, we have,

<img src="0008-pablo-lbp-cpp/images/stem-a5d7d178529d58ead4a90eec3746d15a.png" width="169" height="25" alt="stem a5d7d178529d58ead4a90eec3746d15a" />

Now with rearrangement we have,

<img src="0008-pablo-lbp-cpp/images/stem-103d99977ca241164b8dc0414681840b.png" width="229" height="24" alt="stem 103d99977ca241164b8dc0414681840b" />

Thus, this proves equation (5).

## Appendix C: Proof of Proportional LPT Calculation for Liquidity Added in Pool Weight Ratio

Pool invariant is given by,

<img src="0008-pablo-lbp-cpp/images/stem-15bd89ddb08d995121bff6016bc1d892.png" width="66" height="15" alt="stem 15bd89ddb08d995121bff6016bc1d892" />
←(a)

Note that weights are normalized such that,

<img src="0008-pablo-lbp-cpp/images/stem-d8102e2fc30aa76eb17333dd6c8b275d.png" width="48" height="11" alt="stem d8102e2fc30aa76eb17333dd6c8b275d" />
←(b)

Given liquidity provided for the token `k`, We would like to issue <img src="0008-pablo-lbp-cpp/images/stem-bda53125ba72ee8a17cc536d43e5d471.png" width="36" height="11" alt="stem bda53125ba72ee8a17cc536d43e5d471" />
such that movement of `c` is proportional to it,

<img src="0008-pablo-lbp-cpp/images/stem-0f5e2dfda7a5756d6a950d479d71afc6.png" width="70" height="25" alt="stem 0f5e2dfda7a5756d6a950d479d71afc6" />
←(c)

With (a) when adding liquidity to all assets proportional to the pool
weights we have,

<img src="0008-pablo-lbp-cpp/images/stem-05dd9b810c4c97dc0f23f97492885427.png" width="141" height="13" alt="stem 05dd9b810c4c97dc0f23f97492885427" />
←(d)

With (d) / (a),

<img src="0008-pablo-lbp-cpp/images/stem-b443c7c0d5e81ceeb183220ab139d0dc.png" width="128" height="23" alt="stem b443c7c0d5e81ceeb183220ab139d0dc" />

As the liquidity is deposited in proportion to normalized weights, for
all `i`,

<img src="0008-pablo-lbp-cpp/images/stem-29957408ac7588570a5117ea96f01973.png" width="40" height="22" alt="stem 29957408ac7588570a5117ea96f01973" />

Where `k` is some constant. Then applying (b),

<img src="0008-pablo-lbp-cpp/images/stem-11b4a543a9ad7840eb6e4fe8a89133d6.png" width="84" height="22" alt="stem 11b4a543a9ad7840eb6e4fe8a89133d6" />

Then with (c),

<img src="0008-pablo-lbp-cpp/images/stem-bb05b4adc5dd94cd4f5a87dea8b2f997.png" width="67" height="25" alt="stem bb05b4adc5dd94cd4f5a87dea8b2f997" />

Applying fee on the way-in

<img src="0008-pablo-lbp-cpp/images/stem-1e803fe380449ad10b688c20c99bae49.png" width="94" height="25" alt="stem 1e803fe380449ad10b688c20c99bae49" />

With rearrangement,

<img src="0008-pablo-lbp-cpp/images/stem-ea0f1d60fb5c7dc02809347af8715cad.png" width="153" height="24" alt="stem ea0f1d60fb5c7dc02809347af8715cad" />

Thus proves equation (4).

Last updated 2022-10-06 19:05:40 +0200
