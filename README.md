# Rust WebAssembly Time Display

A WebAssembly application built with Rust that displays both the current time and its corresponding Chinese Lunar Calendar date.

## Features

- Real-time display of current time
- Chinese Lunar Calendar conversion
- Chinese Zodiac year display
- Modern and responsive UI
- Automatic updates every second

## Prerequisites

- Rust and Cargo (https://rustup.rs/)
- wasm-pack (install with `cargo install wasm-pack`)
- A local web server (e.g., `python3 -m http.server`)

## Building

1. Build the WebAssembly module:
```bash
wasm-pack build --target web
```

2. Start a local web server:
```bash
python3 -m http.server 8080
```

3. Open your web browser and navigate to `http://localhost:8080`

## Project Structure

- `src/lib.rs` - Rust source code for the WebAssembly module, including lunar calendar conversion
- `index.html` - Web interface with real-time updates
- `Cargo.toml` - Rust project configuration and dependencies

## Dependencies

- `wasm-bindgen` - WebAssembly bindings
- `chrono` - Date and time handling
- `chinese-lunisolar-calendar` - Chinese lunar calendar calculations
- `serde` and `serde_json` - JSON serialization

## Display Format

The application shows:
1. Current time in ISO format
2. Chinese Lunar Calendar date
3. Chinese Zodiac year

Example output:
```
公历时间: 2024-12-12T17:34:21+08:00
农历二〇二四年十一月初一 龙年
```

## Notes

- The lunar calendar conversion is handled by the `chinese-lunisolar-calendar` crate
- Time is automatically updated every second
- The interface is fully responsive and works on both desktop and mobile browsers
