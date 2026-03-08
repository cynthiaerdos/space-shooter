pub mod systems;
pub mod helpers;

use bevy::prelude::*;
use crate::shared::constants::{ENEMY_SPAWNS_PER_SECOND};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct ProjectileCooldown {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer {
            timer: Timer::from_seconds(1.0 / ENEMY_SPAWNS_PER_SECOND, TimerMode::Repeating),
        })
        .add_systems(Update, (systems::spawn_enemies, systems::enemy_movement, systems::enemy_shooting));
    }
}
