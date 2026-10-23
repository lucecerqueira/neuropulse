#!/usr/bin/env bash
set -e

# Garantir que estamos na raiz do projeto
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Instalar Rust se não estiver presente
if ! command -v rustup &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Adicionar Target WebAssembly
rustup target add wasm32-unknown-unknown

# Instalar Trunk
if ! command -v trunk &> /dev/null; then
    cargo install trunk --version 0.20.1
fi

# Compilar a aplicação WASM para Produção
cd crates/neuropulse-wasm
trunk build --release
