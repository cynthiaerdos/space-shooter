pub mod systems;
pub mod helpers;

use bevy::prelude::*;

use crate::states::GameState;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct PlayerProjectile;

#[derive(Component)]
pub struct EnemyProjectile;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (systems::move_projectiles, systems::despawn_offscreen_projectiles)
                .run_if(in_state(GameState::Playing)),
        );
    }
}
