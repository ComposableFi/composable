# Overview

This RFC targets to help with negative impact flaky and brittle effect gates on developers productivity.

It starts from describing what is [`flaky`](https://www.jetbrains.com/teamcity/ci-cd-guide/concepts/flaky-tests/
), `brittle` and `effect gate`.

Then it describes process to handle such tests.

Describing how to make gates not flaky and not brittle are mainly out of scope of this RFC.

Making process to handle large scale always flaky and always brittle solution (non monorepo to speak or not automated or low engineering environments) are considered is out of scope of current state. 

## Definitions

[`Effect`](https://docs.hercules-ci.com/hercules-ci/effects/) [`gate`](https://learn.microsoft.com/en-us/azure/devops/pipelines/release/approvals/gates?view=azure-devops) is linter checks and automated tests which runs as part of CI before code modification applied to shared codebase or published to shared artifacts storage. 
This RFC concentrates on automated effects gates which are usually tests. So uses check, gate and test interchangeably.

[Flaky](https://docs.gitlab.com/ee/development/testing_guide/flaky_tests.html) tests is test which fails randomly on random CI job runs without changes in codebase it depend on. 
Same gate may run several times without any changes to gate or to dependant artifacts and yet fail with network, file system, permissions or any other errors sometimes. 
While sometimes passing with no error.
 
**Example**

Integration tests fails sometimes because of timeout, sometimes because of network instability and sometimes with assertion error which is not repeatable.

[Brittle] gates fail when changes done to code they depend on because they check assumptions which are not relevant to what they are actually checking. 

**Example**

Integration test asserts change in value by asserting absolute values before and after check. Absolute values could change, while differences will not. Test will fail without any real [regression](https://en.wikipedia.org/wiki/Regression_testing).

### Deeper definitions

Flaky and brittle gates are [false positive](https://en.wikipedia.org/wiki/False_positives_and_false_negatives) gate failures.

Flaky gates are results of badly managed [non determinism](https://en.wikipedia.org/wiki/Nondeterministic_algorithm).

Brittle gates are results of sloppy and low quality(may be tactically applied) engineering practices.

## Why these are bad?

Brittle tests slow down [merge](https://mergify.com/features/merge-queue) process, make CI `red` and take time of developers to double check same failures again and again.

Flaky tests make asynchronous and yet high quality automate product delivery very hard.  Changes must be synced across parties and prevents value delivery in small pieces. That increase delivery time.

## Solutions

### For Flaky 

Flaky effect gates are disabled.

Flaky effect gates are run only on items which market with `#flaky` hashtag or `flaky` label.

Later such gates are enabled again if there was fix for making them less flaky.

Deciding if test is flaky or complex regression out of scope of this RFC. 

https://www.guru99.com/positive-and-negative-testing.html

**Examples**

Integration tests

- Depend flaky test only after non flaky run (example, integrations tests run only after basic performance tests or infrastructure liveness checks )
- Temporal condition is changed to be event based(logic) condition.
- Increase hardware resources provided to test (faster tests run)

### For brittle

Brittle tests is `skipped` with reference to failures to unblock merge queue. 

These should be fixed is an agreeably less brittle way to avoid failure repetition.

**Example**


before after,

invariant of amount and relation

There are many other ways to make tests less brittle. 

Name a few without reference:

- Invariant (we test)


- Tests hierarchies by the complexity of setup and assertions
- Positive/negative testing (https://www.guru99.com/positive-and-negative-testing.html)
- Refactoring to reuse parts
- Constraints(relations) testing (we tests)
- Property test ()
- Types (impossible states will not compile)

These approaches not only make tests less brittle, but less amount of test code delivers more quality assurance with less work.