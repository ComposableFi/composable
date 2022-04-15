#!/bin/bash

while true; do
    sleep 1
    stdout=$(docker-compose logs --tail=1 | grep "POLKADOT LAUNCH COMPLETE")
    if [ -z "$stdout" ]; then
        true
    else
        exit 0
    fi
done
