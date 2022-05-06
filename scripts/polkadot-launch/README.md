# Run Composable's parachain only

Need to do to run 4 relay chain nodes and 1 Composable's collator:

1. build a Composable's collator

	```bash
	old_pwd=$(pwd)
	cd ../..
	cargo build --release
	target/release/composable --version
	cd "$old_pwd"
    ```

2. download a Polkadot Relay Chain

	```bash
	mkdir -p ../../../polkadot/target/release
	curl https://github.com/paritytech/polkadot/releases/download/v0.9.18-rc4/polkadot -Lo ../../../polkadot/target/release/polkadot
	../../../polkadot/target/release/polkadot --version
    ```

3. build this project

	```bash
	yarn
	```

4. run all

	```bash
	yarn composable
	```

URLs:
* `https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9945#/explorer` is the 1st Relay Chain node
* `https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer` is the 1st Composable's collator

# Run Composable's and Basilisk's parachains

Need to do to run 4 relay chain nodes, 2 Composable's collators and 2 Basilisk's collators:

1. build a Composable's collator

	```bash
	(
		cd ../..
		cargo build --release
		target/release/composable --version
	)
	```

2. download a Polkadot Relay Chain

	```bash
	mkdir -p ../../../polkadot/target/release
	curl https://github.com/paritytech/polkadot/releases/download/v0.9.18-rc4/polkadot -Lo ../../../polkadot/target/release/polkadot
	../../../polkadot/target/release/polkadot --version
    ```

3. download a Basilisk's collator

	```bash
	mkdir -p ../../../Basilisk-node/target/release
	curl https://github.com/galacticcouncil/Basilisk-node/releases/download/v7.0.0/basilisk -Lo ../../../Basilisk-node/target/release/basilisk
	chmod +x ../../../Basilisk-node/target/release/basilisk
	../../../Basilisk-node/target/release/basilisk --version
	```

4. build this project

	```bash
	yarn
	```

5. run all

	```bash
	yarn composable_and_basilisk
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
