# Getting Started with CosmWasm

Apart from interacting with `pallet-cosmwasm`, you can also create a base CosmWasm project that you can work upon.

Before running the command, you need to install `cargo-generate`. Because the command we run generates a new project
from a template by using `cargo-generate`.

You can install it by running:
```sh
cargo install cargo-generate
```

Then run the `new` command:

```sh
ccw new --name get-started --description "Get started with CosmWasm"
```

As you can see, the tool created a project for you. Let's skim through the project structure really quickly.

```sh
tree -a
```

Output:
```
.
├── .cargo
│   └── config
├── Cargo.toml
├── .gitignore
├── README.md
└── src
    ├── bin
    │   └── schema.rs
    ├── contract.rs
    ├── error.rs
    ├── integration_tests.rs
    ├── lib.rs
    ├── msg.rs
    └── state.rs
```

There are a few notable things:
* `.cargo/config` defines a few `cargo` aliases to simplify things.
* `Cargo.toml` defines a `run-script` called `optimize` to get an optimized build which is super important
when you want to test your contracts on a chain.
* `integration_tests.rs` contains a basic integration test scenario with `cosmwasm-orchestrate`. It is our great
testing and simulation tool which will cover pretty much all your needs and scenarios.

## Build the contract

You can build the contract by running:

```sh
cargo wasm
```

But the thing is, this is not optimized for binary size. So the output binary size will be pretty huge. To upload
and use it in `pallet-cosmwasm`, we should get an optimized build. 

There are two ways to do this. First is to use the official rust optimizer which needs you have to `cargo-run-script`
and `docker` installed. After installing `cargo-run-script` by running `cargo install cargo-run-script` and `docker`,
run:

```sh
cargo run-script optimize
```

The second and hacky way (in case you don't want to install anything) is building the binary by enabling the binary stripping flags.

```sh
RUSTFLAGS='-C link-arg=-s' cargo build --release --lib --target wasm32-unknown-unknown
```

This is one of the commands that `optimize` uses under the hood.
