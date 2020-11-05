#! /bin/bash
exec cargo run --release --bin="$1" -- "${@:2}"
