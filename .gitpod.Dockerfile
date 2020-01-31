FROM gitpod/workspace-full

USER gitpod

RUN /bin/bash -c "cargo install wasm-pack cargo-generate"