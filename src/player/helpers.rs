use bevy::prelude::{Timer, TimerMode, Res, Commands, Sprite, Transform, Vec3, default};

use super::{Player, ProjectileCooldown};
use crate::shared::components::{RadiusCollider};
use crate::shared::constants::{PLAYER_PROJECTILES_PER_SECOND, PLAYER_Y_COORDINATE, PLAYER_DIMENSION};
use crate::shared::resources::SpriteAssets;

pub fn spawn_player(
    mut commands: Commands,
    sprites: Res<SpriteAssets>
) {
    commands.spawn((
        Player,
        Sprite {
            image: sprites.player.clone(),
            custom_size: Some(PLAYER_DIMENSION),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, PLAYER_Y_COORDINATE, 0.0)),
        RadiusCollider {
            radius: PLAYER_DIMENSION.x.min(PLAYER_DIMENSION.y),
        },
        ProjectileCooldown {
            timer: Timer::from_seconds(1.0 / PLAYER_PROJECTILES_PER_SECOND, TimerMode::Once),
        },
    ));
    
}