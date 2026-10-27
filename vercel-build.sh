#!/bin/bash
set -e

echo "=== A iniciar o build do NeuroPulse para Vercel ==="

# Instalar Rust caso não esteja presente
if ! command -v rustc &> /dev/null; then
    echo "A instalar o Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Adicionar alvo WebAssembly
rustup target add wasm32-unknown-unknown

# Instalar o Trunk CLI se necessário
if ! command -v trunk &> /dev/null; then
    echo "A instalar o Trunk..."
    cargo install trunk --locked
fi

# Compilar a aplicação usando o Trunk a partir da pasta do crate WASM
echo "A compilar a aplicação com o Trunk..."
cd crates/neuropulse-wasm
trunk build --release

echo "=== Build concluído com sucesso! ==="
