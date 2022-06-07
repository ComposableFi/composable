<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-04-22T18:59:06.829836435Z -->

# Staking Rewards Pallet Extrinsics

## Configure

[`configure`](https://dali.devnets.composablefinance.ninja/doc/pallet_staking_rewards/pallet/enum.Call.html#variant.configure)

Enable a protocol staking configuration.

Arguments

* `origin` the origin that signed this extrinsic, must be `T::GovernanceOrigin`.
* `staking_configuration` the staking configuration for the given protocol `asset`.

## Stake

[`stake`](https://dali.devnets.composablefinance.ninja/doc/pallet_staking_rewards/pallet/enum.Call.html#variant.stake)

Stake an amount of protocol asset tokens. Generating an NFT for the staked position.

Arguments

* `origin` the origin that signed this extrinsic. Must be the owner of the NFT targeted
  by `instance_id`.
* `amount` the amount of tokens to stake.
* `duration` the duration for which the tokens will be staked.
* `keep_alive` whether to keep the caller account alive or not.

## Unstake

[`unstake`](https://dali.devnets.composablefinance.ninja/doc/pallet_staking_rewards/pallet/enum.Call.html#variant.unstake)

Unstake an amount of protocol asset tokens.

Arguments

* `origin` the origin that signed this extrinsic. Must be the owner of the NFT targeted
  by `instance_id`.
* `instance_id` the ID of the NFT that represent our staked position.
* `to` the account in which the rewards will be transferred before unstaking.

## Claim

[`claim`](https://dali.devnets.composablefinance.ninja/doc/pallet_staking_rewards/pallet/enum.Call.html#variant.claim)

Claim the current available rewards.

Arguments

* `origin` the origin that signed this extrinsic. Can be anyone. by `instance_id`.
* `instance_id` the ID of the NFT that represent our staked position.
* `to` the account in which the rewards will be transferred.
