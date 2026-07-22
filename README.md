# starter-game-bevy

> A Skilluv starter — Bevy 0.15 + Rust. A tiny top-down shooter with score, WASM-exportable.

[![CI](https://github.com/skilluv-community/starter-game-bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/skilluv-community/starter-game-bevy/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Skilluv](https://img.shields.io/badge/skilluv-community-emerald)](https://skilluv.io)

## English

### What this is

A minimal Bevy 0.15 game illustrating the ECS pattern:

- Player component with WASD/arrow movement
- Bullets with velocity and lifetime timer
- Enemy spawner resource + random spawns
- Naive collision between bullets and enemies
- Score resource + UI text updated only when it changes
- WASM build with a static `index.html` wrapper

### Quickstart

```bash
git clone git@github.com:skilluv-community/starter-game-bevy.git
cd starter-game-bevy
cargo run
```

Controls: `WASD` / arrows to move, `Space` to shoot.

### WASM build

```bash
make wasm         # produces web/pkg/
make serve-web    # http://localhost:8080
```

### Structure

```
src/
  main.rs             App + plugins + scene setup
  components.rs       Player, Enemy, Bullet, Score, EnemySpawnTimer
  systems/
    movement.rs       player_move, player_shoot, bullets_update, enemies_update
    spawner.rs        spawn_enemies
    hud.rs            setup_hud + change-detected update
tests/smoke.rs        Placeholder unit test so cargo test always runs
web/index.html        WASM host page
```

### Docs

- [`docs/en/getting-started.md`](./docs/en/getting-started.md)
- [`docs/en/architecture.md`](./docs/en/architecture.md)

---

## Français

Starter Bevy 0.15 : petit shooter top-down avec score, exportable en WASM.

```bash
git clone git@github.com:skilluv-community/starter-game-bevy.git
cd starter-game-bevy
cargo run
```

Voir [`docs/fr/getting-started.md`](./docs/fr/getting-started.md).

---

## License

MIT — see [LICENSE](./LICENSE).
