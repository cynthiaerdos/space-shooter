use bevy::prelude::{Timer, TimerMode, Commands, Res, Sprite, Transform, Vec3, default};
use rand::RngExt;
use rand::rngs::ThreadRng;

use super::{Enemy, ProjectileCooldown};
use crate::shared::constants::{BOUNDS_X, BOUNDS_Y, ENEMY_DIMENSION, ENEMY_PROJECTILES_PER_SECOND};
use crate::shared::components::{DespawnOffscreen, RadiusCollider};
use crate::shared::resources::SpriteAssets;

pub fn spawn_enemy(
    mut commands: Commands,
    sprites: Res<SpriteAssets>
) {
    let mut rng = ThreadRng::default();
    let x = rng.random_range((-BOUNDS_X + ENEMY_DIMENSION.x)..(BOUNDS_X - ENEMY_DIMENSION.x));
    let y = BOUNDS_Y + ENEMY_DIMENSION.y;

    commands.spawn((
        Enemy,
        Sprite {
            image: sprites.enemy.clone(),
            custom_size: Some(ENEMY_DIMENSION),
            ..default()
        },
        Transform::from_translation(Vec3::new(x, y, 0.0)),
        RadiusCollider {
            radius: ENEMY_DIMENSION.x.min(ENEMY_DIMENSION.y),
        },
        ProjectileCooldown {
            timer: Timer::from_seconds(
                1.0 / (ENEMY_PROJECTILES_PER_SECOND * rng.random_range(0.5..1.5)),
                TimerMode::Repeating
            )
        },
        DespawnOffscreen
    ));
    
}