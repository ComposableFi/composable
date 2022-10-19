# fastest way to build and debug runtime in simulator
RUST_BACKTRACE=full \
SKIP_WASM_BUILD=1 \
RUST_LOG=trace,bdd=trace,parity-db=warn,trie=warn,runtime=trace,substrate-relay=trace,bridge=trace,xcmp=trace,xcm=trace \
cargo +nightly test $1 --package local-integration-tests --features=local-integration-tests,${COMPOSABLE_RUNTIME:-dali} --no-default-features -- --nocapture --test-threads=1


# RUST_BACKTRACE=full SKIP_WASM_BUILD=1 RUST_LOG=trace,parity-db=warn,trie=warn,runtime=trace,substrate-relay=trace,bridge=trace,xcmp=trace,xcm=trace cargo +nightly test --package local-integration-tests --features=local-integration-tests,dali --no-default-features -- --nocapture --test-threads=1