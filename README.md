# Rust WebAssembly Time Display

A simple WebAssembly application built with Rust that displays the current time when a button is clicked.

## Prerequisites

- Rust and Cargo (https://rustup.rs/)
- wasm-pack (install with `cargo install wasm-pack`)
- A local web server (e.g., `python3 -m http.server` or `basic-http-server`)

## Building

1. Build the WebAssembly module:
```bash
wasm-pack build --target web
```

2. Start a local web server:
```bash
python3 -m http.server
```

3. Open your web browser and navigate to `http://localhost:8000`

## Features

- Button is disabled until WebAssembly is fully loaded
- Displays the current time when the button is clicked
- Modern and responsive UI

## Project Structure

- `src/lib.rs` - Rust source code for the WebAssembly module
- `index.html` - Web interface
- `Cargo.toml` - Rust project configuration and dependencies
