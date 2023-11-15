#!/bin/sh
(
 cd node || exit
 clear && cargo run --bin mantis -- --centauri "http://localhost:26657" --osmosis "localhost:36657" --neutron "localhost:46657" --cvm-contract "centauri1" --wallet "mnemonic" --order-contract "centauri1"
)