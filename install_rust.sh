#!/bin/bash

# install_rust.sh - Script to install Rust for the WFL (WebFirst Language) project
# Created: April 16, 2025

echo "Installing Rust for WFL development..."

if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
    echo "Rust is already installed."
    echo "Current Rust version: $(rustc --version)"
    echo "Current Cargo version: $(cargo --version)"
    echo "To update Rust, run: rustup update"
    exit 0
fi

if ! command -v curl &> /dev/null; then
    echo "Installing curl..."
    sudo apt-get update
    sudo apt-get install -y curl
fi

echo "Installing Rust using rustup..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

echo "Adding Rust to PATH..."
source "$HOME/.cargo/env"

echo "Verifying Rust installation..."
rustc --version
cargo --version

echo "Installing additional Rust components..."
rustup component add rustfmt
rustup component add clippy

echo "Installing dependencies for WFL project..."
sudo apt-get update
sudo apt-get install -y build-essential

echo "Rust installation complete!"
echo "You can now build the WFL project using 'cargo build'"
echo "Run tests with 'cargo test'"
echo "Run benchmarks with 'cargo bench'"
