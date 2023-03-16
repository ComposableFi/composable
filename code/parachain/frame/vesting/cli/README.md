# Vest

```bash
export RUST_LOG=info
cargo run -- --client="ws://localhost:9988" add --schedule="./test/input.csv" --key="//Alice" --from="5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" 2>&1 | tee vesting.log
cargo run -- --client="wss://picasso-rpc-lb.composablenodes.tech:443" add --schedule="./test/input.csv" --key="0xff170d6075538580671f6e45f1c2701f46160dfbe57c551d01e15ecc82b8ffd3" --from="5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" 2>&1 | tee vesting.log
```

```bash
cargo run -- --client="ws://localhost:9988" list > vestingSchedules.log
cargo run -- --client="wss://picasso-rpc-lb.composablenodes.tech:443" list > vestingSchedules.log
```


```bash
cargo run -- --client="wss://picasso-rpc-lb.composablenodes.tech:443" clean --schedule="./test/clean.csv" --key="//Alice"
```
