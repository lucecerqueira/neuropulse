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

# Compilar a aplicação usando o Trunk
echo "A compilar a aplicação com o Trunk..."
cd crates/neuropulse-wasm
trunk build --release

# Forçar a cópia manual dos favicons para a pasta dist compilada
echo "A copiar favicons para o diretório de distribuição (dist/)..."
cp ../../favicon.ico dist/ 2>/dev/null || true
cp ../../favicon.svg dist/ 2>/dev/null || true
cp favicon.ico dist/ 2>/dev/null || true
cp favicon.svg dist/ 2>/dev/null || true

echo "=== Build concluído com sucesso! ==="
