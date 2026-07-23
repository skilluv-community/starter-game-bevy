use bevy::prelude::*;

mod components;
mod systems;

use components::{EnemySpawnTimer, Player, Score};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Skilluv starter — Bevy".into(),
                resolution: (960.0, 540.0).into(),
                canvas: Some("#bevy".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.008, 0.024, 0.09)))
        .insert_resource(Score::default())
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(
            0.9,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, (setup_scene, systems::hud::setup_hud))
        .add_systems(
            Update,
            (
                systems::movement::player_move,
                systems::movement::player_shoot,
                systems::movement::bullets_update,
                systems::movement::enemies_update,
                systems::spawner::spawn_enemies,
                systems::hud::update_hud,
            ),
        )
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color: Color::srgb(0.02, 0.59, 0.41),
            custom_size: Some(Vec2::new(28.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -180.0, 0.0),
        Player { speed: 260.0 },
    ));
}
