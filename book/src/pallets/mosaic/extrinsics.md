# Mosaic Extrinsics

## Set Relayer

{{#doc_comment_include ../../../../frame/mosaic/src/lib.rs:set_relayer_docs}}

## Rotate Relayer

`rotate_relayer`

Rotates the relay account.

#### Restrictions

This is only callable by the current Relayer.

The Time To Live (TTL) must be greater than the [`MinimumTTL`](#minimumttl).

### Set Network

`set_network`

Sets the supported networks and maximum transaction sizes accepted by the Relayer.

This can only be called by the current Relayer.

### Set Budget

`set_budget`

Sets the Relayer budget for incoming transactions for specific assets.

This does not reset the current `penalty`.

This can only be called by the [`ControlOrigin`](#controlorigin).

### Transfer To

`transfer_to`

Creates an outgoing transaction request.

Locks the funds locally until picked up by the Relayer.

#### Restrictions

* The network must be supported

* The asset ID must be supported

* The amount must have sufficient funds

* The origin must have sufficient funds

* Transactions that cause overflows (due to being too close to exceeding the max
  balance) will be caught and returned as errors

## Accept Transfer

{{#doc_comment_include ../../../../frame/mosaic/src/lib.rs:accept_transfer_docs}}

### Claim Stale To

`claim_stale_to`

Claims funds from outgoing transactions not yet picked up by the Relayer.

### Time Locked Mint

`timelocked_mint`

Mints new tokens into the pallets wallet.

These tokens will be available for pickup after the `lock_time` blocks have
passed.

### Set Time Lock Duration

`set_timelock_duration`

Sets the time lock, in blocks, on new transfers.

This can only be called by the `ControlOrigin`.

### Rescind Time Locked Mint

`rescind_timelocked_mint`

Burns unclaimed funds that are waiting in incoming transactions.

This may be used by the Relayer in case of finality issues on the other side of
the bridge.

### Claim To

`claim_to`

Collects funds that have been deposited by the Relayer into the owner's account.

### Update Asset Mapping

`update_asset_mapping`

Updates a network asset mapping.

This can only be called by the [`ControlOrigin`](#controlorigin).
