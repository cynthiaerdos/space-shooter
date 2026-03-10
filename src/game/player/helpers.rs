use bevy::prelude::*;

use super::{Player, ProjectileCooldown};
use crate::game::health::helpers::spawn_health_bar;
use crate::game::health::{Health};
use crate::shared::components::{RadiusCollider};
use crate::shared::constants::{PLAYER_DIMENSION, PLAYER_HEALTH, PLAYER_PROJECTILES_PER_SECOND, PLAYER_Y_COORDINATE};
use crate::shared::resources::{Score, SpriteAssets};

pub fn spawn_player(
    mut commands: Commands,
    sprites: Res<SpriteAssets>
) {
    let player = commands.spawn((
        Player,
        Health { current: PLAYER_HEALTH, max: PLAYER_HEALTH },
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
    )).id();

    spawn_health_bar(&mut commands, player);
}

pub fn reset_player_score(
    mut score: ResMut<Score>,
) {
    score.value = 0;
}