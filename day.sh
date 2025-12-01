#! /bin/bash

if [ -d "src/$1" ]; then
    echo "already exists"
    exit 1
fi

mkdir "src/$1"
touch "src/$1/a.rs" "src/$1/b.rs" "src/$1/input.txt"
echo ".. done"
