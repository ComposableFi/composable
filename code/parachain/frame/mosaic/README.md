# Mosaic

The Mosaic pallet enables cross-chain and cross-layer transfers

## Overview

The Mosaic Pallet implements the interface for the Mosaic Relayer. The Mosaic 
Relayer will relay liquidity across chains and layers.

As opposed to the EVM-EVM bridge, this pallet takes a different approach and 
uses mint and burn operations. Because of that it also limits the amount the 
Relayer can mint using a decaying penalty.

### Decaying Penalty

At moment N, the Relayer has a maximum budget. Minting a token adds a 
penalty to the Relayer. The penalty decreases each block according to 
decay function decayer, which depends on the penalty, `current_block`, and 
`last_decay_block`. The current maximum amount that the Relayer can mint is 
given by `budget - decayer(penalty, current_block, last_decay_block)`. The new 
penalty is the decayed previous penalty plus the minted amount.

## Workflow

The Mosaic pallet is comprised of three main components: the Relayer interface, 
incoming transactions, and outgoing transactions.

### The Relayer Interface

The Relayer interface provides the necessary functionality for managing the 
constraints of the Relayer and exposing the functionality needed by the Relayer 
to conduct transactions. The Relayer, while operating on this network, is 
constrained by the set budget, supported networks, and the maximum transaction 
sizes for those networks. The Relayer will need to be able to accept new 
transactions and mint the funds needed for conducting those transactions. 
Additionally, in the event of finality issues on the destination network, the 
Relayer will need to be able to burn funds.

The Relayer's net budget is determined by the set gross budget and the [decay 
penalty](#decaying-penalty). This budget controls the minting capabilities of 
the Relayer on this network.

### Incoming Transactions

Incoming transactions are transactions who's destination is this network. Once 
funds have been minted on this network and the lock time has passed, they may be 
claimed to conclude the transaction.

In the event that any issues occurred with finalizing a transaction, the waiting 
funds associated with incoming transactions that have yet to be claimed may be 
burned by the Relayer.

### Outgoing Transactions

Outgoing transactions are transactions originating on this network who's 
destination is elsewhere. These transactions must first be requested by the 
user, then accepted by the Relayer. Once accepted by the Relayer the funds of 
this transaction will no longer be claimable on the origin network. Before 
acceptance by the Relayer, funds may be reclaimed by the user.
