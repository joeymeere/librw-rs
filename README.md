# librw-rs

Rust bindings for [librw](https://github.com/aap/librw), a native implementation of RenderWare

## Overview 

This exposes two crates:

- `librw-sys`: C ABI over librw's C++ classes. Corresponding Rust types are opaque, besides stabel value types like vectors, matrices, tris etc...
- `librw`: Safe API. Adds proper lifetimes, ownership awareness, and wrappers to interact with stable types instead of raw FFI types from `core`

## Getting Started

```sh
git clone --recursive https://github.com/joeymeere/librw-rs
cd librw-rs
cargo build
```

Then, initialize submodules:

```sh
git submodule update --init --recursive
```

Alternatively, you can use an externally built or modified librw. Do this by overridding the vendored default:

```sh
export LIBRW_DIR=/path/to/librw
export LIBRW_LIB_DIR=/path/to/librw/build/src
export LIBRW_LIB_NAME=rw
export LIBRW_LINK_KIND=static
cargo build
```

GL3 builds require SDL2 - `librw-sys` uses `pkg-config` by default, or accepts an SDL2
install prefix:

```sh
SDL2_DIR=/path/to/SDL2-2.32.10/install cargo build --features gl3
```

`SDL2_DIR` supplies the `include` and `lib` directories. Set `SDL2_INCLUDE_DIR` or
`SDL2_LIB_DIR` when the installation uses a different layout