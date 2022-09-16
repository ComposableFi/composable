# Asset Registry Contract

Asset registry is used by XCVM interpreter to get the contract address of a given asset.

Asset mapping can be updated by using `SetAssets(BTreeMap<String, String>)` execute message where keys are asset id's(which must be valid `u32` integers) and values are contract addresses.

A contract address can be queried by using `GetAssetContract(u32)` where `u32` is an asset id.

## Compile

```sh
RUSTFLAGS='-C link-arg=-s' cargo b --package=xcvm-asset-registry --target=wasm32-unknown-unknown --profile="cosmwasm-contracts"
```

* `-C link-arg=-s` is used for stripping the binary which reduces the binary size drastically.
* `--profile="cosmwasm-contracts"` must be used for cosmwasm contracts.

## Test

```sh
cargo test --package="xcvm-asset-registry"
```
