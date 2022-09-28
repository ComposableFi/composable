# Composing Services with Arion
*Services together strong*

---

Sometimes, a single package is not enough. When we need to compose many services together, we would usually use something like `docker compose` in order to compose many containerized services together. However, we want our orchestrations to be as **reproducible**, **declarative**, and **reliable** as our single packages. This is why we use [Arion](https://docs.hercules-ci.com/arion/), a `nix` wrapper around `docker compose`, in order to define our service compositions.

An example of this is the `devnet-xcvm` package inside of the main composable repo.

