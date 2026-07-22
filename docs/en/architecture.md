# Architecture — starter-game-bevy

## Opinionated choices

### 1. ECS-first

Everything is a component or a resource. `Player`, `Enemy`, `Bullet` are components. `Score` and `EnemySpawnTimer` are resources. Systems are plain functions the scheduler runs.

### 2. Systems in dedicated modules

`systems/movement.rs`, `systems/spawner.rs`, `systems/hud.rs` — grouped by responsibility, easy to find and test individually.

### 3. `res-change-detection` in HUD

`systems::hud::update_hud` skips work if `Score` didn't change. This is the idiomatic Bevy pattern to avoid rewriting text every frame.

### 4. WASM as a first-class target

The `webgl2` feature is enabled and a static `web/index.html` hosts the compiled WASM. Deploy the `web/` folder to any static host — no server needed.

### 5. Coloured rectangles instead of art

The starter uses `Sprite` with `color` and `custom_size`. Swap in `Handle<Image>` when you have real art.

### 6. Minimal Bevy features

We only enable the features we actually use, keeping the build small.

## What's out of scope

- Actual sprite atlases and audio (bring your own CC0 assets).
- Menu / pause / game-over screens (add `State`s when needed).
- Physics (add `bevy_rapier2d` or `avian` for real collisions).
- Networking / multiplayer.
