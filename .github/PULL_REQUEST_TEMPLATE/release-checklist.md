# Release Checklist

## Understand

In the PR description,

- [ ] List updates to each runtime together with their audit reports since the last runtime upgrade to each of them.
- [ ] List updates to the node codebase.
- [ ] List upgrades to main dependencies such as substrate, core substrate pallets, ORML etc.

## Verify

- [ ] Storage/logic migrations from the previous versions has been removed.
- [ ] Make sure proper logic/storage migrations are included as necessary
- [ ] Verify documentation has been updated.

## Action

- [ ] Generate weights, i.e run `benchmark`
- [ ] Runtime [versioning](https://docs.substrate.io/build/upgrade-the-runtime/) updates
  - [ ] Update `spec_version` (automate-able)
  - [ ] Update `transaction_version` if existing extrinsics or their ordering has changed. Can be verified via [metadata comparison](https://github.com/paritytech/polkadot/blob/master/doc/release-checklist.md#extrinsic-ordering).
- [ ] Update composable node version if the code has changed
- [ ] Update composableJs version (if necessary to be released)
- [ ] Update FE version (if necessary to be released)
- [ ] Update Subsquid version (if necessary to be released)
- [ ] Consider and list possible proxy filter updates for available calls.
- [ ] Categorize (and give a title) the release according to the types of changes it does, eg: security patch, bugfix, feature etc.
