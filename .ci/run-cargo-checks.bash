#! /bin/bash

set -e

if [ ! -d .git ]; then
    echo 'run-cargo-checks.bash must be run from repository root' 1>&2
    exit 1
fi

function run() {
    local cmd="$1"
    echo "Running '${cmd}'..."
    $cmd
}

cd ./gen
run 'cargo check'
run 'cargo test'
run 'cargo clippy -- --deny warnings'
run 'cargo fmt -- --check'
