pub mod helpers;
pub mod systems;

use bevy::prelude::*;

use crate::states::GameState;

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
            .add_systems(OnEnter(GameState::Playing), (helpers::reset_player_score, helpers::spawn_player))
            .add_systems(
                Update,
                (systems::check_player_health, systems::player_movement, systems::player_projectile_shooting)
                            .run_if(in_state(GameState::Playing)),
            );
    }
}
