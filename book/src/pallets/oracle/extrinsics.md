<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-06-25T22:31:58.333372989Z -->

# Oracle Pallet Extrinsics

## Add Asset And Info

[`add_asset_and_info`](https://dali.devnets.composablefinance.ninja/doc/pallet_oracle/pallet/enum.Call.html#variant.add_asset_and_info)

Permissioned call to add an asset

* `asset_id`: Id for the asset
* `threshold`: Percent close to mean to be rewarded
* `min_answers`: Min answers before aggregation
* `max_answers`: Max answers to aggregate
* `block_interval`: blocks until oracle triggered
* `reward`: reward amount for correct answer
* `slash`: slash amount for bad answer

Emits `DepositEvent` event when successful.

## Set Signer

[`set_signer`](https://dali.devnets.composablefinance.ninja/doc/pallet_oracle/pallet/enum.Call.html#variant.set_signer)

Call for a signer to be set, called from controller, adds stake.

* `signer`: signer to tie controller to

Emits `SignerSet` and `StakeAdded` events when successful.

## Add Stake

[`add_stake`](https://dali.devnets.composablefinance.ninja/doc/pallet_oracle/pallet/enum.Call.html#variant.add_stake)

call to add more stake from a controller

* `stake`: amount to add to stake

Emits `StakeAdded` event when successful.

## Remove Stake

[`remove_stake`](https://dali.devnets.composablefinance.ninja/doc/pallet_oracle/pallet/enum.Call.html#variant.remove_stake)

Call to put in a claim to remove stake, called from controller

Emits `StakeRemoved` event when successful.

## Reclaim Stake

[`reclaim_stake`](https://dali.devnets.composablefinance.ninja/doc/pallet_oracle/pallet/enum.Call.html#variant.reclaim_stake)

Call to reclaim stake after proper time has passed, called from controller

Emits `StakeReclaimed` event when successful.

## Submit Price

[`submit_price`](https://dali.devnets.composablefinance.ninja/doc/pallet_oracle/pallet/enum.Call.html#variant.submit_price)

Call to submit a price, gas is returned if all logic gates passed
Should be called from offchain worker but can be called manually too
Operational transaction

* `price`: price to submit
* `asset_id`: Id for the asset

Emits `PriceSubmitted` event when successful.
