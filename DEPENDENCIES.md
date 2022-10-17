# Software Dependencies

When adding any dependency to the codebase, ensure that the policies herein described are followed. In particular, keep in mind that:

1. Adding a dependency is a security risk.
2. Adding a dependency is a legal risk.

For 2, if you are unsure of the legal implications, contact our general council through the maintainers/your team lead. 

## Adding a Dependency

Adding dependencies is usually fine, however, keep in mind the following:

- Each dependency is a security risk.
- Each dependency can randomly block updates due to not being maintained.
- Each dependency nominally increases build times.

This means that the following has to be observed when adding a dependency:

1. It must be pinned to a specific version, which cannot be modified by the dependency author.
    - For example, crates.io does not allow modifying existing versions. It is append-only.
2. The developer adding the dependency needs to audit the code for backdoors.
    - For large and widely-used codebases, the auditing portion can be dropped. Libraries such as `tokio` or `substrate` can be trusted as is.

## Git Dependencies

Sometimes, instead of using `crates.io` or the `NPM` registry, we need to use git repositories as a dependency directly. To ensure we are not open to supply chain attacks, the following needs to be observed:

1. We pin to a commit, not a branch.

Note that in the case of a git dependency, the repository can become inaccessible, due to the owner deleting it or making it private. If that is a risk, prefer forking the repository first.

## Forking Repositories

Quite often, we have to fork upstream repositories to introduce our changes. `Polkadot` is a good example, as for each `PATCH` version released by parity, breaking changes are introduced. When forking a repository and pinning to that fork, the following must be ensured:

1. The forked repository is under the `ComposableFi` organization.
2. Do not alter main, but instead create a specific `fork/${REASON}` branch containing our changes. Upstream changes can be merged/rebased.
3. CI should be enabled on the forked repository (can be very hard to accomplish, but we need to test our changes).
4. Branch protection rules must be set on the forked repository for the `fork/${REASON}` branch, ensuring all changes require 2 approvals.
5. The rules for regular git dependencies still apply.