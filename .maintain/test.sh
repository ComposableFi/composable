#!/bin/bash

chains=(
  "./runtime/composable/src/weights,composable-dev"
  "./runtime/picasso/src/weights,picasso-dev"
  "./runtime/dali/src/weights,dali-dev"
)

for i in ${chains[@]}; do
  while IFS=',' read -r output chain; do
    echo $output $chain
  done
done