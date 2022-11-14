# Runtime upgrades

This tool is used by SRE for runtime upgrades. It provides two modes of runtime upgrading: `sudo` based and `democracy` based.

## Protocol

The tool performs the following steps in sequence:

1. Initialize a local chain using [`fork-of-substrate`](https://github.com/maxsam4/fork-off-substrate).

2. Performs a `sudo`-based runtime upgrade against the local chain to verify that the upgrade will succeed.

3. If 2 is succesful, perform either a `sudo`-based upgrade or `democracy` proposal, depending on the provided flags.

## Usage

```bash
docker run --mount type=bind,source=/home/user/runtime.wasm,target=/upgrader/runtime.wasm upgrader upgrade -w wss://kusama.api.onfinality.io/public-ws -k "december suit acoustic cruise crystal tunnel butter piece winner crunch language engine" -r "/upgrader/runtime.wasm" -m "sudo"
```