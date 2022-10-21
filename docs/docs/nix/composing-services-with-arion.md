# Composing Services with Arion
*Services together strong*

---

Sometimes, a single package is not enough. Usually, we use something like `docker compose` to combine containerized services. However, we want our orchestrations to be as **reproducible**, **declarative**, and **reliable** as our single packages. To achieve this, we use [Arion](https://docs.hercules-ci.com/arion/), a `nix` wrapper around `docker compose`, to define our service compositions.

An example is the `devnet-xcvm` package inside the main composable repo.

