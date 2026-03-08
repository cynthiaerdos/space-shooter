pub mod helpers;
pub mod systems;

use bevy::prelude::{Component, Timer, App, Startup, Update, Plugin};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ProjectileCooldown {
    pub timer: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, helpers::spawn_player)
            .add_systems(Update, (systems::player_movement, systems::player_projectile_shooting));
    }
}
