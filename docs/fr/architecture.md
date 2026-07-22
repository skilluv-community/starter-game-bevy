# Architecture — starter-game-bevy

## Choix opinionated

### 1. ECS-first

Tout est composant ou resource. `Player`, `Enemy`, `Bullet` = composants. `Score`, `EnemySpawnTimer` = resources. Systèmes = fonctions ordonnancées.

### 2. Systèmes en modules dédiés

`systems/movement.rs`, `systems/spawner.rs`, `systems/hud.rs` par responsabilité.

### 3. Change detection dans le HUD

`update_hud` skip si `Score` n'a pas changé. Pattern Bevy idiomatique.

### 4. WASM first-class

Feature `webgl2` activée, `web/index.html` host le WASM. Deploy statique.

### 5. Rectangles colorés à la place d'art

`Sprite { color, custom_size }`. Passer à `Handle<Image>` quand tu as des vraies textures.

### 6. Features Bevy minimales

On active uniquement ce qu'on utilise.

## Hors scope

- Vraies sprites + audio (CC0).
- Menu / pause / game-over (`State`s).
- Physique (`bevy_rapier2d` ou `avian`).
- Réseau / multiplayer.
