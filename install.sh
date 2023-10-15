#!/bin/bash

# Set the binary name
BINARY_NAME="whereserver"

# Check if cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Cargo (Rust's build tool) is not installed. Please install Rust and Cargo first."
    exit 1
fi

# Build the binary
echo "Building $BINARY_NAME..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

# Install the binary to /usr/local/bin/
echo "Installing $BINARY_NAME to /usr/local/bin/..."
sudo mv target/release/$BINARY_NAME /usr/local/bin/
if [ $? -ne 0 ]; then
    echo "Installation failed!"
    exit 1
fi

# Ensure it's executable
sudo chmod +x /usr/local/bin/$BINARY_NAME

echo "$BINARY_NAME installed successfully!"
