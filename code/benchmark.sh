# Build dali, picasso and composable runtimes
RUSTFLAGS='-Clink-arg=--export=__heap_base -Clink-arg=--import-memory' cargo build --release -p dali-runtime-wasm --target wasm32-unknown-unknown --features runtime-benchmarks
RUSTFLAGS='-Clink-arg=--export=__heap_base -Clink-arg=--import-memory' cargo build --release -p picasso-runtime-wasm --target wasm32-unknown-unknown --features runtime-benchmarks
RUSTFLAGS='-Clink-arg=--export=__heap_base -Clink-arg=--import-memory' cargo build --release -p composable-runtime-wasm --target wasm32-unknown-unknown --features runtime-benchmarks

# optimize the runtimes
wasm-optimizer --input target/wasm32-unknown-unknown/release/dali_runtime.wasm --output target/wasm32-unknown-unknown/release/dali_runtime_optimized.wasm
wasm-optimizer --input target/wasm32-unknown-unknown/release/picasso_runtime.wasm --output target/wasm32-unknown-unknown/release/picasso_runtime_optimized.wasm
wasm-optimizer --input target/wasm32-unknown-unknown/release/composable_runtime.wasm --output target/wasm32-unknown-unknown/release/composable_runtime_optimized.wasm

# build composable
DALI_RUNTIME=`pwd`/target/wasm32-unknown-unknown/release/dali_runtime_optimized.wasm \
PICASSO_RUNTIME=`pwd`/target/wasm32-unknown-unknown/release/picasso_runtime_optimized.wasm \
COMPOSABLE_RUNTIME=`pwd`/target/wasm32-unknown-unknown/release/composable_runtime_optimized.wasm \
cargo build --release --package composable --features=builtin-wasm,runtime-benchmarks

# run benchmarks
RUST_LOG=debug ./target/release/composable benchmark pallet \
  --chain dali-dev \
  --execution=wasm \
  --wasm-execution=compiled \
  --wasm-instantiation-strategy=legacy-instance-reuse \
  --pallet="cosmwasm" \
  --extrinsic="query_continuation" \
  --steps=1 \
  --repeat=1 
  # --output "parachain/frame/cosmwasm/src/weights.rs"
  # --output "weights.rs"


