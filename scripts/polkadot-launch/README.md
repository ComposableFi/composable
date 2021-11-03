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

2. build a Polkadot Relay Chain

	```bash
	old_pwd=$(pwd)
	cd ../../..
	git clone -b v0.9.12 https://github.com/paritytech/polkadot
	cd polkadot
	cargo build --release
	target/release/polkadot --version
	cd "$old_pwd"
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
	old_pwd=$(pwd)
	cd ../..
	cargo build --release
	target/release/composable --version
	cd "$old_pwd"
    ```

2. build a Polkadot Relay Chain

	```bash
	old_pwd=$(pwd)
	cd ../../..
	git clone -b v0.9.11 https://github.com/paritytech/polkadot
	cd polkadot
	cargo build --release
	target/release/polkadot --version
	cd "$old_pwd"
    ```

3. build a Basilisk's collator

	```bash
	old_pwd=$(pwd)
	cd ../../..
	git clone -b v5.0.2 https://github.com/galacticcouncil/Basilisk-node.git
	cd Basilisk-node
	cargo build --release
	target/release/basilisk --version
	cd "$old_pwd"
	```

4. build this project

	```bash
	yarn
	```

5. run all

	```bash
	yarn composable_and_basilisk
	```

URLs:
* `https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9945#/explorer` is the 1st Relay Chain node
* `https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer` is the 1st Composable's collator
* `https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9998#/explorer` is the 1st Basilisk's collator
