# Mosaic

Pallet implementing an interface for the Mosaic Relayer. As opposed to the EVM-EVM bridge, this pallet takes a different approach and uses `mint` and `burn` operations.
Because of that it also limits the mintable amount by the relayer using a decaying penalty.

## Decaying Penalty

At moment N, the relayer has a maximum budget `budget`. Minting a token adds a penalty `penalty` to the relayer. The penalty decreases each block according to decay function `decayer`,
which depends on the penalty, current_block, and last_decay_block. The current maximum amount that the relayer can mint is given by `budget - decayer(penalty, current_block, last_decay_block)`.
The new penalty is the decayed previous penalty plus the minted amount.
