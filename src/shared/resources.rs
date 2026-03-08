use bevy::prelude::*;

use crate::shared::constants::PLAYER_LIVES;

#[derive(Resource)]
pub struct SpriteAssets {
    pub player: Handle<Image>,
    pub enemy: Handle<Image>,
    pub player_projectile: Handle<Image>,
    pub enemy_projectile: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource)]
pub struct Lives {
    pub value: u32,
}

impl Default for Lives {
    fn default() -> Self {
        Self { value: PLAYER_LIVES }
    }
}