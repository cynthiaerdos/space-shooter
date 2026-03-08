use bevy::prelude::{Image, Handle, Resource};

#[derive(Resource)]
pub struct SpriteAssets {
    pub player: Handle<Image>,
    pub enemy: Handle<Image>,
    pub player_projectile: Handle<Image>,
    pub enemy_projectile: Handle<Image>,
}