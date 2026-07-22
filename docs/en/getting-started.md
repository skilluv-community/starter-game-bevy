# Getting started — starter-game-bevy

## Prerequisites

- Rust 1.80+ (stable)
- On Linux: `libasound2-dev` and `libudev-dev` for audio and gamepad support

Bevy is a big dependency graph — expect the first `cargo build` to take 3-8 minutes depending on the machine.

## Running natively

```bash
cargo run
```

The optimized `dev` profile compiles this fast enough to iterate on gameplay.

## Running in the browser (WASM)

```bash
make wasm
make serve-web
```

Then open <http://localhost:8080>. The `wasm-bindgen-cli` is installed on first use.

## Tests

```bash
cargo test
```

## Common issues

- **Sound doesn't play on Linux**: install `libasound2-dev` and reboot.
- **Wayland flakiness**: switch to X11 via `WAYLAND_DISPLAY= cargo run`.
- **WASM build fails**: ensure `wasm32-unknown-unknown` target is installed (`rustup target add wasm32-unknown-unknown`) and `wasm-bindgen-cli` matches the Bevy version.

## Bevy version

Bevy releases roughly every ~4 months and often has breaking changes. The pin is intentional. When bumping:

1. Read the Bevy migration guide.
2. Update `Cargo.toml`.
3. `cargo build` and fix the compile errors — Bevy usually gives clear compile-time hints.
