#!/bin/bash

HOST="127.0.0.1"
PORT=9188

# to be nixified way to generate subxt from local node
../../../paritytech/polkadot/target/release/polkadot build-spec --chain=rococo-local --raw > rococo-local-raw.json

result/bin/polkadot --chain=./rococo-local-raw.json --tmp &
polkadot_pid=$(echo $!)

../target/release/composable --tmp --force-authoring  --chain=dali-dev  --discover-local --ws-port $PORT --  --chain=./rococo-local-raw.json &
composable_pid=$(echo $!)

# wait for TCP port opening
while ! nc -z $HOST $PORT; do
  sleep 1
  echo "Trying to connect to ${HOST}:${PORT}..."
done

# required subxt v0.25.0
subxt codegen --url "ws://${HOST}:${PORT}" > dali.rs

kill -9 $polkadot_pid $composable_pid
