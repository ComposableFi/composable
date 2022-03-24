# Parachain utils

A set of commonly needed operations to maintain a parachain.

### Runtime Upgrades

```bash
➜ ./target/release/parachain-utils --help
parachain-utils 0.1.0
The command options

USAGE:
    parachain-utils --chain-id <chain-id> --root-key <root-key> --rpc-ws-url <rpc-ws-url> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --chain-id <chain-id>        Chain id [`picasso`, `composable` or `dali`]
        --root-key <root-key>        Root key used to sign transactions
        --rpc-ws-url <rpc-ws-url>    ws url of the node to query and send extrinsics to eg
                                     wss://rpc.composablefinance.ninja (for dali-rocococ)

SUBCOMMANDS:
    help               Prints this message or the help of the given subcommand(s)
    upgrade-runtime

```

In order to perform a runtime upgrade on any chain, you need to supply a few parameters

eg

```bash
parachain-utils --chain-id=dali --root-key=0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a --rpc-ws-url=wss://rpc.composablefinance.ninja upgrade-runtime --path=./dali_runtime.compact.compressed.wasm
```

