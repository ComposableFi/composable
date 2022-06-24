# Run Composable's parachain only

Need to do to run 2 Relay Chain nodes and 1 Composable collators:

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
* [the 1st Relay Chain node](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer)
* [the 1st Composable collator](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer)

# Run Composable's and Basilisk's parachains

Need to do to run 5 Relay Chain nodes, 2 Composable collators and 2 Basilisk collators:

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
	curl https://github.com/galacticcouncil/Basilisk-node/releases/download/v8.0.0/basilisk -Lo ../../../basilisk-node/target/release/basilisk
	chmod +x ../../../basilisk-node/target/release/basilisk
	../../../basilisk-node/target/release/basilisk --version
	```

4. build this project

	```bash
	yarn
	```

5. run all

	```bash
	yarn composable_and_basilisk
	```

# Run  Kusama relay + Dali parachain + Hydra paracahin in Docker via [polkadot-launch](https://github.com/paritytech/polkadot-launch)

Build via `sandbox docker` job in Actions into latest and git hash.

```
cargo make start-devnet-docker
```

URLs:
* [the 1st Relay Chain node](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer)
* [the 1st Composable collator](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer)
* [the 1st Basilisk collator](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9998#/explorer)
