## Build from source

In this step we will setup rust compiler, toolchain and build a node. 

**Setup required libraries**

```bash
sudo apt install -y git clang curl libssl-dev llvm libudev-dev
```
**Setup Rust binary and Toolchain**

```bash
#!/bin/bash

RUST_C="nightly-2021-11-07"

curl https://sh.rustup.rs -sSf | sh -s -- -y && \
export PATH="$PATH:$HOME/.cargo/bin" && \
rustup toolchain uninstall $(rustup toolchain list) && \
rustup toolchain install $RUST_C && \
rustup target add wasm32-unknown-unknown --toolchain $RUST_C && \
rustup default $RUST_C && \
rustup show

```

```
**Get project and build node**

```bash
git clone --depth 1 --branch v2.1.3 https://github.com/ComposableFi/composable.git && \
cd composable && \
export SKIP_WASM_BUILD=1 && \
cargo build --release 
```

**One-liner**

```bash
RUST_C="nightly-2021-11-07"
RELEASE_TAG="v2.1.3"

sudo apt install -y git clang curl libssl-dev llvm libudev-dev && \
git clone --depth 1 --branch $RELEASE_TAG https://github.com/ComposableFi/composable.git && \
cd composable && \
curl https://sh.rustup.rs -sSf | sh -s -- -y && \
export PATH="$PATH:$HOME/.cargo/bin" && \
rustup toolchain uninstall $(rustup toolchain list) && \
rustup toolchain install $RUST_C && \
rustup target add wasm32-unknown-unknown --toolchain $RUST_C && \
rustup default $RUST_C && \
rustup show && \
export SKIP_WASM_BUILD=1 && \
cargo build --release
```
Compiled node should be in 
```bash
./target/release
```
