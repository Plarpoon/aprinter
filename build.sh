#!/bin/bash

# Check if cargo is installed
if ! which cargo > /dev/null 2>&1; then
    echo "Cargo is not installed. Please install it before proceeding."
    exit 1
fi

# Check if perl is installed
if ! which perl > /dev/null 2>&1; then
    echo "Perl is not installed. Please install it before proceeding."
    exit 1
fi

# Check if the FindBin module is available in Perl
if ! perl -MFindBin -e 1 > /dev/null 2>&1; then
    echo "Perl module FindBin is not available. Please install it before proceeding."
    exit 1
fi

# Check if vendor/openssl/libssl.so exists
if [ ! -f vendor/openssl/libssl.so ]; then
    echo "libssl.so is missing. Running configure and make in vendor/openssl..."
    
    # Navigate to the vendor/openssl directory
    cd vendor/openssl || exit
    
    # Run configure and make
    ./configure
    make
else
    echo "libssl.so found. No need to build."
fi

export OPENSSL_DIR=vendor/openssl
export OPENSSL_STATIC=1

cargo clean
cargo build --release
