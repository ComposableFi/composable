# Releasing Composable Node and Related Components

## 1. Abstract

Given the complexity of the number of components and teams involved a standard process is needed to release Composable node and related components.

## 2. Goals

1. Define release numbering convention.
2. Define release process steps.

## 3. Method

### 3.1. Release Numbers

Each composable node release involves a release of (at-least) the following components,

1. The Composable Node - main node executable

    Node follows [semver](https://semver.org/) versioning with an additional `v` prefix. Eg: `v2.3.1`. 

2. Runtime wasm - Dali, Picasso and Composable at the time of this writing.

    In order to allow clear identification with the native runtime version of each node release the runtime version for each runtime is an integer derived by concatenating all numbers in the node version and adding to 10000. Eg: `v2.5.1` -> `251` -> 251 + 10000 -> (`dali-10251`, `picasso-10251`, `composable-10251`). The 10000 addition is to prevent collision of version numbers of this scheme with earlier versions before the scheme was introduced.

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
- Composable Node v2.5.3
-- Runtimes
--- dali-10253
--- picasso-10253
--- composable-10253
-- FE
--- fe-v2.5-picasso-abcd
--- fe-v2.5-pablo-xyz
-- ComposableJS
--- composablejs-v2.5-mnop
-- Subsquid
--- subsquid-v2.5-111
```

### 3.2. Release Process

Typical Composable releases involve multiple rounds of QA and external audits/testing that may cause multiple patch(rc) versions to be released based on feedback/issues. This means that a release branch may have a longer maintenance life cycle independent of the main branch where most of the bleeding edge development happens. In order to execute this expected workflow, following release process steps are proposed.

As the work starts for a `major.minor` (eg: 2.5) release,

1. Create a branch `release-v2.5`.
2. in order to make/deploy (in staging) a release create a tag `release-v2.5.0` which should trigger a workflow.
3. QA/Audit happens on these released tag.
4. Any reported issues must be fixed on `main` and merged/cherry picked to the `release-v2.5` branch. Then a tag should be created for the next round and so on until "release-able" version is found.
5. Node and runtimes are release together from the same tag while other components(eg: fe) must have their own tag/workflows to release.
