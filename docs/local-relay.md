# Overview

Allows to run Kusama relay + Dali parachain + Hydra paracahin via 

```
cargo make devnet-docker
```
URLs:
* https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer is the 1st Relay Chain node
* https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer is the 1st Composable's collator
* https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9998#/explorer is the 1st Basilisk's collator
  
Build via sandbox docker job in Actions into latest and git hash.
