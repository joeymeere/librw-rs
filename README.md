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