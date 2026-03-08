pub mod helpers;
pub mod systems;

use bevy::prelude::*;

use crate::states::GameState;
use crate::shared::helpers::{cleanup};

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct PlayButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), helpers::spawn_main_menu)
            .add_systems(
                Update,
                (systems::handle_play_button_interaction)
                    .run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), cleanup::<MainMenuRoot>);
    }
}