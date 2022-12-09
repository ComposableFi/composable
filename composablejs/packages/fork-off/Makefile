start:
	yarn start

binary:
	./scripts/runtime-wasm.sh -b 1.10002

alice:
	./data/polkadot --chain data/rococo-local.json --alice --tmp --port 30333 --ws-port 9944

bob:
	./data/polkadot --chain data/rococo-local.json --bob --tmp --port 30334 --ws-port 9945

charlie:
	./data/polkadot --chain data/rococo-local.json --charlie --tmp --port 30335 --ws-port 9946

collator:
	RUST_LOG=runtime=debug,parachain=trace,cumulus-collator=trace,aura=trace ./data/binary --chain data/fork.json --collator --alice --force-authoring --tmp --port 40333 --ws-port 9988 -- --execution wasm --chain data/rococo-local.json --port 30343 --ws-port 9977
