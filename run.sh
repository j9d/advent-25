#! /bin/bash

if [ -z "$1" ]; then
    echo "not enough args"
    exit 1
fi

if [ ! -f "src/bin/$1.rs" ]; then
    echo "day/part doesn't exist"
    exit 1
fi

# build and run
cargo build --bin "$1" && cargo run --bin "$1"
