# initialization

## Build

    ```bash
    yarn build
    ```

## Running

    ```bash
    yarn start
    ```

## TS type generation

    ```bash
    # run a local Polkadot cluster

    wscat -c ws://localhost:9998 -x '{"id":"1", "jsonrpc":"2.0", "method": "state_getMetadata", "params":[]}' > edgeware.json

    yarn generate

    # edit src/interfaces/augment-* files
    ```
