#!/bin/bash

while IFS="" read -r string; do
    echo $string
    matched_string=$(echo "$string" | grep "POLKADOT LAUNCH COMPLETE")
    if [ -z "${matched_string}" ]; then
        true
    else
        exit 0
    fi
done < <(./docker-compose logs --follow)
