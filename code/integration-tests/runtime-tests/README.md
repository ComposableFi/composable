# Priorities

1. No false negatives. Such tests to be deleted (see rfc about brittle and flaky tests)
2. Time to merge into main. Should be fast.
3. Super low maintainance. 20% of tests for critical paths cover 80% of failures. Easy to find why in the internet why. Use same tools as ops and devs.
4. No bugs into main merged.
5. Readability of tests output. Easy to reproduce tests locally and doing smoke tests in repeatable way so can just eyeball what wrong.
