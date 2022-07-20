	mkdir -p ../../../basilisk-node/target/release
	curl https://github.com/galacticcouncil/Basilisk-node/releases/download/v8.0.0/basilisk -Lo ../../../basilisk-node/target/release/basilisk
	chmod +x ../../../basilisk-node/target/release/basilisk
	../../../basilisk-node/target/release/basilisk --version