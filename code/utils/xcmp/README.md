# Overview

Client tools.
## Example

```shell
cargo +nightly run sudo execute --suri PRIVATE_KEY_PATH --call 0x0008083432 --network composable_dali_on_parity_rococo --rpc wss://rpc.composablefinance.ninja:443
```

## TODO 
- move tools to `node` as cli.
- move para to account to lib which no reference to generated files (they hang/kill RA - hard to develop)
- allow to execute polkadot js decode reference (so you form message in pd.js and send link to and just it)
- add flag --defaults and suffix with port 443 in this case
- todo add our chains to substrate/subkey like other tools did (easier to pipe addresses)