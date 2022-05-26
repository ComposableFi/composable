# Run Composable's parachain only

Need to do to run 4 relay chain nodes and 1 Composable's collator:

1. build a Composable collator

	```bash
	old_pwd=$(pwd)
	git clone -b main https://github.com/ComposableFi/composable
	cd composable
	cargo build --release
	target/release/composable --version
	cd "$old_pwd"
    ```

2. build a Polkadot relay chain

	```bash
	old_pwd=$(pwd)
	git clone -b mmr-polkadot-v0.9.22 https://github.com/composableFi/polkadot
	cd polkadot
	cargo build --release
	./target/release/polkadot --version
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
* [the 1st Relay Chain node](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer)
* [the 1st Composable collator](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer)

