use bevy::prelude::{Commands, Sprite, Transform, Vec2, Vec3, default};

use super::{Projectile, EnemyProjectile, PlayerProjectile};
use crate::shared::components::{RadiusCollider, DespawnOffscreen, Velocity};
use crate::shared::constants::{PLAYER_PROJECTILE_SPEED_PER_SECOND, ENEMY_PROJECTILE_SPEED_PER_SECOND, PROJECTILE_RADIUS};
use crate::shared::resources::SpriteAssets;

pub fn spawn_player_bullet(commands: &mut Commands, sprites: &SpriteAssets, player_coordinate: Vec2) {
    commands.spawn((
        Projectile,
        PlayerProjectile,
        DespawnOffscreen,
        Velocity(Vec2::new(0.0, PLAYER_PROJECTILE_SPEED_PER_SECOND)),
        Transform::from_translation(Vec3::new(player_coordinate.x, player_coordinate.y + 40.0, 1.0)),
        Sprite {
            image: sprites.player_projectile.clone(),
            custom_size: Some(Vec2::new(40.0, 63.0)),
            ..default()
        },
        RadiusCollider {
            radius: PROJECTILE_RADIUS,
        },
    ));
}

pub fn spawn_enemy_bullet(commands: &mut Commands, sprites: &SpriteAssets, origin: Vec2) {
    commands.spawn((
        Projectile,
        EnemyProjectile,
        DespawnOffscreen,
        Velocity(Vec2::new(0.0, -ENEMY_PROJECTILE_SPEED_PER_SECOND)),
        Transform::from_translation(Vec3::new(origin.x, origin.y - 20.0, 1.0)),
        Sprite {
            image: sprites.enemy_projectile.clone(),
            custom_size: Some(Vec2::new(7.0, 21.0)),
            ..default()
        },
        RadiusCollider {
            radius: PROJECTILE_RADIUS,
        },
    ));
}