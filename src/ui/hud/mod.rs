pub mod helpers;
pub mod systems;

use bevy::prelude::*;
use crate::states::GameState;
use crate::shared::helpers::{cleanup};

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct ScoreText;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), helpers::spawn_hud)
            .add_systems(
                Update,
                (systems::update_score)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup::<HudRoot>);
    }
}