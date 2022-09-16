# fastest way to build and debug runtime in simulator
RUST_BACKTRACE=full \
SKIP_WASM_BUILD=1 \
RUST_LOG=trace,parity-db=warn,trie=warn,runtime=trace,substrate-relay=trace,bridge=trace,xcmp=trace,xcm=trace \
cargo +nightly test sibling_trap_assets_works --package local-integration-tests --features=local-integration-tests,picasso --no-default-features -- --nocapture --test-threads=1