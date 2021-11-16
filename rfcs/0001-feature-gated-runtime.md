# Summary

This proposes a solution to maintain several development streams and runtimes in a single branch.

## Motivation

There is an approach to developing software in branches, which we try to follow.
However, I argue that this complicates things in the current state and makes our progress slower and more brittle.

I suggest using a structured conditional compilation of runtime to achieve several streams of development without losing the security qualities of the codebase.
That approach fits very nicely with Rust and Substrate ecosystems.

The suggested approach will make our development easier until we release it onto Polkadot.

## Detailed design

Runtime is statically compiled. It means that if a pallet is coded but and not added to runtime, a pallet cannot influence runtime in any way.
From this, a pallet under development does not represent a security issue.

### Runtime gating

See usage of 'feature = "develop"` in runtime.

### IDE support

Rust analyzer works well with features behind `rust-analyzer.cargo.features` configuration in VS Code. It is visible what features are on and off.

### Cross pallet functional

Useful functional may consist of several interacting pallets. Can develop and on them one by one behind the gate.
As soon as the functional is ready, can remove develop gate from related pallets.
This will make our project tracking software more manageable, too, by avoiding long-running tasks in branches.
The shorter task is, the more predictable it is, and less friction is added during late integration.

### Shared ownership

As soon as code appears into the main branch, approved by our peer, each of us is the owner of that code.
Extensive functional development is penalized, and significant refactoring is impossible under the risk of hard-to-merge issues.
If one has a prominent feature on develop, one can get stuck in the long cycle to keep it up to date.
Code runaway happens for both in code and in CI/CD in branch-based approach.

Why would others keep up today's feature which is under develop?
Because company agreed that that feature is valuable and should be developed the way it is.
If a decision will change, the code could be deleted, which is easy to do.

## Alternatives

Using develop branch is an attempted alternative to go forward. I see next problems with it as of now:

- When I need code in main, which is in develop, and vice versa, I cannot have it instantly as required as by dependency of my task.
- There are no strict guidelines on what should go to the main and what should go into development.
- Merging main into develop and develop back is hard and error-prone. Can introduce security bugs.
- Develop is not up today with the same GitOps and linting practices with main. This leads to broken and hard-to-fix build issues, like the inability to build runtime on current develop.

## Unresolved questions

- Gating new yet audited extrinsic in existing pallets.
- Versioning conditional blocks of code inside pallets.
- https://github.com/paritytech/substrate/issues/10286

Two of the above issues should be solved regardless of whether we are using develop branch or feature gate.