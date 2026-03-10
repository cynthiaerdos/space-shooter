use bevy::prelude::*;

#[derive(Resource)]
pub struct SpriteAssets {
    pub player: Handle<Image>,
    pub enemy: Handle<Image>,
    pub player_projectile: Handle<Image>,
    pub enemy_projectile: Handle<Image>,
    pub background_with_small_stars: Handle<Image>,
}

#[derive(Resource)]
pub struct FontAssets {
    pub mono: Handle<Font>,
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}