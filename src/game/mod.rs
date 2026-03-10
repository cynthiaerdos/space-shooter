pub mod projectile;
pub mod collision;
pub mod enemy;
pub mod player;
pub mod background;

use bevy::prelude::*;

use crate::shared::helpers::{cleanup};
use crate::states::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            background::BackgroundPlugin,
            player::PlayerPlugin,
            projectile::ProjectilePlugin,
            enemy::EnemyPlugin,
            collision::CollisionPlugin,
        ))
        .add_systems(
            OnExit(GameState::Playing),
            (
                cleanup::<player::Player>,
                cleanup::<enemy::Enemy>,
                cleanup::<projectile::Projectile>,
                cleanup::<background::ScrollingBackground>,
            ),
        );
    }
}