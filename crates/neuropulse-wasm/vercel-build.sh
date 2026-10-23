#!/usr/bin/env bash
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Instalar Rust caso não esteja no PATH
if ! command -v rustup &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Garantir o target WebAssembly
rustup target add wasm32-unknown-unknown

# Baixar o binário pré-compilado do Trunk (evita compilar lightningcss)
TRUNK_VERSION="v0.21.5"
if ! command -v trunk &> /dev/null; then
    echo "Baixando binário pré-compilado do Trunk ${TRUNK_VERSION}..."
    mkdir -p $HOME/.cargo/bin
    curl -sSL "https://github.com/trunk-rs/trunk/releases/download/${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz" | tar -xz -C $HOME/.cargo/bin
    chmod +x $HOME/.cargo/bin/trunk
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# Compilar a aplicação WASM
cd crates/neuropulse-wasm
trunk build --release
