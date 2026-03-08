pub mod systems;
pub mod helpers;

use bevy::prelude::*;
use crate::shared::helpers::{cleanup};
use crate::states::GameState;

#[derive(Component)]
pub struct DeathScreenRoot;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct MenuButton;

pub struct DeathScreenPlugin;

impl Plugin for DeathScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Dead), helpers::spawn_death_screen)
            .add_systems(
                Update,
                (systems::handle_restart_button_interaction, systems::handle_menu_button_interaction)
                    .run_if(in_state(GameState::Dead)),
            )
            .add_systems(OnExit(GameState::Dead), cleanup::<DeathScreenRoot>);
    }
}