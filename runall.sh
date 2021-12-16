#!/bin/sh

echo "========== CHECK =========="
cargo check

echo "========== CLIPPY =========="
cargo clippy

echo "========== TESTS =========="
for day in day*
do
    cd "$day"
    echo $day
    cargo test
    cd ..
    echo "--------------------"
done

echo "========== EXAMPLE =========="
for day in day*
do
    cd "$day"
    echo $day
    cargo run --release -- -i input-example
    cd ..
    echo "--------------------"
done

echo "========== ACTUAL =========="
for day in day*
do
    cd "$day"
    echo $day
    cargo run --release -- -i input-actual
    cd ..
    echo "--------------------"
done
