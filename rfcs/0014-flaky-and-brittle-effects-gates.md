# Overview

This RFC targets to help with the negative impact of flaky and brittle effect gates on developers' productivity.

It starts by describing what is [`flaky`](https://www.jetbrains.com/teamcity/ci-cd-guide/concepts/flaky-tests/
), `brittle`, and `effect gate`.

Then it describes the process to handle such tests.

Describing all engineering ways to make gates not flaky and not brittle is mainly out of the scope of this RFC.

Making a process to handle a large-scale flaky brittle solution (not monorepo or not automated or low engineering environments) is out of the scope of the current RFC. 

## Definitions

[`Effect`](https://docs.hercules-ci.com/hercules-ci/effects/) [`gate`](https://learn.microsoft.com/en-us/azure/devops/pipelines/release/approvals/gates?view=azure-devops) is linter checks 
and automated tests which run as part of CI before code modification applied to a shared codebase or published to shared artifacts storage(deployed). 
This RFC concentrates on automated effects gates which are usually tests. It uses check, gate, and test interchangeably.

[Flaky](https://docs.gitlab.com/ee/development/testing_guide/flaky_tests.html) test
is a test that fails randomly on random CI job runs without (breaking) changes in the codebase it depends on. 
A flaky gate may run several times without any changes to the gate or dependant artifacts
and yet fail with a network, file system, permissions, or any other errors sometimes. 
While sometimes passing with no error.
 
**Example**

> An integration test fail sometimes because of a timeout, sometimes because of network instability in logs (HTTP/TPC/CURL/etc. errors) and sometimes with assertion error which is not repeatable.

[Brittle](https://softwareengineering.stackexchange.com/questions/356236/definition-of-brittle-unit-tests) gates fail when changes were done to the code they depend on
because they check assumptions that are not relevant to what they are actually checking, and these assumptions are wrong.

**Example**

> An integration test asserts change in value by asserting absolute values before and after action. An absolute value could change, while differences will not. The gate will fail without any [regression](https://en.wikipedia.org/wiki/Regression_testing).

### Deeper definitions

Flaky and brittle gates are [false positive](https://en.wikipedia.org/wiki/False_positives_and_false_negatives) gate failures.

Flaky gates happen as consequence of a improper managed [non determinism](https://en.wikipedia.org/wiki/Nondeterministic_algorithm).

Brittle gates happen as consequences sloppy and low quality(may be tactically reasoanble) engineering practices.

## Why flaky and brittle gates are bad?

Brittle tests slow down [merge](https://mergify.com/features/merge-queue) process, make CI `red`, and cosume development resoruces to double check same failures repeatedly. 

Flaky tests make asynchronous and yet high quality automate product delivery very hard.  All changes must be synced across parties, that prevents value delivery in small pieces. Delivery time increased.

Both lead to [alert fatigue](https://en.wikipedia.org/wiki/Alarm_fatigue) , which leads to real issues and regression are ingnored. A quality of output decreases. Resources spent on automation wasted.

## Solutions

### For Flaky 

Flaky effect gates are disabled by default in CI pipeline.

Flaky effect gates are run only on items which market with `#flaky` hashtag or `flaky` label.

Later such gates are enabled again if there was an attempt to fix them.

Deciding if test is flaky or complex regression out of scope of this RFC. 


Possible things to do with flaky tests:

- Depend flaky test only on success after non flaky run. For examples, integrations tests are se to run only after basic performance or infrastructure liveness checks.
- Temporal conditio(wait some time) is changed to event based(data presence) condition.
- Increase hardware resources provided to test run

### For brittle

Brittle tests is `skipped` with reference to failures to unblock merge queue.

These should be fixed is an agreeably less brittle way to avoid failure repetition.

Possible things to do with brittle tests:

- Capture value before action and compare with after action, not absolute 
- Test invariants, relations, constraints and properties.
- Refactor to reuse parts, specifically toward tests hierarchies by the complexity of setup and assertions (exampl,e previos simler test becomes setup of part of more complex tests)
- [Positive/negative testing](https://www.guru99.com/positive-and-negative-testing.html)
- Types (impossible states will not compile)

These approaches not only make tests less brittle, but ensure fewer lines of a test code deliver more coverage.

## But disabling checks is bad ? 

First, we retain flaky and brittle tests to be run and be part of code base. We allow them to be compiled and run for human decision instead of being removed or commented out. So we allow there to be more tests to be here than without described approach.

Second, if you know fix and fix will not break again same way and fix easy is - do it. So disabling is formal option to align, but can solve faster if you know how and have capacity.

Third, 
If it end up to harded to asssure release quality. There could be urge to to more automated gates.  Automation would have choose either run manually gates and evaluate flaky/brittle results depending on domain knowledge, or make attempts to fix some flaky/brittle tests.