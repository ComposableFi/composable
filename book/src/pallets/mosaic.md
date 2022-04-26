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
