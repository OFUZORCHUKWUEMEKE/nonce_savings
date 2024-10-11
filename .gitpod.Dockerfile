FROM gitpod/workspace-full

# Install custom tools, runtime, etc.
RUN sudo apt-get update && \
    sudo apt-get install -y \
    rustup default 1.79.0 \
    sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)" && \
    export PATH="/home/gitpod/.local/share/solana/install/active_release/bin:$PATH" \
    cargo install --git https://github.com/coral-xyz/anchor --tag v0.30.1 avm --locked \
    avm install 0.30.1 \
    rustup default 1.79.0 \
    


