# ensure basilisk collator
mkdir -p ../../../Basilisk-node/target/release
curl https://github.com/galacticcouncil/Basilisk-node/releases/download/v7.0.0/basilisk -Lo ../../../Basilisk-node/target/release/basilisk
chmod +x ../../../Basilisk-node/target/release/basilisk
../../../Basilisk-node/target/release/basilisk --version