#!/bin/sh
# One liner a node operator can use to install and compile the node
echo "Composable Multiplatform installer"
my_os=$(uname -a)

# Ubuntu:
if echo "$my_os" | fgrep -q Ubuntu 2>/dev/null ; then
	apt update && apt install -y git clang curl libssl-dev llvm libudev-dev
# Debian:
elif echo "$my_os" | fgrep -q Debian 2>/dev/null; then
	echo "Debian Detected"
	apt update && apt install -y git clang curl libssl-dev llvm libudev-dev
# Fedora
elif echo "$my_os" | fgrep -q fc32 2>/dev/null; then
	echo "Fedora detected"
	dnf update && dnf install clang curl git openssl-devel
# Arch
elif echo "$my_os" | fgrep -q Arch 2>/dev/null; then
	pacman -Syu --needed --noconfirm curl git clang

# Nix Os
elif echo "$my_os" | fgrep -q NixOS 2>/dev/null; then
	nix-env -iA nixpkgs.{git,rustup}

# OpenBSD
elif echo "$my_os" | fgrep -q OpenBSD 2>/dev/null; then
	pkg_add -uv && pkg_add -iv rust rust-gdb rust-clippy rust-rustfmt

# FreeBSD
elif echo "$my_os" | fgrep -q FreeBSD 2>/dev/null; then
	curl https://sh.rustup.rs -sSf | sh

# Darwin
elif echo "$my_os" | fgrep -q Darwin 2>/dev/null; then
	if ! which brew >/dev/null 2>&1; then
		/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"
	fi

	brew update && brew install openssl cmake llvm

else
	echo "Not supported operating system detected, do you want to proceed with the installation? yes/no"
	read response
	if "no" == $response; then
		echo "Goodbye"
		exit 1
	fi
fi

# NOTE: need to keep Rust version compatible with other ways we run node
echo "Getting the latest version of Rust"
# Check if rust is enabled
if ! which rustup >/dev/null 2>&1; then
	echo "Installing Rust"
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	source ~/.cargo/env
else:
	rustup update
fi # new

rustup default stable
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
echo "Installing composable node"
git clone https://github.com/composablefi/composable && cd composable/ && sh scripts/init.sh && cargo build --release

echo "Done"
