# Hyperspace Relayer


## CLI

**Running the setup step**

In order to generate some code to connect to the substrate nodes in different networks,
the CLI has the `network-setup` command. It expects a list of `json` parameters with the
following shape
```json
{
    "network": String,
    "url": String
}
```

The command, with the input, can be invoked:
```sh
./hyperspace network-setup \
    --input '{"url": "ws://127.0.0.1:9944", "network": "polkadot"}' '{"url": "ws://127.0.0.1:9188", "network": "parachain"}' \
    --source_path ${OUT_DIR}
    --destination-path 'code/centauri/hyperspace/parachain/src/parachain.rs'
````