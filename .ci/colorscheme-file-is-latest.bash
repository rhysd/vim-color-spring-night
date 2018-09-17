#!/bin/bash

set -e

(cd gen && cargo run -- --dir ..)
diff="$(git diff colors/spring-night.vim)"

if [[ "$diff" != "" ]]; then
    echo "ERROR: colors/spring-night.vim is not up-to-date!!"
    echo ""
    echo "$diff"
    exit 1
fi
