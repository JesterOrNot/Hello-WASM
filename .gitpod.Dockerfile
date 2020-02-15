FROM gitpod/workspace-full

RUN bash -cl "cargo install wasm-pack && rustup install nightly && rustup default nightly"
