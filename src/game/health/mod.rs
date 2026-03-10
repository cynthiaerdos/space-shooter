pub mod helpers;
pub mod systems;

use bevy::prelude::*;

use crate::states::GameState;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct HealthBar {
    target: Entity,
    offset: Vec2,
    width: f32,
    height: f32,
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, 
            (systems::health_bar_follow, systems::health_bar_update)
                .run_if(in_state(GameState::Playing)),
        );
    }
}
