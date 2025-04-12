# 🦀 LassDB

**LassDB** is a minimal, embeddable, and WebAssembly-compatible structured key-value database written in Rust.

> ⚡ Safe. 🚀 Fast. 🌍 WASM-ready. 🧠 Schema-aware. 🔍 Queryable.

---

## ✨ Features

- ✅ In-memory & file-backed key-value store (via `sled`)
- ✅ Stores structured Rust structs with **schema evolution**
- ✅ Supports versioned types (`UserV1`, `UserV2`, ...)
- ✅ Built-in **backward migration** and **runtime schema introspection**
- ✅ WASM-compatible design (via `wasm-bindgen`)
- ✅ Command-line interface for local interaction
- ✅ Basic SQL-like **Tiny Query Language (TQL)** for `SELECT` queries
- ✅ Error handling (via `anyhow`) and logging (via `tracing`)
- ✅ Built for learning, edge computing, and low-footprint environments

---

## 🛠 Getting Started

```bash
# Clone the repo
git clone https://github.com/yourname/lassdb
cd lassdb

# Run CLI
cargo run --example cli
```
## 🧪 WASM (WebAssembly) Usage

LassDB is WebAssembly-compatible and can be compiled for use in the browser or other WASM runtimes using `wasm-bindgen`.

### 🔧 Building for WASM

To compile LassDB for WebAssembly:

```bash
wasm-pack build --target web
