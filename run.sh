#! /bin/bash
exec cargo run --release --locked --bin="$1" -- "${@:2}"
