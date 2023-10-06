# Composable Cosmos chain: Testnet 3


## Syncing materials

### Binary and config

Testnet-3 launch version: v2.3.5, precompiled binary for linux could be found here: https://github.com/notional-labs/composable-testnet/releases/tag/v2.3.5

```
cd ~/composable-testnet
git fetch --all
git checkout v2.3.5
make install
banksyd tendermint unsafe-reset-all
```

:::note
Ensure you are running the latest release, head to the [repository](https://github.com/notional-labs/composable-centauri) for the latest testnet releases.
:::

### Peers & seeds:
You can set the peers/seeds in `config.toml` or run the node with `--p2p.seeds="" --p2p.persistent_peers=""`

Feel free to PR your peers/seeds on the respective repository:

*Seeds:*
```
364b8245e72f083b0aa3e0d59b832020b66e9e9d@65.109.80.150:21500,
d2deff06cf95c0d016d8f65822e1c74ce2af9def@95.217.58.111:37656
```

### Download the genesis and start:
```
wget -O ~/.banksy/config/genesis.json https://raw.githubusercontent.com/notional-labs/composable-networks/main/banksy-testnet-3/genesis.json
banksyd start --p2p.seeds 364b8245e72f083b0aa3e0d59b832020b66e9e9d@65.109.80.150:21500
```

## Explorer

Nodestake: https://explorer.nodestake.top/composable-testnet

Ping Pub: https://testnet.ping.pub/centauritestnet
