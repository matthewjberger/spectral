# Rust / Wasm / Wgpu Triangle

This project is a 3D game engine written with [rust](https://www.rust-lang.org/)
using [wgpu](https://wgpu.rs/) that runs on Linux/Windows/MacOS/Wasm.

## Quickstart

```
# Native (Linux/Windows/MacOS)

cargo run -r -p app

# Wasm

# Install trunk
cargo install --locked trunk

# webgpu
trunk serve --features webgpu --open --config apps/app/Trunk.toml

# webgl
trunk serve --features webgl --open --config apps/app/Trunk.toml
```

## Prerequisites (web)

* [trunk](https://trunkrs.dev/)
