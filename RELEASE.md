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

1. Create a PR to the branch of `main` from `release-vx.y.z-rc`, where the version is the next semantic version of the repository depending on changes.

```
main <- release-vx.y.z-rc
```

2. Automation will test the release candidate and run integration tests for this release specifically.
3. Once the release candidate has been approved by QA and auditors, create a PR the actual release branch.

```
release-vx.y.z <- release-vx.y.z-rc
```

4. Once `3.` is created, automation will generate artifacts and create a Github release.

## Backporting fixes

If bug fixes need to be backported to previous releases, a PR through main is always required. Meaning that if we want to fix
`release-vx.y.1` and `release-vx.y.2`, we:

1. PR the fix into main.
2. Create a PR to cherry-pick the fix into `release-vx.y.1`.
    - `release-vx.y.1 <- main`
3. Create a PR to cherry-pick the fix into `release-vx.y.2`.
    - `release-vx.y.2 <- main`

## Security

Tip of the branch from which runtime release is considered should be signed by at least 2 keys owned by Technical committee or by Council members.

These to be owned by Technical committee or by Council members.

### Examples

```shell
 # Alice and Bob are signers
 # Charlie is verifier
 
 # All
 git switch release-vx.y.3
 
 # Alice signs
 git tag --sign "release-vx.y.3/alice" --message "release"
 
 # Bob signs
 git tag --sign "release-vx.y.3/bob" --message "release"
 
 # Charlie looks that each tag is signed and references relevant commit
 git tag --list | grep release-vx.y.3 | xargs git tag --verify 
 ```
