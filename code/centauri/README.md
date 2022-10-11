##  Centauri: Trustless Bridging Protocol

This is the concrete implementation of the centauri bridging protocol, based on IBC, Powered by light clients.

###     [beefy-light-client](algorithms/beefy/verifier/src/lib.rs)

This is a `no_std` compatible crate that contains functions for verifying BEEFY commitments and Parachain headers which have been finalized by the BEEFY protocol.

###     [beefy-prover](algorithms/beefy/prover/src/lib.rs)
This contains utility functions for assembling BEEFY proofs as well as parachain proofs from a running node, that can then be verified by the light-client crate.

###     [beefy-primitives](algorithms/beefy/primitives/src/lib.rs)

A `no_std` compatible crate which contains primitive types which are shared by both crates.

###     [grandpa-light-client](algorithms/grandpa/verifier/src/lib.rs)

This is a `no_std` compatible crate that contains functions for verifying GrandPa commitments and Parachain headers which have been finalized by the GrandPa protocol.

###     [grandpa-prover](algorithms/grandpa/prover/src/lib.rs)
This contains utility functions for assembling Grandpa proofs as well as parachain proofs from a running node, that can then be verified by the light-client crate.

###     [grandpa-primitives](algorithms/grandpa/primitives/src/lib.rs)

A `no_std` compatible crate which contains primitive types which are shared by both crates.

### [Hyperspace Relayer](hyperspace/src/lib.rs)


Rust implementation of the IBC relayer algorithm.

### Goals

 ✅ Event driven architecture.
 <br />
 ✅ Fully stateless with no caching, and instead relies on full nodes for data storage as a source of truth.
  <br />
 ✅ Chain agnostic, so it can be extended to support new chains with little no changes to the core framework.


Named after a fictional technology in starwars for [relaying deep space messages](https://starwars.fandom.com/wiki/Hyperspace_relay).