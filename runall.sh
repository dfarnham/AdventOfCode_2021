#!/bin/sh

inputs="input-example input-actual"

echo "========== CHECK =========="
cargo check

echo "========== CLIPPY =========="
cargo clippy

echo "========== TESTS =========="
for day in day*
do
    echo cargo test --bin "$day" --release
    cargo test --bin "$day" --release
    echo "--------------------"
done

for input in $inputs
do
    echo "========== INPUT FILES =========="
    for day in day*
    do
        if [ -f "$day/$input" ]; then
            echo cargo run --bin "$day" --release -- -i "$day/$input"
            cargo run --bin "$day" --release -- -i "$day/$input"
            echo "--------------------"
        fi
    done
done
