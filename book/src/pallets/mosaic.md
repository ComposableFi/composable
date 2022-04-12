# Mosaic

*The Mosaic pallet enables cross-chain and cross-layer transfers*

---

## Overview

The Mosaic Pallet implements the interface for the Mosaic Relayer. The Mosaic 
Relayer will relay liqudity accross chains and layers.

As opposed to the EVM-EVM bridge, this pallet takes a different approach and 
uses mint and burn operations. Because of that it also limits the amount the 
relayer can mint using a decaying penalty.

### Decaying Penalty

At moment N, the Relayer has a maximum budget budget. Minting a token adds a 
penalty penalty to the Relayer. The penalty decreases each block according to 
decay function decayer, which depends on the penalty, `current_block`, and 
`last_decay_block`. The current maximum amount that the Relayer can mint is 
given by `budget - decayer(penalty, current_block, last_decay_block)`. The new 
penalty is the decayed previous penalty plus the minted amount.

## Pallet Extrinsics

### Set Relayer 

`set_relayer`

Sets the current Relayer configuration.

This is enacted immediately and invalidates inflight, incoming transactions from
the previous Relayer. However, existing budgets relain in place.

This can only be called by the[`ControlOrigin`](#controlorigin). 
### Rotate Relayer

`rotate_relayer`

Rotates the relay account.

#### Restrictions

This is only callable by the current Relayer.

The Time To Live (TTL) must be greater than the [`MinimumTTL`](#minimumttl).

### Set Network

`set_network`

Sets the supported networks and maximum transaction sizes accepted by the 
Relayer.

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

### Accept Transfer 

`accept_transfer`

This is called by the Relayer to confirm that it will relay a transaction.

Once this is called, the sender will be unable to reclaim their tokens.

If all the funds are not removed, the reclaim period will not be reset. If the 
reclaim period is not reset, the Relayer will still attempt to pick up the 
remainder of the transaction.

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

## Pallet Configuration

### Event

### PalletId

### Assets

### MinimumTTL

Minimum time period, in blocks, that outgoing and incoming funds are locked.

### BudgetPenaltyDecayer

The budget penalty decayer.

### NetworkId

Network identifier.

### RemoteAssetId

Remote asset identifier.

### ControlOrigin

Origin capable of setting up the Relayer.

Acts as a root or half council as they will be capable of stopping attacks.

### WeightInfo

Weight implementation used for extrinsics.
