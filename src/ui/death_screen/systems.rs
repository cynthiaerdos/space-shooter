use super::{RestartButton, MenuButton};
use crate::{ states::GameState};

use bevy::prelude::*;

pub fn handle_restart_button_interaction(
    restart_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &restart_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

pub fn handle_menu_button_interaction(
    menu_query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for interaction in &menu_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Menu);
        }
    }
}
