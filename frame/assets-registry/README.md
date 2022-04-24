

# Overview



- helps to hide complexity of creating mappings of local ids to and from HydraDX, with proper authorities and voting
- should support any asset to be added
- support to transfer assets to HydraDX (forward mapping)
- support to transfer assets back from HydraDX to Composable (back mapping)
- supports not only HydraDX

## v1

- support small set of well know assets added manually on both registries
- manually asking HydraDX to add mapping

## vNEXT

- support any assets to be added new via voting processes


//! Pallet for allowing to map assets from this and other parachain.
//!
//! It works as next:
//! 1. Each mapping is bidirectional.
//! 2. Assets map added as candidate and waits for approval.
//! 3. After approval map return mapped value.
//! 4. Map of native token to this chain(here) is added unconditionally.
