# Releasing Composable Node and Related Components

1. Define release numbering convention.
2. Define branching strategy for releases.
3. Define release process steps.

## 3. Method

### 3.1. Release Numbers

Each composable node release involves a release of (at least) the following components,

1. Runtime wasm - Picasso and Composable at the time of this writing.

    In order to allow clear identification with the native runtime version of each node release the runtime version for each runtime is an integer that is monotonic increasing for each release.
2. The Composable Node - main node executable.
   This is in the format `vMajor.Minor.Patch` (eg: `v5.4200.10`). Where `Major=Branch number eg: 5`, `Minor=Runtime spec_version eg: 4200` and `Patch=a patch version 0..n`. The major version number ensures that there is always a way to branch the code for an upcoming release with relevant feature flags etc., while also serving as the major version for that release. Minor version always serves to indicate a backwards compatible patch to that release.

Release template name is:
`<component name>-<node v.major.minor version>-<component specific versioning>`

#### 3.1.1. Expected Typical Release Artifact List 

```
- Composable Node v5.4200.12
-- Runtimes
--- picasso-4200
--- composable-4200
```

### 3.2. Release Process

Typical Composable releases involve multiple rounds of QA and external audits/testing that may cause multiple patch(rc) versions to be released based on feedback/issues. This means that a release branch may have a longer maintenance life cycle independent of the main branch where most of the bleeding edge development happens. In order to execute this expected workflow, the following release process steps are proposed.

As the work starts for a `vMajor` (eg: v5) release,

1. Create a branch `release-v5`.
2. in order to make/deploy (in staging) a release create a tag `release-v5.4200.0` from the previously created branch, which should trigger a workflow.
3. QA/Audit happens on these released tag.
4. Any reported issues must be fixed on `main` and merged/cherry picked to the `release-v5` branch. Then a tag should be created for the next round and so on until "release-able" version is found.
5. Node and runtimes are released together from the same tag while other components(eg: fe) must have their own tag/workflows to release.

## 4. Implementation

The following section lays out the release steps for each release in a checklist form.

### 4.1. Understand

- [ ] List updates to each runtime together with their audit reports since the last runtime upgrade to each of them.
- [ ] List updates to the node codebase.
- [ ] List upgrades to main dependencies such as substrate, core substrate pallets, ORML etc.

### 4.2. Verify

- [ ] Storage/logic migrations from the previous versions have been removed.
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
- [ ] Update relevant frame pallets being released in runtimes to the latest node version
- [ ] Consider and list possible proxy filter updates for available calls.
- [ ] Categorize (and give a title) the release according to the types of changes it does, eg: security patch, bugfix, feature etc.
- [ ] Update the `v<Major>` branch on Github and make PR to master. Get it merged to main before the next step.
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
