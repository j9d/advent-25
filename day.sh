#! /bin/bash

if [ -f "src/bin/$1.rs" ] || [ -f "input/$1.txt" ]; then
    echo "already exists"
    exit 1
fi

cp src/_template.rs "src/bin/$1.rs"
touch "input/$1.txt"
echo ".. done"
