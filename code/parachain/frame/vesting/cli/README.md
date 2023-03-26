# Overview

Purpose of this tool to allow automate vesting operations.

All operations use milliseconds. Timestamp are milliseconds since [Unix Time epoch start](https://en.wikipedia.org/wiki/Unix_time).

Amounts are in raw values of maximal precision.

## Operations examples

[Create vested transfer](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A8000#/extrinsics/decode/0x0201390100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0060bc49cd808f02cd36347be7274397215a17eab56c9b559d2f5f501fbc4099530100000000000000000000000000000000000c41748801000000c87e9a000000001800000013004cc96aec63b3030000)

[Delete vested transfer](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A8000#/extrinsics/decode/0x020123020839020022123bdec5e64df1cd427e96f7e72f67c1dd25682b5503d56aeff4a606662c31010000000000000000000000000000000006020022123bdec5e64df1cd427e96f7e72f67c1dd25682b5503d56aeff4a606662c3100b8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c8434113004059be6f7c40030000)

Update.

[Claim](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A8000#/extrinsics/decode/0x39000100000000000000000000000000000000)

## How it works?

It takes data from spreadsheet or just shell input, generates relevant transactions, dry runs them, and outputs spreadsheet and/or extrinsic. 
Extrinsic can be run in Polkadotjs or via subcat. Batched or not, wrapped into sudo as neeeded.

## Internals

Use sting based subxt client, use default naming, but overridable. 
On start it downloads metadata and matches strings to SCALE. 
Code is non optimized copy pasted by implementation.
May do additional actions (e.g. seed vested accounts with ED), but executing TXes or handling other pallets will not be part of it. 

## Automated examples

```bash
export RUST_LOG=info
```


```bash
cargo run -- --client="ws://localhost:8000" add --schedule="./test/add-collators.csv" --key="//Alice" --from="5uMNuPRaGaJ6BXoys1Myi5gioCsc5dMux4A6R2dnxGPcNoHm"
cargo run -- --client="wss://picasso-rpc-lb.composablenodes.tech:443" add --schedule="./test/add-collators.csv" --key="0xff170d6075538580671f6e45f1c2701f46160dfbe57c551d01e15ecc82b8ffd3" --from="5uMNuPRaGaJ6BXoys1Myi5gioCsc5dMux4A6R2dnxGPcNoHm" --out=./test/add-collators-output.csv
cargo run -- --client="wss://picasso-rpc-lb.composablenodes.tech:443" add --schedule="./test/add-collators.csv" --key="0xff170d6075538580671f6e45f1c2701f46160dfbe57c551d01e15ecc82b8ffd3" --from="5uMNuPRaGaJ6BXoys1Myi5gioCsc5dMux4A6R2dnxGPcNoHm" --batch=true
```

```bash
cargo run -- --client="ws://localhost:8000" list --out=./test/list-output.csv
cargo run -- --client="wss://picasso-rpc-lb.composablenodes.tech:443" list --out=./test/list-output.csv
```


```bash
cargo run -- --client="wss://picasso-rpc-lb.composablenodes.tech:443" unlock --schedule="./test/clean.csv" --key="//Alice"
```

```bash
cargo run -- --client="ws://localhost:8000" delete --schedule="./test/delete-all.csv" --key="//Alice" --to="5uMNuPRaGaJ6BXoys1Myi5gioCsc5dMux4A6R2dnxGPcNoHm"
cargo run -- --client="wss://picasso-rpc-lb.composablenodes.tech:443" delete --schedule="./test/delete-all.csv" --key="//Alice" --to="5uMNuPRaGaJ6BXoys1Myi5gioCsc5dMux4A6R2dnxGPcNoHm"
```


```bash
cargo run -- --client="wss://picasso-rpc-lb.composablenodes.tech:443" add --schedule="./test/add-collators-emissions.csv" --key="0xff170d6075538580671f6e45f1c2701f46160dfbe57c551d01e15ecc82b8ffd3" --from="5uMNuPRaGaJ6BXoys1Myi5gioCsc5dMux4A6R2dnxGPcNoHm" --out=./test/add-collators-emissions-output.csv
cargo run -- --client="wss://picasso-rpc-lb.composablenodes.tech:443" add --schedule="./test/add-collators-emissions.csv" --key="0xff170d6075538580671f6e45f1c2701f46160dfbe57c551d01e15ecc82b8ffd3" --from="5uMNuPRaGaJ6BXoys1Myi5gioCsc5dMux4A6R2dnxGPcNoHm" --batch=true
```