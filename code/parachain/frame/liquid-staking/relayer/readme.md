### polkadot relayer start CLI command
SEED="seed phrase" SLEEP_TIME_MIN=60 cargo run --features composable

### kusama relayer start CLI command
SEED="seed phrase" RELAY_HOST=wss://kusama-rpc-tn.dwellir.com:443 PARA_HOST=wss://rpc.composablenodes.tech:443 SLEEP_TIME_MIN=60 cargo run --features picasso

