use bevy::prelude::*;

use super::{EnemySpawnTimer, ProjectileCooldown, Enemy};
use super::helpers::{spawn_enemy};
use crate::projectile;
use crate::shared::constants::{ENEMY_SPEED, ENEMY_MAX_COUNT};
use crate::shared::resources::SpriteAssets;

pub fn spawn_enemies(
    commands: Commands,
    time: Res<Time>,
    sprites: Res<SpriteAssets>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    enemy_query: Query<&Enemy>,
) {
    spawn_timer.timer.tick(time.delta());

    if !spawn_timer.timer.just_finished() {
        return;
    }

    if enemy_query.iter().count() >= ENEMY_MAX_COUNT {
        return;
    }

    spawn_enemy(commands, sprites);
}

pub fn enemy_movement(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Enemy>>,
) {
     for mut transform in &mut query {
         transform.translation.y -= ENEMY_SPEED * time.delta_secs();
     }
}


pub fn enemy_shooting(
    mut commands: Commands,
    time: Res<Time>,
    sprites: Res<SpriteAssets>,
    mut query: Query<(&Transform, &mut ProjectileCooldown), With<Enemy>>,
) {
    for (transform, mut cooldown) in &mut query {
        cooldown.timer.tick(time.delta());

        if cooldown.timer.just_finished() {
            projectile::helpers::spawn_enemy_bullet(
                &mut commands,
                &sprites,
                transform.translation.truncate(),
            );
        }
    }
}