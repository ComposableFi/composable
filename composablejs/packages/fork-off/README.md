# Picasso Fork off substrate

This is based on [fork-off-substrate script](https://github.com/maxsam4/fork-off-substrate) to support parachain forks while also adding some modifications specific to Picasso parachain.

This script allows bootstrapping a new substrate chain with the current state of a live chain. Using this, you can create a fork of Picasso for development purposes.

## Usage

1. Install dependencies

    ```bash
    yarn
    ```

2. Create a folder called `data` inside the top folder (`fork-off`).

    ```bash
    mkdir data
    ```

3. Download and copy the executable/binary of the currently live Picasso parachain and the runtime wasm 
   listed in the [releases](https://github.com/ComposableFi/composable/releases) page to `fork-off/data` and rename it to `binary`.
   (you may have to install the deb or rpm package and extract the binary from there)

4. Download and copy the runtime WASM blob of the Picasso release(eg: `picasso_runtime_v10005.wasm`) to `fork-off/data` and rename it to `runtime.wasm`.

5. Go back to the `fork-off` package (`cd ../composablejs/packages/fork-off`) and run the main script. This will download the right Polkadot version and generate all the necessary files for the new chain.

    ```bash
    yarn start
    ```
6. On different terminals, run each of these commands:

    ```bash
   make alice
   ```
    ```bash
   make bob
   ```
    ```bash
   make charlie
   ```
    ```bash
   make collator1
   ```
   ```bash
   make collator2
   ```
   The first three will run the relay chain and the last two will run the forked parachain.


7. Download the next version of the Picasso runtime wasm from the [releases](https://github.com/ComposableFi/composable/releases/latest) page, save it in the `/data` folder, and rename it from `picasso_runtime_v****.wasm` to `runtime_upgrade.wasm`. This will be automated soon.


8. Start the runtime upgrade by running

    ```bash
    yarn run upgrade
    ```
    This can take a few minutes from the moment the chain starts producing blocks.
   Once the script is finished, the parachain should be upgraded on the next block. It will run on port `9988` and can be visualized in https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer

## Credits

This script is based on [a script shared in the substrate riot channel](https://hackmd.io/mGgNZX0VT4S0UTaq89-_SQ)


