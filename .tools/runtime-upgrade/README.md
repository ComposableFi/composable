# Runtime upgrades

This tool is used by SRE for runtime upgrades. It provides two modes of runtime upgrading: `sudo` based and `democracy`-based.

## Protocol

The tool performs the following steps in sequence:

1. Initialize a local chain using [`fork-of-substrate-fork`](https://github.com/ComposableFi/fork-off-substrate-fork#picasso-fork-off-substrate). Instructions are provided there in the readme.

2. Performs a `sudo`-based runtime upgrade against the local chain to verify that the upgrade will succeed. TODO

3. If 2 is successful, perform either a `sudo`-based upgrade or `democracy` proposal, depending on the provided flags. DONE

## Usage

```bash
# Building
docker build -t $NAME .

# Running
export WASM=/path/to/runtime.wasm
docker run --mount type=bind,source=$WASM,target=/upgrader/runtime.wasm $NAME upgrade -w  $URL -k $KEY -r "/upgrader/runtime.wasm" -m "sudo"
```

- URL has protocol `ws`: `ws://127.0.0.1:9988` for example
- KEY can be `//Alice`, 12 words or hex encoded.

To connect to localhost, you'll probably need to use host networking. This script does not need to run on a machine with a node.