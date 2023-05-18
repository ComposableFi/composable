# Releasing Composable Node and Related Components

1. Define release numbering convention.
2. Define branching strategy for releases.
3. Define release process steps.

## 3. Method

### 3.1. Release Numbers

Each composable node release involves a release of (at-least) the following components,

1. Runtime wasm - Dali, Picasso and Composable at the time of this writing.

    In order to allow clear identification with the native runtime version of each node release the runtime version for each runtime is an integer that is monotonic increasing for each release.
2. The Composable Node - main node executable.
   This is in the format `vMajor.Minor.Patch` (eg: `v5.4200.10`). Where `Major=Branch number eg: 5`, `Minor=Runtime spec_version eg: 4200` and `Patch=a patch version 0..n`. The major version number ensures that there is always a way to branch the code for an upcoming release with relevant feature flags etc., while also serving as the major version for that release. Minor version always serves to indicate a backwards compatible patch to that release.
3. Subsquid - Data archival and query system for networks.
4. Frontends - There are two FE components in existence at the time of this writing.
5. ComposableJs - This is the library to interact with composable parachains using typescript/JS.

For 3, 4, 5 have the following characteristics which require them to be versioned in a different scheme,
- They all depend on the node rpc and runtime (and types) directly hence requiring updates whenever the node RPC/event interfaces change.
- They all can make breaking changes from the point of view of external dependants such as ComposableJS which could have major versions/breaking interface changes without a direct relationship to Node/runtime changes.

Because of these characteristics following specific versioning scheme is proposed be used,

`<component name>-<node v.major.minor version>-<component specific versioning>`

Eg: 

For Picasso FE : `fe-v2.4-picasso-abcd`

For Subsquid : `subsquid-v2.5-1.0.1`

etc.

#### 3.1.1. Expected Typical Release Artifact List 

```
- Composable Node v5.4200.12
-- Runtimes
--- dali-4200
--- picasso-4200
--- composable-4200
-- FE
--- fe-v5.4200-picasso-abcd
--- fe-v5.4200-pablo-xyz
-- ComposableJS
--- composablejs-v5.4200-mnop
-- Subsquid
--- subsquid-v5.4200-111
```

### 3.2. Release Process

Typical Composable releases involve multiple rounds of QA and external audits/testing that may cause multiple patch(rc) versions to be released based on feedback/issues. This means that a release branch may have a longer maintenance life cycle independent of the main branch where most of the bleeding edge development happens. In order to execute this expected workflow, following release process steps are proposed.

As the work starts for a `vMajor` (eg: v5) release,

1. Create a branch `release-v5`.
2. in order to make/deploy (in staging) a release create a tag `release-v5.4200.0` from the previously created branch, which should trigger a workflow.
3. QA/Audit happens on these released tag.
4. Any reported issues must be fixed on `main` and merged/cherry picked to the `release-v5` branch. Then a tag should be created for the next round and so on until "release-able" version is found.
5. Node and runtimes are released together from the same tag while other components(eg: fe) must have their own tag/workflows to release.

#### 3.2.1 Frontend releases:
Frontend releases follows the above guideline, with a few rules:
- Versioning and Triggers:
Create a tag `staging-fe-v[MAJOR].[MINOR]-picasso-[FE_VERSION_NUMBER]`
- To deploy to production, the approved commit should be tagged with: `fe-v[MAJOR].[MINOR]-picasso-[FE_VERSION_NUMBER]`

> Note: [MAJOR] and [MINOR] follows the convention of the branch.

**[FE_VERSION_NUMBER] is a number incremented on each release.**

## 4. Implementation

The following section lays out the release steps for each release in a checklist form.

### 4.1. Understand

- [ ] List updates to each runtime together with their audit reports since the last runtime upgrade to each of them.
- [ ] List updates to the node codebase.
- [ ] List upgrades to main dependencies such as substrate, core substrate pallets, ORML etc.

### 4.2. Verify

- [ ] Storage/logic migrations from the previous versions has been removed.
- [ ] Make sure proper logic/storage migrations are included as necessary
- [ ] Verify documentation has been updated.
- [ ] Verify that there are no critical update instructions in release notes from Substrate/Cumulus/Polkadot releases that may not have been taken into account.

### 4.3. Act

- [ ] If it is the first release of the `v<Major>` (eg: v5) line then create a branch `release-v<Major>`. Execute the following steps on that branch.
- [ ] Generate weights, i.e run `benchmark`
- [ ] Bump numbers in runtimes `version.rs` [according rules](https://docs.substrate.io/maintain/runtime-upgrades/) and 
   - [ ] Update `spec_version` (automate-able)
   - [ ] Update `transaction_version` if existing extrinsics or their ordering has changed. Can be verified via [metadata comparison](https://github.com/paritytech/polkadot/blob/master/doc/release-checklist.md#extrinsic-ordering).
- [ ] Update composable node version if the code has changed (relevant number in workspace `Cargo.toml`)
- [ ] Update composableJs version (if necessary to be released)
- [ ] Update FE version (if necessary to be released)
- [ ] Update Subsquid version (if necessary to be released)
- [ ] Update relevant frame pallets being released in runtimes to the latest node version
- [ ] Consider and list possible proxy filter updates for available calls.
- [ ] Categorize (and give a title) the release according to the types of changes it does, eg: security patch, bugfix, feature etc.
- [ ] Update the `v<Major>` branch on Github and make PR to master. Get it merged to main before next step.
- [ ] Get the tip of the branch signed according to 3.2.4 section.
- [ ] Finally, create a tag `v<Branch.Spec_version.Path>` (eg: `v5.4201.0`) to trigger the release artifact build.

```shell
nix run .\#tag-release 5.4200.0
```

It will trigger `on.push.tags` [GitHub Tag based release flow](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases) 

- [ ] Run [upgrade](https://substrate.stackexchange.com/questions/1061/what-is-the-proper-way-of-executing-a-runtime-upgrade-on-a-parachain) to new runtime on fork of latest block of live network.


## Notifications

- GitHub allows to subscribe to Release Candidate and Release publish.
- Web3Alerts.io allows to subscribe to parachain runtime update authorized, runtime uploaded and runtime applied.

These notification allow integrations (bridges, frontend, indexers) to align their roll outs with runtime upgrades.