#!/usr/bin/env bash
set -e

# Garantir dependências do Rust
if ! command -v rustup &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

rustup target add wasm32-unknown-unknown

# Baixar o binário pré-compilado do Trunk
TRUNK_VERSION="v0.21.5"
if ! command -v trunk &> /dev/null; then
    echo "Baixando binário pré-compilado do Trunk ${TRUNK_VERSION}..."
    mkdir -p $HOME/.cargo/bin
    curl -sSL "https://github.com/trunk-rs/trunk/releases/download/${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz" | tar -xz -C $HOME/.cargo/bin
    chmod +x $HOME/.cargo/bin/trunk
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# Se estiver na raiz, navega para a pasta do WASM. Se já estiver nela, permanece.
if [ -d "crates/neuropulse-wasm" ]; then
    cd crates/neuropulse-wasm
fi

# Executa a compilação no diretório atual
trunk build --release
