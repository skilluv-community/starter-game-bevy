use bevy::prelude::*;

use crate::components::{Bullet, Enemy, Player, Score};

const BULLET_SPEED: f32 = 500.0;

pub fn player_move(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&Player, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (player, mut tf) in &mut q {
        let mut dir = Vec2::ZERO;
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            dir.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            dir.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            dir.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            dir.x += 1.0;
        }
        let dir = dir.normalize_or_zero();
        tf.translation.x += dir.x * player.speed * dt;
        tf.translation.y += dir.y * player.speed * dt;
    }
}

pub fn player_shoot(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    q: Query<&Transform, With<Player>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    let Ok(tf) = q.get_single() else {
        return;
    };
    commands.spawn((
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.2),
            custom_size: Some(Vec2::new(6.0, 12.0)),
            ..default()
        },
        Transform::from_translation(tf.translation),
        Bullet {
            velocity: Vec2::new(0.0, BULLET_SPEED),
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        },
    ));
}

pub fn bullets_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Bullet, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (entity, mut bullet, mut tf) in &mut q {
        tf.translation.x += bullet.velocity.x * dt;
        tf.translation.y += bullet.velocity.y * dt;
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn enemies_update(
    time: Res<Time>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut enemies: Query<(Entity, &Enemy, &mut Transform), Without<Bullet>>,
) {
    let dt = time.delta_secs();
    for (enemy_e, enemy, mut e_tf) in &mut enemies {
        e_tf.translation.y -= enemy.speed * dt;

        // Naive collision check
        for (b_e, b_tf) in &bullets {
            if b_tf.translation.distance(e_tf.translation) < 24.0 {
                commands.entity(enemy_e).despawn();
                commands.entity(b_e).despawn();
                score.0 += 1;
                break;
            }
        }

        if e_tf.translation.y < -400.0 {
            commands.entity(enemy_e).despawn();
        }
    }
}
