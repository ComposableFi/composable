# Releasing

This repository uses a branch-based strategy to manage releases.

> **Note**
> This strategy has been introduced later on in this repository's lifetime, so not all releases have an associated branch.

## Naming scheme

A release branch is identified as follows:

`release-{VERSION}{CANDIDATE}`

where `VERSION` is a semantic version in the form of `vx.y.z`, and `CANDIDATE` is the optional `-rc` tag, indicating that it is a release candidate.

- `release-v3.2.1-rc` is the candidate to become `release-v3.2.1`.

## Creating a release

1. Create a PR to the branch of `release-vx.y.z-rc` from `main`, where the version is the next semantic version of the repository depending on changes.

```
release-vx.y.z-rc <- main
```

2. Automation will test the release candidate and run integration tests for this release specifically.

    - If fixes need to be applied to this release branch; PR the fixes to main, and cherry-pick the commits into the release candidate.

3. Once the release candidate has been approved by QA and auditors, create a PR from the candidate to the actual release:

```
release-vx.y.z <- release-vx.y.z-rc
```

Automation will test this PR again, although we do not expect any failures at this point.

4. Once `3.` is merged, automation will generate artifacts and create a Github release.

## Backporting fixes

If bug fixes need to be backported to previous releases, a PR through main is always required. Meaning that if we want to fix 
`release-vx.y.1` and `release-vx.y.2`, we:

1. PR the fix into main.
2. Create a PR to cherry-pick the fix into `release-vx.y.1`.
    - `release-vx.y.1 <- main`
3. Create a PR to cherry-pick the fix into `release-vx.y.2`.
    - `release-vx.y.2 <- main`