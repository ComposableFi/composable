#  composable-bridge-common

A set of crates that power composable's trustless bridge infrastructure.

###     [beefy-light-client](algorithms/beefy/src/lib.rs)

This is a `no_std` compatible crate that contains functions for verifying BEEFY commitments and Parachain headers which have been finalized by the BEEFY protocol.

###     [beefy-prover](algorithms/beefy/prover/src/lib.rs)
This contains utility functions for assembling BEEFY proofs as well as parachain proofs from a running node, that can then be verified by the light-client crate.

###     [beefy-primitives](algorithms/beefy/primitives/src/lib.rs)

A `no_std` compatible crate which contains primitive types which are shared by both crates.

###     [grandpa-light-client](algorithms/grandpa/src/lib.rs)

This is a `no_std` compatible crate that contains functions for verifying GrandPa commitments and Parachain headers which have been finalized by the GrandPa protocol.

###     [grandpa-prover](algorithms/grandpa/prover/src/lib.rs)
This contains utility functions for assembling Grandpa proofs as well as parachain proofs from a running node, that can then be verified by the light-client crate.

###     [grandpa-primitives](algorithms/grandpa/primitives/src/lib.rs)

A `no_std` compatible crate which contains primitive types which are shared by both crates.
