# Running Checks
_Cut your check-fix feedback-loop by not waiting on CI runners._

---

We've all been there. You're working on a PR, submit it for review, ping your colleagues to review it on Slack, only to notice that checks are suddenly failing in CI.

The failing checks happened because it was difficult to reliably run the same checks that CI is running on your local machine. However, Nix fixes this and allows you to **run the same checks locally**.

Since checks are just packages defined by our repositories' flakes, you can efficiently run them locally.

Nix also allows you to easily format the entire repository per our formatting spec.

## Examples

### Formatting the repository

We have many different formatting checks, but instead of checking what is unformatted, you can simply run one command to format the entire repository:

```bash
nix run ".#fmt"
```

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

You now know all you need to know to work on existing packages. However, if you need to introduce a new package, here's how to use Nix to [define your own packages](./defining-your-own-packages.html)!




