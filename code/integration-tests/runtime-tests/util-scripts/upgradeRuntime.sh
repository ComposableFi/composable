#!/bin/bash
RED='\033[0;31m';
NC='\033[0m';
GREEN='\033[0;32m';
usage="$(basename "$0") [-r] [-e] [-f] -- after running chopsticks, you can use this to do a runtime upgrade on chopsticks.
where:
    -r  release version e.g: release-v9.10041.1
    -e  optional endpoint value. by default tries connecting to local port ws://127.0.0.1:8000
    -d  if release is not available on github releases, please specify the directory that holds the wasm file e.g: target/releases";

release_version="";
endpoint="";
file_path="";
while getopts ":r:e:d:" option; do
  case "$option" in
    r) release_version=$OPTARG
       ;;
    e) endpoint=$OPTARG
       ;;
    d) file_path=$OPTARG
      ;;
    :) echo "missing argument for -%r\e" "$OPTARG" >&2
       echo "$usage" >&2
       exit 1
       ;;
    *) echo "invalid command: no parameter included with argument $OPTARG"
       ;;
  esac
done
if [ -z "$release_version" ] && [ -z "$file_path" ]; then
  echo -e "${RED} please specify the release version with -r or specify a file path with -f ${NC}"
  echo "$usage"
  exit 1;
fi
if [ -n "$release_version" ]; then
  echo -e "${GREEN}downloading wasm from releases page ${NC}";
  rm -rf ../tests/utils/downloads/*;
  major_release=$(echo "$release_version" | awk -F'.' '{print $2}');
  (
  cd ../tests/utils/downloads;
  curl -OL "https://github.com/ComposableFi/composable/releases/download/$release_version/picasso_runtime_v$major_release.wasm";
  )
  ts-node ../tests/utils/runtimeUpgrade.ts "../tests/utils/downloads" $endpoint;
elif [ -n "$file_path" ]; then
  echo -e "${GREEN} doing runtime upgrade";
  ts-node ../tests/utils/runtimeUpgrade.ts $file_path $endpoint;
fi
