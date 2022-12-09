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

3. Build the executable/binary of the currently live Picasso parachain and the runtime wasm. For `picasso-1400` runtime onwards these are listed in the [releases](https://github.com/ComposableFi/composable/releases) page.

    You can build the binary with built-in wasm for the given version by going into the `/code` folder and running the following:

    ```bash
   # go to the /code folder
   cd ../../../code

   # check out the tag, eg: release-v1.10002
    git checkout release-v1.10002

    cargo +nightly build --release -p wasm-optimizer
    cargo +nightly build --release -p dali-runtime-wasm --target wasm32-unknown-unknown
    cargo +nightly build --release -p picasso-runtime-wasm --target wasm32-unknown-unknown
    cargo +nightly build --release -p composable-runtime-wasm --target wasm32-unknown-unknown
   ./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/dali_runtime.wasm --output ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm
   ./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/picasso_runtime.wasm --output ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm
   ./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/composable_runtime.wasm --output ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm

    export DALI_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm) && \
   export PICASSO_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm) && \
   export COMPOSABLE_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm) && \
   cargo build --release --package composable --features=builtin-wasm
    ```

4. Copy the binary from `target/release/composable` to `fork-off/data` and rename it to `binary`.

    ```bash
     cp ./target/release/composable ../composablejs/packages/fork-off/data/binary
    ```

5. Copy the runtime WASM blob of the Picasso release from `/target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm` to `fork-off/data` and rename it to `runtime.wasm`.
    ```bash
    cp ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm ../composablejs/packages/fork-off/data/runtime.wasm
   ```

6. Checkout your working branch

7. Go back to the `fork-off` package (`cd ../composablejs/packages/fork-off`) and run the main script. This will download the right Polkadot version and generate all the necessary files for the new chain.

    ```bash
    yarn start
    ```

8. On different terminals, run each of these commands:

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
   make collator
   ```
   The first 3 will run the relay chain and the last one will run the forked parachain.

9. Register the parachain on the relay chain. This can be done manually or by running the following on a new terminal:

    ```bash
    yarn run register
    ```
   Once the script is finished, it should take around 2 minutes for the parachain to be registered. It will run on port `9988` and can be visualized in https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer

### Optional

For manual registration, this extrinsic should be run as sudo from the relay chain at https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944 and with a `ParaId` of 2087.

   ![image](https://user-images.githubusercontent.com/2915325/99548884-1be13580-2987-11eb-9a8b-20be658d34f9.png)

## Credits

This script is based on [a script shared in the substrate riot channel](https://hackmd.io/mGgNZX0VT4S0UTaq89-_SQ)


