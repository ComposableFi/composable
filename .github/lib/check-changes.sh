#!/bin/bash

# Creates a list of files that were changed in HEAD since BASE
# git diff --name-only ${BASE} ${HEAD} | grep ^${PREFIX}/.*$
# Filters out files that are outside the path we're interested in

CHANGED="false"
CHANGED_PATHS=$(git diff --name-only ${BASE} ${HEAD} | grep ^${PREFIX}/.*$ | cut -d\/ -f${NESTING_LEVEL} | uniq | awk '{ print "{\"target\": \""$0"\"}" }' | paste -sd "," -)
if [ -n "${CHANGED_PATHS}" ];then
    CHANGED="true"
fi
echo "::set-output name=changed::${CHANGED}"
echo "::set-output name=paths::{\"include\": [${CHANGED_PATHS}]}"
exit 0
