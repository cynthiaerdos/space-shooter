pub mod helpers;
pub mod systems;

use bevy::prelude::*;

use crate::states::GameState;

#[derive(Component)]
pub struct ScrollingBackground;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Playing), helpers::setup_background)
        .add_systems(
            Update,
            (systems::scroll_background)
                .run_if(in_state(GameState::Playing)),
        );
    }
}


