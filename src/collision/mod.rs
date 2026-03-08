pub mod systems;

use bevy::prelude::{App,Plugin, Update};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                systems::player_projectile_hits_enemy,
                systems::enemy_projectile_hits_player
            ),
        );
    }
}
