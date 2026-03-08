pub mod systems;

use bevy::prelude::*;

use crate::states::GameState;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (systems::player_projectile_hits_enemy, systems::enemy_projectile_hits_player)
                .run_if(in_state(GameState::Playing)),
        );
    }
}
