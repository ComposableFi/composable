# Vest

```bash
export RUST_LOG=info
cargo +nightly run -- --client="ws://localhost:9988" --schedule="./test/test.csv" --key="//Alice" --from="5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" 
cargo +nightly run -- --client="wss://picasso-rpc.composable.finance:443" --schedule="reviewed.csv" --key="0xff170d6075538580671f6e45f1c2701f46160dfbe57c551d01e15ecc82b8ffd3" --from="5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" 2>&1 | tee vesting.log
```