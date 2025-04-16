#!/bin/bash

# install_rust.sh - Script to install Rust for the WFL (WebFirst Language) project
# Created: April 16, 2025

echo "Installing Rust for WFL development..."

# Check if Rust is already installed
if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
    echo "Rust is already installed."
    echo "Current Rust version: $(rustc --version)"
    echo "Current Cargo version: $(cargo --version)"
    echo "To update Rust, run: rustup update"
    exit 0
fi

# Install curl if not already installed
if ! command -v curl &> /dev/null; then
    echo "Installing curl..."
    sudo apt-get update
    sudo apt-get install -y curl
fi

# Download rustup installer to a file first instead of piping directly to sh
echo "Downloading Rust installer..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup-init.sh

# Make the installer executable
chmod +x /tmp/rustup-init.sh

# Run the installer with -y flag for non-interactive installation
echo "Installing Rust using rustup..."
/tmp/rustup-init.sh -y

# Clean up the installer
rm /tmp/rustup-init.sh

# Add Rust to PATH
echo "Adding Rust to PATH..."
source "$HOME/.cargo/env"

# Verify installation
echo "Verifying Rust installation..."
rustc --version
cargo --version

# Install additional Rust components
echo "Installing additional Rust components..."
rustup component add rustfmt
rustup component add clippy

# Install dependencies for WFL project
echo "Installing dependencies for WFL project..."
sudo apt-get update
sudo apt-get install -y build-essential

echo "Rust installation complete!"
echo "You can now build the WFL project using 'cargo build'"
echo "Run tests with 'cargo test'"
echo "Run benchmarks with 'cargo bench'"
