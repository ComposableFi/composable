#!/bin/bash

PARA_PORT=9188

# to be nixified way to generate subxt from local node
../../../paritytech/polkadot/target/release/polkadot build-spec --chain=rococo-local --raw > rococo-local-raw.json

result/bin/polkadot --chain=./rococo-local-raw.json --tmp &
polkadot_pid=$(echo $!)

../target/release/composable --tmp --force-authoring  --chain=dali-dev  --discover-local --ws-port $PARA_PORT --  --chain=./rococo-local-raw.json &
composable_pid=$(echo $!)

# wait for TCP port opening
while ! nc -z 127.0.0.1 $PARA_PORT; do
  sleep 0.5
done

# required subxt v0.25.0
subxt codegen --url "ws://127.0.0.1:${PARA_PORT}" > dali.rs

kill -9 $polkadot_pid $composable_pid