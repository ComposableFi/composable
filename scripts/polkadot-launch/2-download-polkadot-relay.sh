	# Download a Polkadot Relay Chain
	mkdir -p ../../../polkadot/target/release
	curl https://github.com/paritytech/polkadot/releases/download/v0.9.22/polkadot -Lo ../../../polkadot/target/release/polkadot
	# TODO: really need to check hash and NIX does that
	chmod +x ../../../polkadot/target/release/polkadot
	../../../polkadot/target/release/polkadot --version