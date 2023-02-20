# Overview

RFC improving Security of on chain operations

## Champion

@dzmitry-lahoda
 
## Audit

Peer review only, ideally review from other chain developers, but not required


## Problem

- Picasso and Composable chains lack security and trust in the community. 
- Our governance setup is not compatible with ongoing iterations of Dotsama tooling.

##  Solution

- Enable Democracy 2.0 from Parity on both chains. This solution is order of magnitude simpler than what we have, and is officially running on Kusama and Polkadot forward looking solutions for all types of governance.
- Disable sudo during runtime upgrade.

We will increase security using decentralization and ensure the community that we are aligned to decentralized goals.

## No Gos

We will not handle now, but

- Democracy 2.0 is ready for multi token democracy
- Is compatible with fNFT 
- fNFT and democracy 2.0 are compatible with some forms of ZK democracies and initiatives for delegation

## Resources

- Several days to do changes/tests/prs in runtime code by Dzmitry Lahoda.
- Several days from Dominic to rewrite Democracy tests.
- Several days to do relevant runtime upgrades.

## Success Criteria

- No single signature sudo on Picasso and Composable enabled.
- Simplified democracy governance sends on chain remarks.
