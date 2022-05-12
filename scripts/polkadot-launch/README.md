## Check

After Steps do check any URLs:

* `https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9945#/explorer` is the 1st Relay Chain node
  
* `https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer` is the 1st Composable's collator (or check spec for `wsPort`).  
* !!!Wait!!! It may take time network catches up (it was 3 minutes for me) to start block production
 
 * Ensure you have enough space for network state!!!!

* By deliberate choice networks here start from clean state each time by default. If you need start from previous state, use `basePath` .

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


## References

- https://docs.substrate.io/tutorials/v3/private-network/
- https://docs.substrate.io/tutorials/v3/cumulus/start-relay/
- https://docs.substrate.io/tutorials/v3/permissioned-network/
- https://github.com/paritytech/cumulus/blob/master/polkadot-parachains/src/command.rs
- https://github.com/paritytech/polkadot-launch/blob/master/src/spawn
- https://wiki.polkadot.network/docs/learn-common-goods
- https://docs.substrate.io/tutorials/v3/cumulus/connect-parachain/
- https://github.com/paritytech/cumulus
- https://docs.substrate.io/how-to-guides/v3/basics/custom-chain-spec/