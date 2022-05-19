#! /bin/bash

maybe_taplo=`whereis taplo` 
if [[ ${maybe_taplo} = "taplo: " ]]; then 
    cargo install taplo-cli 2> /dev/null;
fi
cargo +nightly-2022-04-18 fmt --all && taplo fmt