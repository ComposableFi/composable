#  beefy-rs

A set of crates that are useful for working with the BEEFY protocol on polkadot parachains.

###     [beefy-light-client](./src/lib.rs)

This is a `no_std` compatible crate that contains functions for verifying BEEFY commitments and Parachain headers which have been finalized by the BEEFY protocol.

###     [beefy-prover](./prover/src/lib.rs)
This contains utility functions for assembling BEEFY proofs as well as parachain proofs from a running node, that can then be verified by the light-client crate.

###     [beefy-primitives](./primitives/src/lib.rs)

A `no_std` compatible crate which contains primitive types which are shared by both crates.