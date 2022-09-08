<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-09-05T18:35:35.096109Z -->

# Mosaic Pallet Extrinsics

## Set Relayer

[`set_relayer`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.set_relayer)

Sets the current Relayer configuration.

This is enacted immediately and invalidates inflight/ incoming transactions from the
previous Relayer. However, existing budgets remain in place.

This can only be called by the [`ControlOrigin`].

[`ControlOrigin`]: https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/trait.Config.html#associatedtype.ControlOrigin

## Rotate Relayer

[`rotate_relayer`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.rotate_relayer)

Rotates the Relayer Account

### Restrictions

* Only callable by the current Relayer.
* The Time To Live (TTL) must be greater than the [`MinimumTTL`](Config::MinimumTTL)

## Set Network

[`set_network`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.set_network)

Sets supported networks and maximum transaction sizes accepted by the Relayer.

Only callable by the current Relayer

## Set Budget

[`set_budget`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.set_budget)

Sets the relayer budget for *incoming* transactions for specific assets. Does not reset
the current `penalty`.

### Restrictions

* This can only be called by the [`ControlOrigin`](Config::ControlOrigin)

## Transfer To

[`transfer_to`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.transfer_to)

Creates an outgoing transaction request, locking the funds locally until picked up by
the Relayer.

### Restrictions

* Network must be supported.
* AssetId must be supported.
* Amount must be lower than the networks `max_transfer_size`.
* Origin must have sufficient funds.
* Transfers near Balance::max may result in overflows, which are caught and returned as
  an error.

## Accept Transfer

[`accept_transfer`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.accept_transfer)

This is called by the Relayer to confirm that it will relay a transaction.

Once this is called, the sender will be unable to reclaim their tokens.

If all the funds are not removed, the reclaim period will not be reset. If the
reclaim period is not reset, the Relayer will still attempt to pick up the
remainder of the transaction.

### Restrictions

* Only callable by the current Relayer
* Outgoing transaction must exist for the user
* Amount must be equal or lower than what the user has locked

### Note

* Reclaim period is not reset if not all the funds are moved; meaning that the clock
  remains ticking for the relayer to pick up the rest of the transaction.

## Claim Stale To

[`claim_stale_to`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.claim_stale_to)

Claims user funds from the `OutgoingTransactions`, in case that the Relayer has not
picked them up.

## Timelocked Mint

[`timelocked_mint`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.timelocked_mint)

Mints new tokens into the pallet's wallet, ready for the user to be picked up after
`lock_time` blocks have expired.

Only callable by the current Relayer

## Set Timelock Duration

[`set_timelock_duration`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.set_timelock_duration)

Sets the time lock, in blocks, on new transfers

This can only be called by the [`ControlOrigin`](Config::ControlOrigin)

## Rescind Timelocked Mint

[`rescind_timelocked_mint`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.rescind_timelocked_mint)

Burns funds waiting in incoming_transactions that are still unclaimed.

May be used by the Relayer in case of finality issues on the other side of the bridge.

## Claim To

[`claim_to`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.claim_to)

Collects funds deposited by the Relayer into the owner's account

## Update Asset Mapping

[`update_asset_mapping`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.update_asset_mapping)

Update a network asset mapping.

This can only be called by the [`ControlOrigin`](Config::ControlOrigin)

Possibly emits one of:

* `AssetMappingCreated`
* `AssetMappingDeleted`
* `AssetMappingUpdated`

## Add Remote Amm Id

[`add_remote_amm_id`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.add_remote_amm_id)

Adds a remote AMM for a specific Network

## Remove Remote Amm Id

[`remove_remote_amm_id`](https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/enum.Call.html#variant.remove_remote_amm_id)

Removes a remote AMM for a specific Network
