# Overview

Proposes flow to do runtime upgrades of Picasso.

In its core [upgrades](https://docs.substrate.io/tutorials/get-started/forkless-upgrade) are as described by Parity for the developer network.

Overview of what is technical essence of upgrade can be found [here](../docs/runtime-upgrades-and-versioning.md)
Extended with security measures, like Karura and Kusama have, to protect liquidity on network.

It is expected that the reader has understood:

- [explainers](https://www.youtube.com/playlist?list=PLOyWqupZ-WGuAuS00rK-pebTMAOxW41W8)
- [upgrades](https://docs.substrate.io/tutorials/get-started/forkless-upgrade)
- [governance](../doc/governance.md)
- how to use `polkadot.js` against our parachains

It is helpful if the reader understands *nix [substitute user](https://en.wikipedia.org/wiki/Sudo) operation.

This document is not intended to described detailed specifications of `quality gates`.
The Document specifies relevant domains and stakeholders and teams, but doesn't name exactly what communication channel these would use as communication medium neither enumerates all stakeholders.

## Constraints

- Upgrades can be done only under `Root` origin, but cannot be `single signature`. And yet we must be able to release very fast, for examples in case of hotfix.

- Upgrades are not immediate because runtime should be copied to parachain collators and relay validators and be enabled simultaneously on a specific block in the future.

- There should be alignment of stakeholders on quality of release to be sure that bridged Picasso assets are secure.

## Legend

***represent on chain things*** which can be interacted via Polkadot.js or Composable parachain SDK.

## Flow

### Overview of flow

Release notes collected, specifically `git commits` which must be included in the release, are traceable to projects or pull request `items`.

Relevant notes and references to runtime build artifact (`wasm`) are shared in the Composable Finance channel with relevant representatives(`shareholders`).
Representatives agree with this release.

The release is stored as ***preimage*** on Picasso. Preimage `hash` is shared in a channel with ***council*** representatives.

Council creates default YES ***motion*** to vote on the enactment of preimage on ***democracy***.
***technicalCollective*** fast tracks enactment of new runtime.

Each step executed here is mapping teams to roles they play in the process.

#### Overview of actors

| role                     | groups of people                                                  | area                                                         |
| ------------------------ | ----------------------------------------------------------------- | ------------------------------------------------------------ |
| release engineers        | `@ComposableFi/sre`                                               |                                                              |
| integration stakeholders | not direct contributors                                           | depend on runtime UI/Frontend/Bots/Explorer/Data integration |
| council                  | ***council***(on chain list of keys with attached ***identity***) | tokenomics, marketing, PICA holders                          |
| technical                | ***technicalCollective***                                         | speed up on chain changes                                    |

#### Overview of core quality gates

Minimal set of gates:

- Well prepared `git commit`:
  - Well know commit passed all automated gates on protected release branch
    - Hot fix must pass the gate too, no `force push` release
  - All changes since previous upgrade are summarized (see Acala release for examples)
- All changes to runtime are forward compatible.
  - If extrinsic was added, it is retained and new versions are added with larger dispatch identifiers.
  - If storage was added, it is migrated by relevant runtime code on upgrade to the new version.
    - Storage is not upgraded in place as there are consumers which may not be able to read it after it is changed. 
    - The same relates to events.
- The runtime was upgraded on Dali Rococo Testnet and produced blocks here for several hours.
- All pallets included in Picasso runtime have been audited sufficiently.
- Reasonable representatives of stakeholders approved the release to Picasso.

### Collect release notes

If one wants to release changes to runtime, he shares merged pull requests or `done` project items linked to git commits with `release engineers`.
That work is done by  `@ComposableFi/developers` or product owners, or anybody qualified.
Only commits from `main protected branch` are accepted.

`Release engineers` confirm proper receipt of the proposed changes or request more information.

### Alignment kick-off

Release engineers produce `wasm` from the commit of the protected branch, which contains all relevant commits considered for inclusion in the Picasso upgrade.

After some preliminary checks and light alignment,
`release engineers` upgrade runtime on Dali Rococo Testnet using `sudo` key.

Release engineers share relevant summary of all accepted commits(`release notes`), `wasm` references and how to access runs of new runtime to `stakeholders` via the relevant channels.

### Align

Here stakeholders act on their respective quality gates and vote to agree with the release.

The necessary agreement and the exact teams to take part, depend on the properties of the upgrade.
For example, the complexity of the upgrade or a need to hot-fix.

On top of `core quality gates` the following can be considered:

- `@ComposableFi/testers` runs a full relevant set of integrations tests on Dali Rococo Testnet(Testnet for short).
- `@ComposableFi/technical-writers` ensure that available documentation is not in conflict with the Picasso upgrade.
- `@ComposableFi/blockchain-integrations`, `@ComposableFi/bots` consider that user interface, historical data explorer and bot integrations will not be negatively impacted
- `@ComposableFi/security` consider that the runtime configuration(default values and included pallets) of Picasso is secure enough for release.

Amount and structure of alignment are not specified here but should be decided per case.

### On chain upgrade operations

See [docs](../docs/docs/internal/runtime-upgrades.md).
