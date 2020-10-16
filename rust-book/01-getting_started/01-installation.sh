#!/bin/bash
echo "Rust installation command from https://doc.rust-lang.org/stable/book/ch01-01-installation.html"

curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
echo "At the end you should get a 'Rust is installed.  Great!"

echo "Adding $HOME/.cargo/bin to your path temporarily"
export PATH="$HOME/.cargo/bin:$PATH"

echo "Let's update rust to the latest versions"
echo "% rustup update"
rustup update

echo "to uninstall rust:  'rustup self uninstall'"

echo "Let us find out what version of rust is installed"
echo "% rustc --version"
rustc --version

echo "Let us open up the locally installed documentation"
echo "% rustup doc"
rustup doc
