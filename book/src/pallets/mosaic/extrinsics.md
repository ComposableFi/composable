# `mosaic` Pallet Extrinsics

## `set_budget`

Sets the current Relayer configuration.

This is enacted immediately and invalidates inflight/ incoming transactions from the
previous Relayer. However, existing budgets remain in place.

This can only be called by the \[`ControlOrigin`\].

### Restrictions

* Only callable by root

## `transfer_to`

Creates an outgoing transaction request, locking the funds locally until picked up by
the relayer.

### Restrictions

* Network must be supported.
* AssetId must be supported.
* Amount must be lower than the networks `max_transfer_size`.
* Origin must have sufficient funds.
* Transfers near Balance::max may result in overflows, which are caught and returned as
  an error.

## `accept_transfer`

This is called by the Relayer to confirm that it will relay a transaction.

Once this is called, the sender will be unable to reclaim their tokens.

If all the funds are not removed, the reclaim period will not be reset. If the
reclaim period is not reset, the Relayer will still attempt to pick up the
remainder of the transaction.

### Restrictions

* Origin must be relayer
* Outgoing transaction must exist for the user
* Amount must be equal or lower than what the user has locked

### Note

* Reclaim period is not reset if not all the funds are moved; menaing that the clock
  remains ticking for the relayer to pick up the rest of the transaction.

## `claim_stale_to`

Claims user funds from the `OutgoingTransactions`, in case that the relayer has not
picked them up.

## `timelocked_mint`

Mints new tokens into the pallet's wallet, ready for the user to be picked up after
`lock_time` blocks have expired.

## `set_timelock_duration`

No documentation available at this time.

## `rescind_timelocked_mint`

Burns funds waiting in incoming_transactions that are still unclaimed. May be used by
the relayer in case of finality issues on the other side of the bridge.

## `claim_to`

Collects funds deposited by the relayer into the owner's account

## `update_asset_mapping`

Update a network asset mapping.

The caller must be `ControlOrigin`.

Possibly emits one of:

* `AssetMappingCreated`
* `AssetMappingDeleted`
* `AssetMappingUpdated`
