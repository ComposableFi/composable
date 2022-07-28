# Overview

Proposes flow to do runtime upgrades, likely similar to what others do.

Expected that the reader has understood official Parity documents and explainers regarding upgrades.

## Constraints

- Upgrades can be done only under `Root` origin.
- Upgrades are not immediate because runtime should be copied to the majority of para chain collators and relay validators and be enabled simultaneously on a specific block in the future.

## Flow

### I want some code to be running in Picasso parachain

I make sure that:

- All code is on main protected branch of this repo.
- Code is part `dali` runtime configuration.
- Code is part of `picasso` runtime configuration.
- Relevant(see other documents regarding acceptance criteria) preliminary checks, tests, and audits passed for added code.
- I send git commit hash to #sre to ask to deploy `wasm` including that hash.

### As SRE, we help to deploy changes to the runtime

- Given valid and credible git hash
- We ensure that runtime `wasm` with referenced code is available in `GitHub Releases`
- We collect all hashes to include in the next runtime upgrade.
- We ask for consensus from @ComposableFi/testers (QA) and @ComposableFi/technical-writers (docs are up to date with runtime) and @ComposableFi/security and @ComposableFi/blockchain-integrations (UI/Frontend/Explorer) are for upgrade including relevant hashes

### With SUDO (key, multi-signature, proxy)

@ComposableFi/sre follow Parity docs on runtime upgrade.

### Without SUDO

- @ComposableFi/sre run CI job which uploads preimage to upgrade runtime on behalf of sudo
- Sent message to `council` members about the new upgrade.

### Council collective

- Creates default YES voting on democracy
- Votes yes with their funds

### Technical collective

- Fast tracks enactment

## References

- <https://docs.substrate.io/tutorials/get-started/forkless-upgrade/>
- <https://github.com/paritytech/substrate/blob/master/frame/system/src/lib.rs>
- <https://paritytech.github.io/substrate/master/sp_version/struct.RuntimeVersion.html>
- Parith Technical explainers
- [governance](../doc/governance.md)
