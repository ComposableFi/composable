# Vest

```bash
export RUST_LOG=info
cargo +nightly run -- --client="ws://localhost:9988" add --schedule="./test/input." --key="//Alice" --from="5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" 2>&1 | tee vesting.log
cargo +nightly run -- --client="wss://picasso-rpc-lb.composablenodes.tech:443" add --schedule="./test/input." --key="0xff170d6075538580671f6e45f1c2701f46160dfbe57c551d01e15ecc82b8ffd3" --from="5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" 2>&1 | tee vesting.log
```

```bash
cargo +nightly run -- --client="ws://localhost:9988" list > vestingSchedules.log
cargo +nightly run -- --client="wss://picasso-rpc-lb.composablenodes.tech:443" list > vestingSchedules.log
```