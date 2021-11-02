#!/usr/sh
echo "Composable Multiplatform installer"
myos=$(uname -a)
echo $myos

# Ubuntu:
if echo "$myos" | fgrep -q Ubuntu 2>/dev/null ; then      
	apt update && apt install -y git clang curl libssl-dev llvm libudev-dev
# Debian:
elif echo "$myos" | fgrep -q Debian 2>/dev/null; then
	apt update && apt install -y git clang curl libssl-dev llvm libudev-dev
# Fedora
elif echo "$myos" | fgrep -q Fedora 2>/dev/null; then
	dnf update && dnf install clang curl git openssl-devel
# Arch
elif echo "$myos" | fgrep -q Arch 2>/dev/null; then
	pacman -Syu --needed --noconfirm curl git clang

# OpenBSD
#$ pkg_info -Q rust 
#rust-1.51.0
#rust-clippy-1.51.0
#rust-gdb-1.51.0
#rust-rustfmt-1.51.0

elif echo "$myos" | fgrep -q OpenBSD 2>/dev/null; then
	pkg_add -uv && pkg_add -iv rust rust-gdb rust-clippy rust-rustfmt 

# FreeBSD
elif echo "$myos" | fgrep -q FreeBSD 2>/dev/null; then
	curl https://sh.rustup.rs -sSf | sh

# Darwin
elif echo "$myos" | fgrep -q Darwin 2>/dev/null; then
	if ! which brew >/dev/null 2>&1; then
		/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"
	fi

	brew update && brew install openssl cmake llvm

else
	echo "Invalid operating system detected, do you want to proceed with the installation? yes/no"
	read response
	if "no" == $response; then
		echo "Goodbye"
		exit 1

fi

echo "Getting the latest version of Rust"
# Check if rust is enabled
if ! which rustup >/dev/null 2>&1; then
	echo "Installing Rust"
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	source ~/.cargo/env
else:
	rustup update

rustup default stable
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
echo "Installing composable node"
git clone https://github.com/composablefi/composable
cd composable/ && sh scripts/init.sh && cargo build --release

echo "Done"
