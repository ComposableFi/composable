# Download a Polkadot Relay Chain
# Runs kusama or polkadot relay chains
mkdir -p ../../../polkadot/target/release
curl --location https://github.com/paritytech/polkadot/releases/download/v0.9.18/polkadot --output ../../../polkadot/target/release/polkadot
sudo chmod +x ../../../polkadot/target/release/polkadot
../../../polkadot/target/release/polkadot --version