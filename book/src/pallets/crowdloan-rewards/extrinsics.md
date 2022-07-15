<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-06-25T22:31:58.382857475Z -->

# Crowdloan Rewards Pallet Extrinsics

## Initialize

[`initialize`](https://dali.devnets.composablefinance.ninja/doc/pallet_crowdloan_rewards/pallet/enum.Call.html#variant.initialize)

Initialize the pallet at the current transaction block.

## Initialize At

[`initialize_at`](https://dali.devnets.composablefinance.ninja/doc/pallet_crowdloan_rewards/pallet/enum.Call.html#variant.initialize_at)

Initialize the pallet at the given transaction block.

## Populate

[`populate`](https://dali.devnets.composablefinance.ninja/doc/pallet_crowdloan_rewards/pallet/enum.Call.html#variant.populate)

Populate pallet by adding more rewards.
Can be called multiple times. If an remote account already has a reward, it will be
replaced by the new reward value.
Can only be called before `initialize`.

## Associate

[`associate`](https://dali.devnets.composablefinance.ninja/doc/pallet_crowdloan_rewards/pallet/enum.Call.html#variant.associate)

Associate a reward account. A valid proof has to be provided.
This call also claim the first reward (a.k.a. the first payment, which is a % of the
vested reward).
If logic gate pass, no fees are applied.

The proof should be:

````haskell
proof = sign (concat prefix (hex reward_account))
````

## Claim

[`claim`](https://dali.devnets.composablefinance.ninja/doc/pallet_crowdloan_rewards/pallet/enum.Call.html#variant.claim)

Claim a reward from the associated reward account.
A previous call to `associate` should have been made.
If logic gate pass, no fees are applied.
