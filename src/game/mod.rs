pub mod projectile;
pub mod collision;
pub mod enemy;
pub mod player;
pub mod background;
pub mod health;

use bevy::prelude::*;

use crate::shared::helpers::{cleanup};
use crate::states::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            projectile::ProjectilePlugin,
            health::HealthPlugin,
            collision::CollisionPlugin,
            background::BackgroundPlugin,
        ))
        .add_systems(
            OnExit(GameState::Playing),
            (
                cleanup::<player::Player>,
                cleanup::<enemy::Enemy>,
                cleanup::<projectile::Projectile>,
                cleanup::<health::HealthBar>,
                cleanup::<background::ScrollingBackground>,
            ),
        );
    }
}