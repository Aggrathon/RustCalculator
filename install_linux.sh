#!/bin/sh
echo "Building the calculator:"
cd `dirname "$0"`
cargo build --release
strip /target/release/calc
echo "Installing the executable to /usr/local/bin/calc:"
sudo rm -f /usr/local/bin/calc
sudo cp target/release/calc /usr/local/bin/calc
echo "Done!"