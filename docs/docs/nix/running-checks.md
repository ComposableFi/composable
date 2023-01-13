# Running Checks

*Cut your check-fix feedback-loop by not waiting on CI runners.*

We've all been there. You're working on a PR, submit it for review, ping your colleagues to review it on Slack, only to notice that checks are suddenly failing in CI.

The failing checks happened because it was difficult to reliably run the same checks that CI is running on your local machine. However, Nix fixes this and allows you to **run the same checks locally**.

Since checks are just packages defined by our repositories' flakes, you can efficiently run them locally.

Nix also allows you to easily format the entire repository per our formatting spec.

## Formatting the repository

We have many different formatting checks, but instead of checking what is unformatted, you can simply run one command to format the entire repository:

```bash
nix run ".#fmt"
```

## Running all checks

This check runs most of the checks that CI runs. Note that running these checks for the first time will take very long as nothing is cached yet. After that, checks should be faster.

```bash
nix run ".#check"
```

## Running individual checks

It is also possible to run individual checks, here are some examples:

### Checking your spelling

```bash
nix build ".#spell-check"
```

### Clippy errors

```bash
nix build ".#cargo-clippy-check"
```

### Dali integration tests

```bash
nix build ".#check-dali-integration-tests"
```

### All others

There are many more checks we can run. If you want to reproduce one from CI locally, you can check `nix flake show` and the workflows defined under `.github/` to see which `nix` command they are invoking.

---

You now know all you need to know to work on existing packages. However, if you need to introduce a new package, here's how to use Nix to [define your own packages](./defining-your-own-packages)!
