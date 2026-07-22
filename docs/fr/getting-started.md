# Démarrage — starter-game-bevy

## Prérequis

- Rust 1.80+ stable
- Sur Linux : `libasound2-dev` et `libudev-dev`

Bevy = gros graphe de deps ; premier `cargo build` en 3-8 min.

## Run natif

```bash
cargo run
```

## Run dans le navigateur (WASM)

```bash
make wasm
make serve-web
```

<http://localhost:8080>.

## Tests

```bash
cargo test
```

## Problèmes courants

- **Son coupé Linux** : installer `libasound2-dev`.
- **Wayland instable** : `WAYLAND_DISPLAY= cargo run`.
- **WASM build fail** : `rustup target add wasm32-unknown-unknown`.

## Version Bevy

Bevy release ~tous les 4 mois avec breaking changes. Pin volontaire. Bumper = lire le migration guide + patcher Cargo.toml + fixer les erreurs de compile.
