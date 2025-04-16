#!/bin/bash

# install_rust.sh - Script to install Rust for the WFL (WebFirst Language) project
# Created: April 16, 2025
# This script installs Rust and related components needed for WFL development

set -e  # Exit immediately if a command exits with a non-zero status

# Function to check if command exists
command_exists() {
  command -v "$1" &> /dev/null
}

echo "Installing Rust for WFL development..."

# Check if Rust is already installed
if command_exists rustc && command_exists cargo; then
    echo "Rust is already installed."
    echo "Current Rust version: $(rustc --version)"
    echo "Current Cargo version: $(cargo --version)"
    echo "To update Rust, run: rustup update"
    exit 0
fi

# Install curl if not already installed
if ! command_exists curl; then
    echo "Installing curl..."
    # Ask for confirmation before running sudo commands
    read -p "This requires sudo access to install curl. Continue? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Installation cancelled."
        exit 1
    fi
    sudo apt-get update
    sudo apt-get install -y curl
fi

# Create a temporary directory for downloads
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

# Download rustup installer to a file first instead of piping directly to sh
echo "Downloading Rust installer..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o "$TEMP_DIR/rustup-init.sh"

# Verify the download
if [ ! -s "$TEMP_DIR/rustup-init.sh" ]; then
    echo "Error: Failed to download rustup installer"
    exit 1
fi

# Make the installer executable
chmod +x "$TEMP_DIR/rustup-init.sh"

# Run the installer with -y flag for non-interactive installation
echo "Installing Rust using rustup..."
"$TEMP_DIR/rustup-init.sh" -y

# Add Rust to PATH for the current session
echo "Adding Rust to PATH..."
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
else
    echo "Warning: $HOME/.cargo/env not found. You may need to manually add Rust to your PATH."
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# Verify installation
echo "Verifying Rust installation..."
if command_exists rustc && command_exists cargo; then
    rustc --version
    cargo --version
else
    echo "Error: Rust installation failed. Please check the output above for errors."
    exit 1
fi

# Install additional Rust components
echo "Installing additional Rust components..."
rustup component add rustfmt
rustup component add clippy

# Install dependencies for WFL project
echo "Installing dependencies for WFL project..."
# Ask for confirmation before running sudo commands
read -p "This requires sudo access to install build dependencies. Continue? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    sudo apt-get update
    sudo apt-get install -y build-essential
else
    echo "Skipping installation of build dependencies."
fi

echo "Rust installation complete!"
echo "You can now build the WFL project using 'cargo build'"
echo "Run tests with 'cargo test'"
echo "Run benchmarks with 'cargo bench'"
