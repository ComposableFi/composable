#!/bin/bash
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd $script_dir
./git-repo-watcher -d /composable -i 60
