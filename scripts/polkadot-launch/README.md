## Check

After Steps do check any URLs:
* `https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9945#/explorer` is the 1st Relay Chain node
* `https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer` is the 1st Composable's collator


# Run Composable's parachain only


Do: 

`relay.sh`

`composable.sh`


Build this project:

```bash
yarn
```

Run:

```bash
yarn rococo-dali
```
## Run Composable's and Basilisk's parachains

Do:

`relay.sh`

`composable.sh`

`basilisk.sh`

```bash
yarn
```

Run all
```bash
yarn rococo-dali-basilisk
```


## Run  Kusama relay + Dali parachain + Hydra paracahin in Docker via Polka launcher

Build via `sandbox docker` job in Actions into latest and git hash.

```
cargo make start-devnet-docker
```
URLs:
* [Relay]https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer) is the 1st Relay Chain node
* [Composable Dali](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer) is the 1st Composable's collator
* [Basilisk](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9998#/explorer) is the 1st Basilisk's collator
