use bevy::prelude::*;
use rand::Rng;

use crate::components::{Enemy, EnemySpawnTimer};

pub fn spawn_enemies(
    time: Res<Time>,
    mut commands: Commands,
    mut timer: ResMut<EnemySpawnTimer>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let x = rand::thread_rng().gen_range(-400.0..400.0);
        commands.spawn((
            Sprite {
                color: Color::srgb(0.9, 0.3, 0.3),
                custom_size: Some(Vec2::splat(28.0)),
                ..default()
            },
            Transform::from_xyz(x, 320.0, 0.0),
            Enemy { speed: 90.0 },
        ));
    }
}
