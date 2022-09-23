# Overview

An overview of the technical side of Composable's governance.

This document describes organization entities(represented as origins) ability to exercise authority rather than enacted token economics motions as a consequence.
Protocols' governance configurations are not set in stone. These can be reconfigured and modified as time goes by. The document does not prescribe a specific setup.
This document helps decision-making and frames what governance can be.

The description targets more technical audiences with references to non-technical components in the `book` or relevant documents.

## Requirements

- Need to support non `runtime` native tokens, which are native `protocol` tokens
- Can setup test runtimes for fast and easy testing
- Must be able to act on `root` behalf
- Each `protocol` must be able to have its own governance with it's own protocol token if required

## Relevant contracts and pallets

Given primitives can be composed and configured to archive desired level of decentralization and security.

We can use any of the pallets above to execute on behalf of `origin` of varying levels of complexity.

Pallets can be configured dependent on protocol needs.

### Substrate

**Multisig**:

- Answers `N out of M signed`
- Allows several accounts to represent one account
- Cannot add or remove accounts

 **Collective**:

- Answers `N out of M agree`
- Allows to have set of accounts(origins) agree(simple voting)
- Execute dispatchable from one member(and validate that member is part of collective)
- Execute dispatch from just enough members to satisfy threshold(and validate that threshold is
   met)
- Only members of collective can propose dispatchable (hence it is not `democracy`)
- Allows to seed initial members via root call
- Usage [picasso](./council.md)

 **Ranked collective**:

- As collective, but members has different weight per vote

 **Membership**:

- Store and update members
- Answers `is this one X in set`
- Sends events about set update
- Allows `collective` to propose membership changes
- Unix like group

 **Proxy**:

- Is like sudo
- But cannot do root operations
- Allows to delegate tokens only for voting
- Composes with `Multisig` well

**Democracy**:

- Allows voting with tokens
- Allows several types of voting defaults
- Allows several ways to submit proposals
- And fast track some changes or cancel
- Very well documented

**ElectionsPhragmen**:

- Allows voting on a setup **collective** of members.
- Same as used to setup validators

There are other pallets like `Referenda`, `Society`, `Fellowship`, and `Incentives(bribe)`.

## Composable

 **Governance-registry**

- Allows the assurance that the origin of each token can mint and burn its respective token(s)
- Any `democracy` voting using said token will dispatch calls using that origin (native token always dispatches from `root`)
- Each token can have only one origin

 **Bribe**

Ideated primitive which allows:

- Users can rent/borrow governance tokens to use in democracy for a specified amount of other tokens. Users can also delegate tokens from fNFT staking to receive governance tokens. [bribe]
- Users can deposit incentives of other tokens for specific votes, which voters can claim after the `right` decision passes [bribe]

`bribe` will be composable with `democracy`

## Tokenomics

See `composable/staking-rewards` and `polkadot/staking`, `substrate/treasury`, `sora-xor/sora2-network` and other `protocols` pallets on which `governances` can act in mix of some tokenomics/incentives.

And relevant chapters from the `book` on Angular, Apollo, Pablo, and Cubic.

## Multi-democracy

As of now, `democracy` per asset is configured during compile time, which limits democracies to some well-known protocols, but makes it fully compatible with the existing frontend.

At a later date, `democracy` will be done on top of the latest developments from Parity but allows for thousands of runtime configured `democracies`(with `governance-registry`).

## Sudo access

Sudo is built into many pallets in substrate/cumulus/orml/polkadot as the origin that issues transactions.

That pallet is usually given the unconstrained possibility to execute many extrinsics.

In some critical cases, there is no ability configured for pallets to be executed by anyone other than Sudo.

The `sudo` pallet is backed by a single key.

### Sudo from `democracy`

In the current set of existing pallets, `sudo` calls can be issued by democracy.

For the following part, we assume you know the workflow of democracy.

Steps:

- `democracy` is set with `ExternalMajorityOrigin` to be `collective`(Council) with few members
- `simple majority carries referendum` proposed by `ExternalMajorityOrigin`
- `InstantOrigin`(`collective`, `technical`) fast-track referendum for voting
- `EnactmentPeriod` is overridden by a period that is set to be small in the previous extrinsic call

### Multisignature proxy setup

Example, imagine we want to get next:

> There is updatable multisignature account with 2 of 3 threshold.
>
> Each of these 3 also updated multisignature account of any chosen threshold.

`Seed account` `S` creates 3 anonymous proxies `A`, `B`, and `C`.

`S` creates `2 of 3` multisignature account `ABC`. `S` sets `ABC` as `origin` of some protocol.
Example, manager of Pablo pool.

`S` does ***proxy*** call on behalf of `A` to delegate to `X`. `X` is arbitrary multisignature.
`S` removes self from `A` delegates.

If `X` to update self, it creates new multisignature `X'`.
`X` does ***proxy*** call on behalf of `A` to delegate to `X'`.
`X` removes self from `A` delegates.

Same of `Y` and `Z` to be delegates of `B` and `C`.

Imagine `X` is `security team`, `Y` is `composable officers` and `Z` is `core developers`.
This way we can have simplified weaker form of `councils` and `fast track democracy`.

## References

### In polkadot

- <https://www.shawntabrizi.com/substrate/the-sudo-story-in-substrate/>
- <https://github.com/AcalaNetwork/Acala/blob/master/runtime/common/src/lib.rs>
- <https://polkadot.network/blog/polkadot-governance/>
- <https://wiki.polkadot.network/docs/maintain-guides-democracy>
- <https://kusama.polkassembly.io/>
- <https://wiki.polkadot.network/docs/learn-governance>
- <https://www.youtube.com/watch?v=tBvxn8WfcFI>

### Others

- <https://vitalik.ca/general/2019/12/07/quadratic.html>
- <https://vitalik.ca/general/2021/08/16/voting3.html>

[bribe]: https://bribe.gitbook.io/bribe/
