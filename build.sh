#!/bin/bash
export OPENSSL_DIR=vendor/openssl
export OPENSSL_STATIC=1

cargo clean
cargo build --release
