# to be nixified way to generate subxt from local node
../../../paritytech/polkadot/target/release/polkadot build-spec --chain=rococo-local --raw > rococo-local-raw.json

../target/release/composable --tmp --force-authoring  --chain=dali-dev  --discover-local --  --chain=./rococo-local-raw.json &

sleep 30

# dz@dz-pc-11:~/github.com/ComposableFi/composable/scripts$ subxt codegen --url  ws://127.0.0.1:9944 > dali.rs
# The application panicked (crashed).
# Message:  Unknown prelude type 'NonZeroU64'
# Location: /home/dz/.cargo/registry/src/github.com-1ecc6299db9ec823/subxt-codegen-0.24.0/src/types/type_path.rs:126

subxt codegen --url  ws://127.0.0.1:9944 > dali.rs